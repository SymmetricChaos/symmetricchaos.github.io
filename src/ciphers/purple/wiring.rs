// data references from here: https://github.com/gremmie/purple/blob/master/purple/data.py

pub const SIXES_DEC: [[usize; 6]; 25] = [
    [1, 0, 2, 4, 3, 5],
    [5, 2, 4, 1, 0, 3],
    [0, 4, 3, 5, 1, 2],
    [3, 2, 1, 0, 5, 4],
    [2, 5, 0, 3, 4, 1],
    [1, 0, 5, 4, 2, 3],
    [5, 4, 3, 1, 0, 2],
    [2, 5, 0, 3, 4, 1],
    [4, 3, 1, 5, 2, 0],
    [3, 4, 2, 1, 0, 5],
    [1, 0, 3, 4, 5, 2],
    [4, 3, 5, 2, 1, 0],
    [2, 0, 1, 5, 3, 4],
    [3, 1, 4, 0, 2, 5],
    [0, 5, 1, 2, 4, 3],
    [4, 3, 2, 5, 0, 1],
    [5, 1, 4, 2, 3, 0],
    [1, 2, 3, 0, 4, 5],
    [0, 1, 2, 4, 5, 3],
    [2, 0, 5, 3, 1, 4],
    [5, 4, 0, 1, 3, 2],
    [0, 2, 5, 3, 1, 4],
    [5, 3, 4, 0, 2, 1],
    [3, 5, 0, 1, 4, 2],
    [4, 1, 3, 2, 5, 0],
];

pub const SIXES_ENC: [[usize; 6]; 25] = [
    [1, 0, 2, 4, 3, 5],
    [4, 3, 1, 5, 2, 0],
    [0, 4, 5, 2, 1, 3],
    [3, 2, 1, 0, 5, 4],
    [2, 5, 0, 3, 4, 1],
    [1, 0, 4, 5, 3, 2],
    [4, 3, 5, 2, 1, 0],
    [2, 5, 0, 3, 4, 1],
    [5, 2, 4, 1, 0, 3],
    [4, 3, 2, 0, 1, 5],
    [1, 0, 5, 2, 3, 4],
    [5, 4, 3, 1, 0, 2],
    [1, 2, 0, 4, 5, 3],
    [3, 1, 4, 0, 2, 5],
    [0, 2, 3, 5, 4, 1],
    [4, 5, 2, 1, 0, 3],
    [5, 1, 3, 4, 2, 0],
    [3, 0, 1, 2, 4, 5],
    [0, 1, 2, 5, 3, 4],
    [1, 4, 0, 3, 5, 2],
    [2, 3, 5, 4, 1, 0],
    [0, 4, 1, 3, 5, 2],
    [3, 5, 4, 1, 2, 0],
    [2, 3, 5, 0, 4, 1],
    [5, 1, 3, 2, 0, 4],
];

