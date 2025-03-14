/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::arch::Chunk;
use crate::bn462::big::NLEN;

// Base Bits= 60
pub const MODULUS: [Chunk; NLEN] = [
    0x401B00840138013,
    0x87F640000000002,
    0xFFFF6FF66FC6FF6,
    0x8F41C8020FFFFFF,
    0xD81290,
    0xFF0CF6B7D9BFCA0,
    0x23FFFFFFFFFF6,
    0x24048036012,
];
pub const ROI: [Chunk; NLEN] = [
    0x401B00840138012,
    0x87F640000000002,
    0xFFFF6FF66FC6FF6,
    0x8F41C8020FFFFFF,
    0xD81290,
    0xFF0CF6B7D9BFCA0,
    0x23FFFFFFFFFF6,
    0x24048036012,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x89118D28DC21038,
    0x1C24CD524708896,
    0x96F6AF594FD13D3,
    0xFC17B3AFB34F599,
    0x617CBFE0F54B3BC,
    0x105034B613F1E2,
    0x47E597791E4CB9,
    0x12EACA995DA,
];
pub const MCONST: Chunk = 0x718CE9E711BB5E5;
pub const SQRTM3: [Chunk; NLEN] = [
    0x402400CC0210022,
    0x2FEF80000000002,
    0xFFFF6FF4BFB07F1,
    0xAEC27C032FFFFFF,
    0xD81440,
    0xFF03F5DFD2FFB80,
    0x23FFFFFFFFFF6,
    0x24048036012,
];
pub const FRA: [Chunk; NLEN] = [
    0xEE4BE3FF2575D1A,
    0x180FC7D89659DBD,
    0x8D90D45D1D93FA1,
    0x7C0562173310CC7,
    0x87E11C792504D54,
    0x72708592ED03A9A,
    0xB40BCFDB5A8CC11,
    0x16FF4348CB,
];
pub const FRB: [Chunk; NLEN] = [
    0x2204AE0955FF85B,
    0x20902388657BEB6,
    0xD00F94F4BD630A,
    0xBADB14A1A7A84FF,
    0xE85F7B00D8E4B1D,
    0x8325B27493CA001,
    0xE90EA882008E6B6,
    0x23C37F80940,
];

//*** rom curve parameters *****
// Base Bits= 60
// Ate Bits= 118
// G2 Table size= 125

