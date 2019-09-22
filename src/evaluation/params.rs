pub const TEMPO_BONUS_MG: i16 = 10;
pub const TEMPO_BONUS_EG: i16 = 15;
pub const SHIELDING_PAWN_MISSING_MG: [i16; 4] = [0, -38, -61, -83];
pub const SHIELDING_PAWN_MISSING_EG: [i16; 4] = [-7, 4, 2, 1];
pub const SHIELDING_PAWN_MISSING_ON_OPEN_FILE_MG: [i16; 4] = [-41, -49, -104, -169];
pub const SHIELDING_PAWN_MISSING_ON_OPEN_FILE_EG: [i16; 4] = [-10, -4, 10, 2];
pub const PAWN_DOUBLED_VALUE_MG: i16 = 4;
pub const PAWN_DOUBLED_VALUE_EG: i16 = -18;
pub const PAWN_ISOLATED_VALUE_MG: i16 = -16;
pub const PAWN_ISOLATED_VALUE_EG: i16 = -15;
pub const PAWN_BACKWARD_VALUE_MG: i16 = -14;
pub const PAWN_BACKWARD_VALUE_EG: i16 = -17;
pub const PAWN_SUPPORTED_VALUE_MG: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 8, 22, 10, 20, -5, 36, 18],
    [3, 10, 11, 13, 13, 4, 9, 12],
    [-7, 2, 12, 5, 27, 30, -5, -17],
    [-14, -13, 51, 72, 75, 62, 15, -15],
    [15, 23, 24, 28, 28, 23, 20, 15],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const PAWN_SUPPORTED_VALUE_EG: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [11, 21, 25, 28, 40, 22, 16, 19],
    [-2, 9, 8, 30, 17, 3, 1, -7],
    [-5, 6, 28, 36, 36, 10, 9, -3],
    [13, 28, 50, 95, 100, 67, 29, 32],
    [16, 22, 26, 34, 38, 30, 19, 12],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const PAWN_ATTACK_CENTER_MG: i16 = -19;
pub const PAWN_ATTACK_CENTER_EG: i16 = -16;
pub const PAWN_MOBILITY_MG: i16 = 7;
pub const PAWN_MOBILITY_EG: i16 = 16;
pub const PAWN_PASSED_VALUES_MG: [i16; 7] = [0, -8, -21, -7, 27, 43, 81];
pub const PAWN_PASSED_VALUES_EG: [i16; 7] = [0, 1, 18, 51, 91, 136, 231];
pub const PAWN_PASSED_NOT_BLOCKED_VALUES_MG: [i16; 7] = [0, 10, -3, -23, -37, -8, 84];
pub const PAWN_PASSED_NOT_BLOCKED_VALUES_EG: [i16; 7] = [0, 24, 15, 53, 118, 277, 345];
pub const ROOK_BEHIND_SUPPORT_PASSER_MG: i16 = 12;
pub const ROOK_BEHIND_SUPPORT_PASSER_EG: i16 = 3;
pub const ROOK_BEHIND_ENEMY_PASSER_MG: i16 = 12;
pub const ROOK_BEHIND_ENEMY_PASSER_EG: i16 = -128;
pub const PAWN_PASSED_WEAK_MG: i16 = -9;
pub const PAWN_PASSED_WEAK_EG: i16 = -15;
pub const KNIGHT_SUPPORTED_BY_PAWN_MG: i16 = 2;
pub const KNIGHT_SUPPORTED_BY_PAWN_EG: i16 = 7;
pub const KNIGHT_OUTPOST_MG_TABLE: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [-7, 2, 18, -3, 9, 3, 0, 8],
    [32, 33, 48, 44, 58, 56, 21, 42],
    [18, 49, 60, 66, 75, 72, 78, 11],
    [11, 4, 41, 33, 35, 59, 25, 19],
    [15, 19, 18, 25, 36, 20, 17, 16],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const KNIGHT_OUTPOST_EG_TABLE: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [8, 8, 17, 8, 13, 3, 3, -2],
    [9, 23, 36, 40, 37, 35, 27, 18],
    [18, 28, 37, 49, 56, 52, 39, 17],
    [10, 8, 32, 59, 49, 43, 23, 11],
    [15, 22, 20, 25, 29, 18, 24, 15],
    [0, 1, 0, 0, 1, 0, 1, 0],
];
pub const ROOK_ON_OPEN_FILE_BONUS_MG: i16 = 43;
pub const ROOK_ON_OPEN_FILE_BONUS_EG: i16 = 28;
pub const ROOK_ON_SEVENTH_MG: i16 = 20;
pub const ROOK_ON_SEVENTH_EG: i16 = 52;
pub const PAWN_PIECE_VALUE_MG: i16 = 106;
pub const PAWN_PIECE_VALUE_EG: i16 = 187;
pub const KNIGHT_PIECE_VALUE_MG: i16 = 435;
pub const KNIGHT_PIECE_VALUE_EG: i16 = 731;
pub const KNIGHT_VALUE_WITH_PAWNS: [i16; 17] = [
    -41, -119, -47, -37, -30, -21, -4, 1, 18, 23, 32, 38, 46, 52, 51, 60, 57,
];
pub const BISHOP_PIECE_VALUE_MG: i16 = 487;
pub const BISHOP_PIECE_VALUE_EG: i16 = 714;
pub const BISHOP_PAIR_BONUS_MG: i16 = 34;
pub const BISHOP_PAIR_BONUS_EG: i16 = 112;
pub const ROOK_PIECE_VALUE_MG: i16 = 665;
pub const ROOK_PIECE_VALUE_EG: i16 = 1283;
pub const QUEEN_PIECE_VALUE_MG: i16 = 1544;
pub const QUEEN_PIECE_VALUE_EG: i16 = 2389;
pub const DIAGONALLY_ADJACENT_SQUARES_WITH_OWN_PAWNS_MG: [i16; 5] = [-18, -17, -26, -2, -100];
pub const DIAGONALLY_ADJACENT_SQUARES_WITH_OWN_PAWNS_EG: [i16; 5] = [58, 61, 46, 19, -100];
pub const KNIGHT_MOBILITY_BONUS_MG: [i16; 9] = [-45, -19, -9, -3, 0, 0, -1, -3, -4];
pub const KNIGHT_MOBILITY_BONUS_EG: [i16; 9] = [-69, 3, 9, 14, 25, 40, 42, 46, 42];
pub const BISHOP_MOBILITY_BONUS_MG: [i16; 14] =
    [-10, 5, 14, 22, 29, 38, 44, 48, 46, 45, 47, 48, 66, 93];