pub const TWENTIES_1_DEC: [[usize; 20]; 25] = [
    [
        5, 18, 13, 0, 9, 3, 1, 6, 12, 8, 7, 15, 2, 17, 14, 10, 4, 11, 19, 16,
    ],
    [
        3, 4, 15, 16, 13, 0, 19, 14, 2, 7, 17, 10, 11, 12, 9, 18, 1, 5, 8, 6,
    ],
    [
        16, 0, 12, 5, 14, 10, 18, 11, 15, 17, 9, 2, 6, 13, 7, 19, 3, 8, 1, 4,
    ],
    [
        2, 13, 19, 3, 5, 15, 7, 18, 1, 11, 16, 8, 4, 0, 10, 9, 6, 12, 14, 17,
    ],
    [
        18, 5, 7, 19, 12, 4, 17, 3, 9, 2, 15, 14, 13, 11, 6, 1, 16, 10, 0, 8,
    ],
    [
        1, 10, 8, 13, 6, 18, 5, 2, 17, 12, 11, 7, 9, 14, 15, 16, 19, 3, 4, 0,
    ],
    [
        15, 6, 5, 17, 8, 9, 12, 0, 16, 1, 4, 3, 10, 18, 19, 13, 7, 14, 2, 11,
    ],
    [
        0, 19, 6, 15, 11, 13, 4, 17, 14, 9, 12, 5, 7, 2, 3, 8, 10, 16, 18, 1,
    ],
    [
        16, 8, 10, 7, 19, 17, 6, 13, 0, 15, 14, 4, 18, 1, 5, 11, 3, 9, 12, 2,
    ],
    [
        11, 7, 16, 8, 2, 19, 3, 9, 13, 4, 6, 17, 1, 15, 12, 5, 0, 18, 14, 10,
    ],
    [
        19, 0, 15, 10, 1, 16, 8, 3, 7, 14, 9, 12, 2, 17, 13, 4, 5, 6, 11, 18,
    ],
    [
        4, 3, 14, 1, 12, 18, 5, 15, 11, 13, 7, 6, 16, 9, 17, 2, 8, 0, 10, 19,
    ],
    [
        14, 16, 9, 18, 15, 1, 10, 7, 8, 6, 2, 13, 17, 12, 11, 0, 4, 19, 5, 3,
    ],
    [
        10, 11, 6, 2, 7, 14, 15, 5, 3, 19, 1, 4, 0, 8, 18, 17, 9, 13, 16, 12,
    ],
    [
        11, 15, 1, 6, 3, 7, 14, 18, 4, 0, 10, 8, 19, 16, 5, 13, 12, 2, 17, 9,
    ],
    [
        7, 14, 17, 0, 11, 10, 16, 13, 19, 15, 12, 18, 8, 6, 2, 3, 1, 4, 9, 5,
    ],
    [
        6, 2, 4, 17, 16, 12, 18, 19, 13, 10, 8, 9, 1, 5, 0, 14, 11, 15, 3, 7,
    ],
    [
        9, 12, 3, 13, 17, 2, 1, 16, 10, 18, 19, 0, 5, 11, 8, 6, 14, 7, 4, 15,
    ],
    [
        12, 6, 8, 11, 19, 15, 13, 9, 18, 5, 0, 1, 10, 3, 4, 2, 17, 16, 7, 14,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ],
    [
        8, 19, 11, 4, 9, 16, 0, 12, 6, 14, 3, 2, 15, 7, 17, 10, 18, 1, 13, 5,
    ],
    [
        17, 14, 1, 12, 0, 6, 9, 4, 18, 16, 5, 19, 8, 10, 11, 7, 2, 3, 15, 13,
    ],
    [
        15, 17, 18, 9, 10, 19, 4, 8, 0, 3, 11, 12, 6, 5, 16, 1, 13, 14, 2, 7,
    ],
    [
        4, 7, 0, 14, 18, 8, 11, 1, 5, 2, 13, 16, 3, 19, 15, 12, 17, 9, 6, 10,
    ],
    [
        13, 9, 3, 7, 8, 11, 2, 10, 16, 19, 18, 5, 14, 4, 1, 17, 15, 6, 0, 12,
    ],
];

