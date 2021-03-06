use crate::board_representation::game_state::{GameMove, GameMoveType, PieceType};
use crate::move_generation::movegen;
use crate::search::moveordering::MoveOrderingStage::{
    BadCapture, GoodCapture, GoodCaptureInitialization, Killer, PVMove, Quiet, QuietInitialization,
    TTMove,
};
use crate::search::quiescence::{see, PIECE_VALUES};
use crate::search::searcher::Thread;
use crate::search::{CombinedSearchParameters, GradedMove};

//For MVV-LVA calculation
pub const ATTACKER_VALUE: [i16; 6] = [0, 1, 2, 3, 4, 5];
pub const TARGET_VALUE: [i16; 5] = [100, 400, 400, 650, 1100];

pub fn mvvlva(mv: &GameMove) -> i16 {
    debug_assert!(mv.is_capture());
    TARGET_VALUE[mv.get_captured_piece().to_index()] - ATTACKER_VALUE[mv.piece_type.to_index()]
}

pub const NORMAL_STAGES: [MoveOrderingStage; 8] = [
    PVMove,
    TTMove,
    GoodCaptureInitialization,
    GoodCapture,
    Killer,
    QuietInitialization,
    Quiet,
    BadCapture,
];
pub const QUIESCENCE_STAGES: [MoveOrderingStage; 3] =
    [TTMove, GoodCaptureInitialization, GoodCapture];