pub const BISHOP_MOBILITY_BONUS_EG: [i16; 14] =
    [-30, -22, -9, 3, 30, 51, 57, 65, 73, 71, 64, 64, 50, 49];
pub const ROOK_MOBILITY_BONUS_MG: [i16; 15] =
    [-24, -8, -3, 5, 1, 7, 16, 23, 21, 21, 22, 19, 18, 14, 17];
pub const ROOK_MOBILITY_BONUS_EG: [i16; 15] =
    [-26, 8, 15, 18, 29, 38, 36, 41, 54, 62, 68, 73, 76, 79, 79];
pub const QUEEN_MOBILITY_BONUS_MG: [i16; 28] = [
    -19, 6, 0, 10, 13, 18, 23, 21, 21, 23, 23, 18, 18, 16, 14, 11, 10, 6, 8, 6, 17, 17, 22, 41, 49,
    59, 56, 55,
];
pub const QUEEN_MOBILITY_BONUS_EG: [i16; 28] = [
    -40, -37, -35, -39, -43, -46, -38, -1, 13, 19, 34, 61, 61, 78, 90, 101, 110, 117, 125, 121,
    125, 133, 122, 116, 88, 98, 89, 88,
];
pub const ATTACK_WEIGHT_MG: [i16; 8] = [0, 41, 112, 138, 208, 114, 100, 100];
pub const SAFETY_TABLE_MG: [i16; 100] = [
    0, 0, 1, 2, 3, 11, 60, 70, 12, 15, 20, 27, 57, 62, 79, 41, 46, 53, 61, 69, 89, 103, 84, 87, 91,
    100, 116, 126, 137, 133, 141, 151, 171, 184, 202, 208, 213, 225, 237, 248, 261, 277, 280, 295,
    307, 319, 331, 342, 354, 367, 377, 389, 401, 412, 423, 436, 449, 459, 471, 483, 494, 501, 500,
    501, 500, 500, 500, 500, 499, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];