pub const TWENTIES_1_ENC: [[usize; 20]; 25] = [
    [
        3, 6, 12, 5, 16, 0, 7, 10, 9, 4, 15, 17, 8, 2, 14, 11, 19, 13, 1, 18,
    ],
    [
        5, 16, 8, 0, 1, 17, 19, 9, 18, 14, 11, 12, 13, 4, 7, 2, 3, 10, 15, 6,
    ],
    [
        1, 18, 11, 16, 19, 3, 12, 14, 17, 10, 5, 7, 2, 13, 4, 8, 0, 9, 6, 15,
    ],
    [
        13, 8, 0, 3, 12, 4, 16, 6, 11, 15, 14, 9, 17, 1, 18, 5, 10, 19, 7, 2,
    ],
    [
        18, 15, 9, 7, 5, 1, 14, 2, 19, 8, 17, 13, 4, 12, 11, 10, 16, 6, 0, 3,
    ],
    [
        19, 0, 7, 17, 18, 6, 4, 11, 2, 12, 1, 10, 9, 3, 13, 14, 15, 8, 5, 16,
    ],
    [
        7, 9, 18, 11, 10, 2, 1, 16, 4, 5, 12, 19, 6, 15, 17, 0, 8, 3, 13, 14,
    ],
    [
        0, 19, 13, 14, 6, 11, 2, 12, 15, 9, 16, 4, 10, 5, 8, 3, 17, 7, 18, 1,
    ],
    [
        8, 13, 19, 16, 11, 14, 6, 3, 1, 17, 2, 15, 18, 7, 10, 9, 0, 5, 12, 4,
    ],
    [
        16, 12, 4, 6, 9, 15, 10, 1, 3, 7, 19, 0, 14, 8, 18, 13, 2, 11, 17, 5,
    ],
    [
        1, 4, 12, 7, 15, 16, 17, 8, 6, 10, 3, 18, 11, 14, 9, 2, 5, 13, 19, 0,
    ],
    [
        17, 3, 15, 1, 0, 6, 11, 10, 16, 13, 18, 8, 4, 9, 2, 7, 12, 14, 5, 19,
    ],
    [
        15, 5, 10, 19, 16, 18, 9, 7, 8, 2, 6, 14, 13, 11, 0, 4, 1, 12, 3, 17,
    ],
    [
        12, 10, 3, 8, 11, 7, 2, 4, 13, 16, 0, 1, 19, 17, 5, 6, 18, 15, 14, 9,
    ],
    [
        9, 2, 17, 4, 8, 14, 3, 5, 11, 19, 10, 0, 16, 15, 6, 1, 13, 18, 7, 12,
    ],
    [
        3, 16, 14, 15, 17, 19, 13, 0, 12, 18, 5, 4, 10, 7, 1, 9, 6, 2, 11, 8,
    ],
    [
        14, 12, 1, 18, 2, 13, 0, 19, 10, 11, 9, 16, 5, 8, 15, 17, 4, 3, 6, 7,
    ],
    [
        11, 6, 5, 2, 18, 12, 15, 17, 14, 0, 8, 13, 1, 3, 16, 19, 7, 4, 9, 10,
    ],
    [
        10, 11, 15, 13, 14, 9, 1, 18, 2, 7, 12, 3, 0, 6, 19, 5, 17, 16, 8, 4,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ],
    [
        6, 17, 11, 10, 3, 19, 8, 13, 0, 4, 15, 2, 7, 18, 9, 12, 5, 14, 16, 1,
    ],
    [
        4, 2, 16, 17, 7, 10, 5, 15, 12, 6, 13, 14, 3, 19, 1, 18, 9, 0, 8, 11,
    ],
    [
        8, 15, 18, 9, 6, 13, 12, 19, 7, 3, 4, 10, 11, 16, 17, 0, 14, 1, 2, 5,
    ],
    [
        2, 7, 9, 12, 0, 8, 18, 1, 5, 17, 19, 6, 15, 10, 3, 14, 11, 16, 4, 13,
    ],
    [
        18, 14, 6, 2, 13, 11, 17, 3, 4, 1, 7, 5, 19, 0, 12, 16, 8, 15, 10, 9,
    ],
];