pub const QUIESCENCE_IN_CHECK_STAGES: [MoveOrderingStage; 6] = [
    TTMove,
    GoodCaptureInitialization,
    GoodCapture,
    QuietInitialization,
    Quiet,
    BadCapture,
];
pub enum MoveOrderingStage {
    PVMove,
    TTMove,
    GoodCaptureInitialization,
    GoodCapture,
    Killer,
    QuietInitialization,
    Quiet,
    BadCapture,
}
pub struct MoveOrderer {
    pub stage: usize,
    pub stages: &'static [MoveOrderingStage],
    pub gen_only_captures: bool,
    pub has_legal_move: bool,
}
impl MoveOrderer {
    pub fn next(
        &mut self,
        thread: &mut Thread,
        p: &CombinedSearchParameters,
        pv_table_move: &Option<GameMove>,
        tt_move: &Option<GameMove>,
    ) -> Option<(GameMove, f64)> {
        if self.stage >= self.stages.len() {
            return None;
        }
        match self.stages[self.stage] {
            MoveOrderingStage::PVMove => {
                self.stage += 1;
                if pv_table_move.is_some() {
                    return Some((pv_table_move.unwrap(), 0.));
                } else {
                    return self.next(thread, p, pv_table_move, tt_move);
                }
            }
            MoveOrderingStage::TTMove => {
                self.stage += 1;
                if tt_move.is_some() && tt_move != pv_table_move {
                    return Some((tt_move.unwrap(), 0.));
                } else {
                    return self.next(thread, p, pv_table_move, tt_move);
                }
            }
            MoveOrderingStage::GoodCaptureInitialization => {
                //Generate moves first!
                let agsi = movegen::generate_moves(
                    &p.game_state,
                    self.gen_only_captures,
                    &mut thread.movelist.move_lists[p.current_depth],
                    &thread.attack_container.attack_containers[p.current_depth],
                );
                self.has_legal_move = agsi.stm_haslegalmove;
                let our_mvlist = &mut thread.movelist.move_lists[p.current_depth];

                if pv_table_move.is_some() {
                    let mv_index = our_mvlist.find_move(pv_table_move.as_ref().unwrap(), false);
                    if mv_index < our_mvlist.move_list.len() {
                        our_mvlist.move_list.remove(mv_index);
                    }
                }
                if tt_move.is_some() {
                    let mv_index = our_mvlist.find_move(tt_move.as_ref().unwrap(), false);
                    if mv_index < our_mvlist.move_list.len() {
                        our_mvlist.move_list.remove(mv_index);
                    }
                }

                //Give any capture move in movelist its MVV-LVA score
                for mv in our_mvlist.move_list.iter_mut() {
                    if mv.0.is_capture() {
                        mv.1 = Some(f64::from(mvvlva(&mv.0)));
                    }
                }

                self.stage += 1;
                return self.next(thread, p, &None, &None);
            }
            MoveOrderingStage::GoodCapture => {
                //We now have all of the captures sorted by mvv lva
                let our_list = &mut thread.movelist.move_lists[p.current_depth];
                let highest_mvv_lva = our_list.highest_score();
                if highest_mvv_lva.is_none() || (highest_mvv_lva.unwrap().1).1.unwrap() < 0. {
                    self.stage += 1;
                    return self.next(thread, p, pv_table_move, tt_move);
                } else {
                    let (gm_index, graded_move) = highest_mvv_lva.unwrap();
                    our_list.move_list.remove(gm_index);
                    if PIECE_VALUES[graded_move.0.get_captured_piece().to_index()]
                        - PIECE_VALUES[graded_move.0.piece_type.to_index()]
                        >= 0
                        || graded_move.0.piece_type == PieceType::King
                    {
                        return Some((graded_move.0, 0.));
                    } else {
                        let see_value = see(
                            p.game_state,
                            &graded_move.0,
                            self.stages.len() == NORMAL_STAGES.len(),
                            &mut thread.see_buffer,
                        );
                        if see_value >= 0 {
                            return Some((graded_move.0, 0.));
                        } else {
                            our_list
                                .move_list
                                .push(GradedMove(graded_move.0, Some(f64::from(see_value))));
                            return self.next(thread, p, &None, &None);
                        }
                    }
                }
            }
            MoveOrderingStage::Killer => {
                debug_assert!(
                    thread.killer_moves[p.current_depth][0].is_none()
                        || !thread.killer_moves[p.current_depth][0]
                            .unwrap()
                            .is_capture()
                );
                debug_assert!(
                    thread.killer_moves[p.current_depth][1].is_none()
                        || !thread.killer_moves[p.current_depth][1]
                            .unwrap()
                            .is_capture()
                );
                let our_list = &mut thread.movelist.move_lists[p.current_depth];
                let mut found_index = our_list.move_list.len();
                for (index, gmv) in our_list.move_list.iter().enumerate() {
                    if gmv.1.is_none()
                        && (Some(gmv.0) == thread.killer_moves[p.current_depth][0]
                            || Some(gmv.0) == thread.killer_moves[p.current_depth][1])
                    {
                        found_index = index;
                        break;
                    }
                }
                if found_index < our_list.move_list.len() {
                    let res = our_list.move_list[found_index].0;
                    our_list.move_list.remove(found_index);
                    return Some((res, 0.));
                } else {
                    self.stage += 1;
                    return self.next(thread, p, &None, &None);
                }
            }
            MoveOrderingStage::QuietInitialization => {
                for mv in thread.movelist.move_lists[p.current_depth]
                    .move_list
                    .iter_mut()
                {
                    if mv.1.is_none() {
                        debug_assert!(!mv.0.is_capture());
                        mv.1 = Some(
                            thread.hh_score[p.game_state.color_to_move][mv.0.from as usize]
                                [mv.0.to as usize] as f64
                                / thread.bf_score[p.game_state.color_to_move][mv.0.from as usize]
                                    [mv.0.to as usize] as f64
                                / 1000.0,
                        );
                    }
                }
                self.stage += 1;
                return self.next(thread, p, &None, &None);
            }
            MoveOrderingStage::Quiet => {
                let our_list = &mut thread.movelist.move_lists[p.current_depth];
                let highest = our_list.highest_score();
                if highest.is_none() {
                    self.stage = self.stages.len();
                    return None;
                } else {
                    let (index, gmv) = highest.unwrap();
                    if gmv.1.unwrap() < 0. {
                        self.stage += 1;
                        return self.next(thread, p, &None, &None);
                    }
                    debug_assert!(!gmv.0.is_capture());
                    our_list.move_list.remove(index);
                    return Some((gmv.0, 0.));
                }
            }
            MoveOrderingStage::BadCapture => {
                let our_list = &mut thread.movelist.move_lists[p.current_depth];
                let highest = our_list.highest_score();
                if highest.is_none() {
                    self.stage = self.stages.len();
                    return None;
                } else {
                    let (index, gmv) = highest.unwrap();
                    debug_assert!(gmv.0.is_capture());
                    debug_assert!(gmv.1.unwrap() < 0.);
                    our_list.move_list.remove(index);
                    return Some((gmv.0, gmv.1.unwrap()));
                }
            }
        }
    }
}