pub const CURVE_COF_I: isize = 1;
pub const CURVE_COF: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B_I: isize = 5;
pub const CURVE_B: [Chunk; NLEN] = [0x5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x401B007E010800D,
    0x17F7C0000000002,
    0xFFFF6FF66FC7BF7,
    0x8EE1C201F7FFFFF,
    0xD81290,
    0xFF0CF6B7D9BFCA0,
    0x23FFFFFFFFFF6,
    0x24048036012,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xC98D91F36B6980D,
    0x9C0372E5CD70113,
    0x69A416A0B1E7923,
    0x4B2E689DB1BBB4E,
    0x95F63B3EDBEC3CF,
    0x160B9AC9264B6F,
    0x191FADBA34A0A3,
    0x21A6D67EF25,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x8A5734D36FB03DE,
    0x32C66600622ECAA,
    0xB8AE40EB80F4754,
    0x50426E6AF77DF11,
    0xCCCFA7D788C6596,
    0x7432A490EEDA842,
    0x7F7ABB82B33676A,
    0x118EA0460F,
];
pub const CURVE_HTPC: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_BNX: [Chunk; NLEN] = [0xFFFFFFFFFFFBFFF, 0x4001FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CRU: [Chunk; NLEN] = [
    0x401F80A801A401A,
    0xDBF2E0000000002,
    0xFFFF6FF597BBBF3,
    0x9F0222029FFFFFF,
    0xD81368,
    0xFF08764BD65FC10,
    0x23FFFFFFFFFF6,
    0x24048036012,
];
pub const CURVE_PXA: [Chunk; NLEN] = [
    0x68E3D934AE1E4DF,
    0xAA6A8B488076954,
    0xBAD92E0032AE1F0,
    0x408208F9AD2699,
    0x1CD96ED61C91382,
    0xBDC5482E0337E7C,
    0x8DDA0DFB38E3A8C,
    0x257CCC85B5,
];
pub const CURVE_PXB: [Chunk; NLEN] = [
    0x3108BA6AA8CD283,
    0x8B59BF7E850E9B7,
    0x555B783718F50AF,
    0x58B18134DD86BAE,
    0x730CBED91768840,
    0x566BA3C98E2A354,
    0x99102AF8EDCA849,
    0x1D2E4343E85,
];
pub const CURVE_PYA: [Chunk; NLEN] = [
    0xE5B68DF0DB7154E,
    0x140E7B11D7C3376,
    0xC9DDAE32E03695A,
    0xFE810F1399A1F41,
    0x6E23C3FA7A6BB42,
    0x809ECA03563470,
    0x22C1979517427A2,
    0xA0650439DA,
];
pub const CURVE_PYB: [Chunk; NLEN] = [
    0xA14CC552CA2A93A,
    0x68D53743493B9EB,
    0x444A04EF87387AA,
    0x70FD725CC647692,
    0x69AC57B393F1AB3,
    0x6324D44D5E6B0C,
    0x8CBE0172C8AE373,
    0x73EF0CBD43,
];
pub const CURVE_W: [[Chunk; NLEN]; 2] = [
    [0x60020003, 0x7FFF00000000000, 0xFFFFFFFFFFFF3FF, 0x60060017FFFFF, 0x0, 0x0, 0x0, 0x0],
    [0xFFFFFFFFFFF7FFF, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
];
pub const CURVE_SB: [[[Chunk; NLEN]; 2]; 2] = [
    [
        [0x60028004, 0x77FEC0000000000, 0xFFFFFFFFFFFF3FF, 0x60060017FFFFF, 0x0, 0x0, 0x0, 0x0],
        [
            0x401B007E011000E,
            0xFF780000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
    ],
    [
        [0xFFFFFFFFFFF7FFF, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0x60020003, 0x7FFF00000000000, 0xFFFFFFFFFFFF3FF, 0x60060017FFFFF, 0x0, 0x0, 0x0, 0x0],
    ],
];
pub const CURVE_WB: [[Chunk; NLEN]; 4] = [
    [0x20004000, 0xDBFFE0000000000, 0xFFFFFFFFFFFFBFF, 0x20020007FFFFF, 0x0, 0x0, 0x0, 0x0],
    [0xFFFCFFE3FFABFFB, 0x14029FFFFFFFFFF, 0x9008002, 0xF53FBFFF9000000, 0xFFFFFFFFFFFFF6F, 0x30048024005F, 0x0, 0x0],
    [0xFFFE7FF1FFD3FFD, 0xC015FFFFFFFFFF, 0x4804001, 0xFA9FDFFFC800000, 0xFFFFFFFFFFFFFB7, 0x18024012002F, 0x0, 0x0],
    [0x2000C001, 0xD3FFA0000000000, 0xFFFFFFFFFFFFBFF, 0x20020007FFFFF, 0x0, 0x0, 0x0, 0x0],
];
pub const CURVE_BB: [[[Chunk; NLEN]; 4]; 4] = [
    [
        [0xFFFFFFFFFFFC000, 0x4001FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0xFFFFFFFFFFFBFFF, 0x4001FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0xFFFFFFFFFFFBFFF, 0x4001FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [
            0x401B007E011000F,
            0xFF780000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
    ],
    [
        [0xFFFFFFFFFFF7FFF, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [
            0x401B007E010C00E,
            0x13F7A0000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
        [
            0x401B007E010C00D,
            0x13F7A0000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
        [
            0x401B007E010C00E,
            0x13F7A0000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
    ],
    [
        [0xFFFFFFFFFFF7FFE, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0xFFFFFFFFFFF7FFF, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0xFFFFFFFFFFF7FFF, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [0xFFFFFFFFFFF7FFF, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    ],
    [
        [
            0x401B007E010C00F,
            0x13F7A0000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
        [
            0x401B007E011800F,
            0x7F740000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
        [0xFFFFFFFFFFF7FFD, 0x8003FFFFFFFFFF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        [
            0x401B007E010C00F,
            0x13F7A0000000002,
            0xFFFF6FF66FC7BF7,
            0x8EE1C201F7FFFFF,
            0xD81290,
            0xFF0CF6B7D9BFCA0,
            0x23FFFFFFFFFF6,
            0x24048036012,
        ],
    ],
];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = false;