pub const TWENTIES_2_DEC: [[usize; 20]; 25] = [
    [
        14, 8, 0, 4, 16, 18, 2, 1, 9, 7, 10, 17, 11, 15, 5, 12, 19, 3, 13, 6,
    ],
    [
        11, 5, 14, 1, 3, 8, 7, 15, 18, 16, 4, 10, 19, 6, 9, 17, 0, 13, 12, 2,
    ],
    [
        3, 17, 4, 7, 15, 0, 11, 14, 19, 13, 12, 16, 10, 1, 6, 8, 5, 2, 9, 18,
    ],
    [
        5, 10, 1, 19, 13, 6, 17, 11, 14, 2, 7, 4, 9, 0, 16, 18, 8, 15, 3, 12,
    ],
    [
        6, 1, 12, 2, 8, 3, 16, 13, 0, 11, 17, 19, 5, 10, 15, 14, 4, 7, 18, 9,
    ],
    [
        4, 16, 13, 6, 9, 8, 18, 19, 7, 12, 0, 1, 15, 2, 3, 11, 10, 17, 5, 14,
    ],
    [
        7, 3, 2, 10, 18, 12, 1, 8, 11, 15, 9, 16, 13, 14, 19, 5, 17, 0, 6, 4,
    ],
    [
        19, 0, 15, 9, 14, 7, 13, 10, 17, 4, 2, 6, 12, 16, 18, 3, 1, 8, 11, 5,
    ],
    [
        8, 7, 6, 14, 4, 1, 3, 12, 16, 0, 10, 5, 18, 17, 13, 9, 2, 19, 15, 11,
    ],
    [
        9, 11, 10, 17, 7, 15, 19, 16, 4, 5, 8, 2, 3, 18, 12, 6, 0, 13, 14, 1,
    ],
    [
        10, 6, 13, 3, 17, 19, 5, 0, 12, 18, 11, 14, 4, 8, 15, 1, 16, 9, 7, 2,
    ],
    [
        1, 2, 8, 9, 12, 13, 14, 15, 6, 10, 19, 11, 17, 5, 0, 4, 7, 16, 18, 3,
    ],
    [
        15, 9, 14, 0, 16, 2, 12, 8, 3, 6, 5, 7, 1, 13, 4, 10, 11, 18, 17, 19,
    ],
    [
        18, 15, 17, 11, 2, 12, 8, 9, 5, 1, 16, 13, 10, 3, 6, 19, 14, 4, 0, 7,
    ],
    [
        17, 13, 11, 18, 0, 6, 9, 5, 10, 14, 4, 8, 7, 19, 16, 3, 2, 12, 1, 15,
    ],
    [
        19, 2, 18, 1, 3, 4, 10, 13, 8, 9, 17, 15, 14, 11, 7, 6, 12, 5, 16, 0,
    ],
    [
        2, 5, 3, 13, 1, 11, 15, 4, 17, 19, 6, 18, 0, 14, 8, 7, 9, 10, 12, 16,
    ],
    [
        4, 14, 19, 8, 9, 16, 0, 18, 12, 11, 3, 1, 6, 5, 10, 13, 15, 7, 2, 17,
    ],
    [
        13, 19, 12, 16, 4, 17, 7, 3, 1, 14, 15, 0, 8, 18, 2, 5, 6, 9, 11, 10,
    ],
    [
        7, 10, 0, 5, 18, 13, 4, 17, 16, 2, 9, 12, 11, 19, 14, 15, 3, 1, 6, 8,
    ],
    [
        16, 18, 5, 0, 11, 14, 19, 6, 15, 8, 2, 10, 12, 9, 1, 17, 7, 3, 4, 13,
    ],
    [
        0, 4, 11, 19, 5, 10, 13, 7, 8, 6, 18, 3, 2, 12, 9, 16, 17, 15, 14, 1,
    ],
    [
        15, 7, 9, 12, 10, 5, 18, 4, 2, 3, 14, 19, 16, 1, 17, 0, 13, 6, 8, 11,
    ],
    [
        18, 12, 7, 15, 19, 9, 6, 0, 1, 17, 13, 5, 8, 4, 11, 2, 16, 14, 10, 3,
    ],
    [
        12, 0, 16, 14, 6, 3, 15, 2, 13, 4, 1, 9, 17, 7, 10, 8, 18, 11, 19, 5,
    ],
];