pub const ATTACK_WEIGHT_EG: [i16; 8] = [-1, 68, 76, 116, 196, 110, 100, 100];
pub const SAFETY_TABLE_EG: [i16; 100] = [
    -1, 7, 5, 2, 9, 7, 17, 9, 20, 19, 21, 24, 30, 29, 36, 38, 46, 51, 57, 65, 69, 76, 82, 86, 89,
    98, 105, 113, 121, 133, 141, 150, 169, 182, 192, 201, 213, 226, 237, 249, 259, 273, 284, 296,
    307, 322, 330, 342, 354, 366, 378, 389, 401, 412, 424, 436, 448, 459, 471, 483, 494, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];
pub const KNIGHT_ATTACK_WORTH_MG: i16 = 6;
pub const KNIGHT_ATTACK_WORTH_EG: i16 = 1;
pub const BISHOP_ATTACK_WORTH_MG: i16 = 7;
pub const BISHOP_ATTACK_WORTH_EG: i16 = 0;
pub const ROOK_ATTACK_WORTH_MG: i16 = 7;
pub const ROOK_ATTACK_WORTH_EG: i16 = 2;
pub const QUEEN_ATTACK_WORTH_MG: i16 = 7;
pub const QUEEN_ATTACK_WORTH_EG: i16 = 4;
pub const KNIGHT_SAFE_CHECK_MG: i16 = 14;
pub const KNIGHT_SAFE_CHECK_EG: i16 = 7;
pub const BISHOP_SAFE_CHECK_MG: i16 = 6;
pub const BISHOP_SAFE_CHECK_EG: i16 = 13;
pub const ROOK_SAFE_CHECK_MG: i16 = 7;
pub const ROOK_SAFE_CHECK_EG: i16 = 13;
pub const QUEEN_SAFE_CHECK_MG: i16 = 6;
pub const QUEEN_SAFE_CHECK_EG: i16 = 29;
pub const PSQT_PAWN_MG: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [-41, -42, -35, -45, -26, -24, 4, -34],
    [-35, -40, -10, -2, 1, -4, -23, -26],
    [-32, -29, 10, 28, 33, 27, -4, -15],
    [-24, -10, -4, 9, 28, 16, 15, -1],
    [-6, 8, 37, 32, 60, 112, 56, 21],
    [5, 1, 28, 33, 37, 8, -20, -66],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const PSQT_PAWN_EG: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [-20, -32, -31, -38, -16, -19, -34, -35],
    [-7, -19, -16, -15, -12, 0, -17, -16],
    [9, -10, -7, -20, -9, -1, -7, -7],
    [38, 7, -21, -43, -40, -18, 2, 15],
    [61, 47, 3, -51, -61, -5, 41, 40],
    [93, 59, 46, -12, -17, 6, 23, 32],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const PSQT_KNIGHT_MG: [[i16; 8]; 8] = [
    [-68, -42, -45, -27, -26, -17, -40, -66],
    [-57, -44, -23, 0, -4, -3, -15, -20],
    [-50, -10, 8, 13, 24, 17, 21, -27],
    [-25, 0, 23, 13, 29, 26, 13, -17],
    [-10, -1, 30, 58, 20, 56, -2, 8],
    [-22, 29, 38, 45, 106, 83, 47, -21],
    [-65, -28, 21, 34, -13, 89, -24, -48],
    [-178, -48, -39, -42, -31, -53, -40, -143],
];
pub const PSQT_KNIGHT_EG: [[i16; 8]; 8] = [
    [-58, -50, -24, -19, -19, -32, -43, -57],
    [-31, -29, -7, -6, -8, -11, -12, -10],
    [-17, 8, 12, 37, 27, 21, 3, -11],
    [-12, 12, 32, 42, 44, 24, 9, -11],
    [-1, 9, 30, 37, 29, 32, 3, -19],
    [-23, 3, 38, 31, 13, 15, -3, -29],
    [-49, -10, -3, 0, -30, -8, -15, -59],
    [-155, -53, -40, -35, -27, -53, -46, -143],
];
pub const PSQT_BISHOP_MG: [[i16; 8]; 8] = [
    [1, 20, -4, -5, -5, -20, 7, 1],
    [0, 12, 18, -5, 7, 16, 28, 11],
    [1, 18, 13, 7, 6, 22, 24, 12],
    [-6, -15, -9, 18, 15, 5, -1, 6],
    [-25, -6, 7, 17, 28, 8, -1, -44],
    [-14, 6, 16, 37, 10, 46, 20, 18],
    [-47, -18, -17, -7, -12, -17, -32, -43],
    [-55, -26, -18, -51, -47, -28, -19, -72],
];
pub const PSQT_BISHOP_EG: [[i16; 8]; 8] = [
    [-34, -35, -29, -19, -6, -9, -28, -32],
    [-26, -19, -27, -7, -6, -17, -4, -45],
    [-15, 29, 20, 18, 25, 26, 12, -1],
    [-3, 1, 24, 24, 18, 14, -4, -23],
    [-2, 12, 6, 31, 15, 6, -11, -10],
    [-4, 0, 6, -5, -10, 9, -9, -18],
    [-26, -10, -8, -8, -16, -22, -27, -44],
    [-28, -8, -18, -22, -32, -26, -28, -46],
];
pub const PSQT_ROOK_MG: [[i16; 8]; 8] = [
    [-3, 3, 8, 17, 24, 22, 36, 4],
    [-11, -4, 7, -6, 8, 14, 27, 8],
    [-11, 0, -2, -6, -3, 3, 27, 11],
    [-11, -17, -16, 6, 1, -14, 2, 3],
    [-23, -9, 4, 9, 12, 9, 1, -34],
    [-16, 8, 13, 27, 21, 47, 29, 22],
    [-41, -28, -13, -3, -12, -7, -23, -34],
    [-44, -24, -15, -44, -42, -25, -17, -63],
];
pub const PSQT_ROOK_EG: [[i16; 8]; 8] = [
    [-12, -15, -3, -13, -12, -8, -18, -28],
    [-29, -24, -22, -11, -11, -21, -12, -43],
    [-15, 9, 6, 1, 1, 3, -2, -11],
    [0, -1, 13, 5, 2, 5, -7, -23],
    [5, 14, 11, 12, -4, -1, -6, -7],
    [2, 8, 13, -6, -6, 8, -4, -17],
    [-33, -14, -3, -9, -16, -14, -19, -35],
    [-13, -4, -3, 0, -15, -13, -19, -30],
];
pub const PSQT_QUEEN_MG: [[i16; 8]; 8] = [
    [1, 5, 2, 10, 10, -17, 9, 1],
    [-2, 5, 12, 15, 17, 23, 24, 18],
    [-3, 5, 5, -2, 4, 13, 35, 16],
    [-2, -13, -9, -1, 5, 2, 16, 17],
    [-27, -6, -5, -9, 2, 3, 3, -12],
    [-17, 1, 6, 27, 14, 41, 28, 37],
    [-43, -35, -23, -10, -14, -6, -31, -25],
    [-54, -28, -18, -47, -43, -25, -17, -70],
];
pub const PSQT_QUEEN_EG: [[i16; 8]; 8] = [
    [-36, -38, -24, -9, -7, -10, -27, -31],
    [-28, -21, -20, 1, -1, -16, -8, -43],
    [-16, 21, 20, 14, 22, 31, 15, -3],
    [1, 3, 21, 19, 12, 14, 7, -17],
    [-2, 11, 2, 20, 5, 1, -4, -2],
    [-8, -4, 4, -10, -7, 9, -4, -15],
    [-26, -13, -6, -11, -14, -12, -22, -33],
    [-28, -13, -20, -21, -28, -22, -25, -45],
];
pub const PSQT_KING_MG: [[i16; 8]; 8] = [
    [66, 101, 87, -24, 47, -8, 74, 81],
    [86, 66, 35, -10, -14, 21, 79, 83],
    [-37, -16, -66, -88, -90, -67, -8, -39],
    [-64, -84, -73, -103, -106, -85, -93, -82],
    [-70, -86, -84, -104, -103, -87, -91, -72],
    [-60, -80, -80, -100, -100, -80, -80, -61],
    [-60, -80, -80, -100, -100, -80, -80, -60],
    [-60, -80, -80, -100, -100, -80, -80, -60],
];
pub const PSQT_KING_EG: [[i16; 8]; 8] = [
    [-128, -98, -75, -73, -99, -68, -92, -154],
    [-74, -40, -23, -17, -14, -25, -52, -95],
    [-58, -12, 10, 22, 22, 7, -15, -56],
    [-49, 4, 42, 57, 48, 28, -1, -67],
    [-41, 33, 61, 80, 78, 62, 33, -41],
    [-26, 60, 70, 82, 80, 75, 68, -24],
    [-34, 34, 52, 30, 39, 53, 44, -29],
    [-79, -51, -42, -22, -19, -24, -46, -65],
];