pub const TWENTIES_2_ENC: [[usize; 20]; 25] = [
    [
        2, 7, 6, 17, 3, 14, 19, 9, 1, 8, 10, 12, 15, 18, 0, 13, 4, 11, 5, 16,
    ],
    [
        16, 3, 19, 4, 10, 1, 13, 6, 5, 14, 11, 0, 18, 17, 2, 7, 9, 15, 8, 12,
    ],
    [
        5, 13, 17, 0, 2, 16, 14, 3, 15, 18, 12, 6, 10, 9, 7, 4, 11, 1, 19, 8,
    ],
    [
        13, 2, 9, 18, 11, 0, 5, 10, 16, 12, 1, 7, 19, 4, 8, 17, 14, 6, 15, 3,
    ],
    [
        8, 1, 3, 5, 16, 12, 0, 17, 4, 19, 13, 9, 2, 7, 15, 14, 6, 10, 18, 11,
    ],
    [
        10, 11, 13, 14, 0, 18, 3, 8, 5, 4, 16, 15, 9, 2, 19, 12, 1, 17, 6, 7,
    ],
    [
        17, 6, 2, 1, 19, 15, 18, 0, 7, 10, 3, 8, 5, 12, 13, 9, 11, 16, 4, 14,
    ],
    [
        1, 16, 10, 15, 9, 19, 11, 5, 17, 3, 7, 18, 12, 6, 4, 2, 13, 8, 14, 0,
    ],
    [
        9, 5, 16, 6, 4, 11, 2, 1, 0, 15, 10, 19, 7, 14, 3, 18, 8, 13, 12, 17,
    ],
    [
        16, 19, 11, 12, 8, 9, 15, 4, 10, 0, 2, 1, 14, 17, 18, 5, 7, 3, 13, 6,
    ],
    [
        7, 15, 19, 3, 12, 6, 1, 18, 13, 17, 0, 10, 8, 2, 11, 14, 16, 4, 9, 5,
    ],
    [
        14, 0, 1, 19, 15, 13, 8, 16, 2, 3, 9, 11, 4, 5, 6, 7, 17, 12, 18, 10,
    ],
    [
        3, 12, 5, 8, 14, 10, 9, 11, 7, 1, 15, 16, 6, 13, 2, 0, 4, 18, 17, 19,
    ],
    [
        18, 9, 4, 13, 17, 8, 14, 19, 6, 7, 12, 3, 5, 11, 16, 1, 10, 2, 0, 15,
    ],
    [
        4, 18, 16, 15, 10, 7, 5, 12, 11, 6, 8, 2, 17, 1, 9, 19, 14, 0, 3, 13,
    ],
    [
        19, 3, 1, 4, 5, 17, 15, 14, 8, 9, 6, 13, 16, 7, 12, 11, 18, 10, 2, 0,
    ],
    [
        12, 4, 0, 2, 7, 1, 10, 15, 14, 16, 17, 5, 18, 3, 13, 6, 19, 8, 11, 9,
    ],
    [
        6, 11, 18, 10, 0, 13, 12, 17, 3, 4, 14, 9, 8, 15, 1, 16, 5, 19, 7, 2,
    ],
    [
        11, 8, 14, 7, 4, 15, 16, 6, 12, 17, 19, 18, 2, 0, 9, 10, 3, 5, 13, 1,
    ],
    [
        2, 17, 9, 16, 6, 3, 18, 0, 19, 10, 1, 12, 11, 5, 14, 15, 8, 7, 4, 13,
    ],
    [
        3, 14, 10, 17, 18, 2, 7, 16, 9, 13, 11, 4, 12, 19, 5, 8, 0, 15, 1, 6,
    ],
    [
        0, 19, 12, 11, 1, 4, 9, 7, 8, 14, 5, 2, 13, 6, 18, 17, 15, 16, 10, 3,
    ],
    [
        15, 13, 8, 9, 7, 5, 17, 1, 18, 2, 4, 19, 3, 16, 10, 0, 12, 14, 6, 11,
    ],
    [
        7, 8, 15, 19, 13, 11, 6, 2, 12, 5, 18, 14, 1, 10, 17, 3, 16, 9, 0, 4,
    ],
    [
        1, 10, 7, 5, 9, 19, 4, 13, 15, 11, 14, 17, 0, 8, 3, 6, 2, 12, 16, 18,
    ],
];

pub const TWENTIES_3_DEC: [[usize; 20]; 25] = [
    [
        6, 18, 10, 2, 19, 0, 9, 5, 15, 11, 16, 12, 7, 8, 3, 17, 4, 13, 14, 1,
    ],
    [
        14, 16, 13, 1, 11, 12, 7, 2, 0, 18, 8, 3, 9, 6, 10, 19, 15, 5, 17, 4,
    ],
    [
        1, 10, 19, 11, 0, 18, 3, 9, 8, 13, 5, 14, 12, 2, 6, 15, 17, 7, 4, 16,
    ],
    [
        15, 2, 11, 8, 3, 19, 5, 18, 17, 1, 4, 7, 13, 10, 9, 0, 14, 16, 12, 6,
    ],
    [
        11, 17, 15, 3, 8, 2, 14, 12, 5, 19, 7, 1, 6, 9, 4, 18, 13, 0, 16, 10,
    ],
    [
        12, 8, 4, 5, 7, 6, 11, 16, 13, 17, 19, 9, 1, 18, 10, 14, 3, 2, 0, 15,
    ],
    [
        3, 6, 1, 14, 16, 9, 18, 4, 7, 15, 0, 11, 2, 12, 5, 13, 19, 8, 10, 17,
    ],
    [
        8, 5, 3, 9, 17, 15, 7, 13, 4, 11, 16, 0, 19, 14, 12, 18, 1, 10, 6, 2,
    ],
    [
        4, 13, 17, 16, 12, 14, 10, 11, 6, 7, 2, 5, 0, 1, 19, 3, 8, 9, 15, 18,
    ],
    [
        10, 15, 8, 17, 2, 11, 4, 14, 9, 0, 13, 16, 1, 3, 18, 5, 7, 6, 12, 19,
    ],
    [
        18, 7, 2, 14, 13, 4, 0, 10, 1, 9, 11, 15, 17, 19, 16, 6, 12, 3, 8, 5,
    ],
    [
        0, 11, 16, 12, 8, 6, 13, 1, 14, 3, 4, 10, 5, 15, 2, 7, 17, 18, 19, 9,
    ],
    [
        2, 3, 9, 11, 0, 17, 1, 7, 13, 12, 18, 6, 15, 5, 14, 8, 16, 19, 4, 10,
    ],
    [
        8, 10, 5, 4, 9, 3, 16, 18, 12, 14, 6, 1, 11, 17, 13, 19, 0, 15, 7, 2,
    ],
    [
        7, 12, 13, 15, 18, 11, 19, 6, 9, 2, 14, 8, 3, 16, 0, 10, 4, 1, 5, 17,
    ],
    [
        17, 15, 14, 3, 1, 16, 12, 11, 5, 10, 19, 18, 13, 4, 8, 0, 7, 6, 2, 9,
    ],
    [
        13, 0, 6, 19, 5, 12, 15, 17, 11, 8, 3, 16, 4, 10, 1, 2, 9, 14, 18, 7,
    ],
    [
        16, 18, 0, 10, 6, 1, 17, 3, 2, 7, 9, 4, 14, 11, 15, 8, 5, 12, 19, 13,
    ],
    [
        9, 14, 1, 13, 10, 5, 6, 0, 15, 19, 12, 2, 8, 7, 17, 16, 18, 4, 11, 3,
    ],
    [
        19, 8, 7, 5, 11, 10, 1, 4, 3, 6, 15, 13, 16, 2, 14, 9, 12, 18, 17, 0,
    ],
    [
        10, 19, 12, 7, 15, 9, 17, 13, 18, 5, 14, 3, 0, 16, 6, 4, 2, 8, 1, 11,
    ],
    [
        15, 4, 9, 18, 3, 17, 14, 16, 0, 2, 1, 19, 10, 5, 7, 12, 6, 11, 13, 8,
    ],
    [
        5, 9, 18, 15, 4, 8, 0, 19, 16, 3, 10, 17, 6, 13, 12, 1, 11, 7, 2, 14,
    ],
    [
        7, 6, 4, 0, 14, 13, 8, 15, 10, 16, 17, 5, 18, 19, 2, 11, 3, 1, 9, 12,
    ],
    [
        12, 1, 16, 6, 13, 7, 2, 8, 19, 4, 15, 9, 5, 0, 11, 14, 10, 17, 3, 18,
    ],
];

pub const TWENTIES_3_ENC: [[usize; 20]; 25] = [
    [
        5, 19, 3, 14, 16, 7, 0, 12, 13, 6, 2, 9, 11, 17, 18, 8, 10, 15, 1, 4,
    ],
    [
        8, 3, 7, 11, 19, 17, 13, 6, 10, 12, 14, 4, 5, 2, 0, 16, 1, 18, 9, 15,
    ],
    [
        4, 0, 13, 6, 18, 10, 14, 17, 8, 7, 1, 3, 12, 9, 11, 15, 19, 16, 5, 2,
    ],
    [
        15, 9, 1, 4, 10, 6, 19, 11, 3, 14, 13, 2, 18, 12, 16, 0, 17, 8, 7, 5,
    ],
    [
        17, 11, 5, 3, 14, 8, 12, 10, 4, 13, 19, 0, 7, 16, 6, 2, 18, 1, 15, 9,
    ],
    [
        18, 12, 17, 16, 2, 3, 5, 4, 1, 11, 14, 6, 0, 8, 15, 19, 7, 9, 13, 10,
    ],
    [
        10, 2, 12, 0, 7, 14, 1, 8, 17, 5, 18, 11, 13, 15, 3, 9, 4, 19, 6, 16,
    ],
    [
        11, 16, 19, 2, 8, 1, 18, 6, 0, 3, 17, 9, 14, 7, 13, 5, 10, 4, 15, 12,
    ],
    [
        12, 13, 10, 15, 0, 11, 8, 9, 16, 17, 6, 7, 4, 1, 5, 18, 3, 2, 19, 14,
    ],
    [
        9, 12, 4, 13, 6, 15, 17, 16, 2, 8, 0, 5, 18, 10, 7, 1, 11, 3, 14, 19,
    ],
    [
        6, 8, 2, 17, 5, 19, 15, 1, 18, 9, 7, 10, 16, 4, 3, 11, 14, 12, 0, 13,
    ],
    [
        0, 7, 14, 9, 10, 12, 5, 15, 4, 19, 11, 1, 3, 6, 8, 13, 2, 16, 17, 18,
    ],
    [
        4, 6, 0, 1, 18, 13, 11, 7, 15, 2, 19, 3, 9, 8, 14, 12, 16, 5, 10, 17,
    ],
    [
        16, 11, 19, 5, 3, 2, 10, 18, 0, 4, 1, 12, 8, 14, 9, 17, 6, 13, 7, 15,
    ],
    [
        14, 17, 9, 12, 16, 18, 7, 0, 11, 8, 15, 5, 1, 2, 10, 3, 13, 19, 4, 6,
    ],
    [
        15, 4, 18, 3, 13, 8, 17, 16, 14, 19, 9, 7, 6, 12, 2, 1, 5, 0, 11, 10,
    ],
    [
        1, 14, 15, 10, 12, 4, 2, 19, 9, 16, 13, 8, 5, 0, 17, 6, 11, 7, 18, 3,
    ],
    [
        2, 5, 8, 7, 11, 16, 4, 9, 15, 10, 3, 13, 17, 19, 12, 14, 0, 6, 1, 18,
    ],
    [
        7, 2, 11, 19, 17, 5, 6, 13, 12, 0, 4, 18, 10, 3, 1, 8, 15, 14, 16, 9,
    ],
    [
        19, 6, 13, 8, 7, 3, 9, 2, 1, 15, 5, 4, 16, 11, 14, 10, 12, 18, 17, 0,
    ],
    [
        12, 18, 16, 11, 15, 9, 14, 3, 17, 5, 0, 19, 2, 7, 10, 4, 13, 6, 8, 1,
    ],
    [
        8, 10, 9, 4, 1, 13, 16, 14, 19, 2, 12, 17, 15, 18, 6, 0, 7, 5, 3, 11,
    ],
    [
        6, 15, 18, 9, 4, 0, 12, 17, 5, 1, 10, 16, 14, 13, 19, 3, 8, 11, 2, 7,
    ],
    [
        3, 17, 14, 16, 2, 11, 1, 0, 6, 18, 8, 15, 19, 5, 4, 7, 9, 10, 12, 13,
    ],
    [
        13, 1, 6, 18, 9, 12, 3, 5, 7, 11, 16, 14, 0, 4, 15, 10, 2, 17, 19, 8,
    ],
];

// // Function used to build encryption wiring
// fn enc_to_dec() {
//     for line in INSERT_CONST_NAME {
//         let mut new = Vec::with_capacity(20);
//         for v in 0..20 {
//             new.push(line.iter().position(|x| x == &v).unwrap())
//         }
//         println!("{:?},",new);
//     }
// }
