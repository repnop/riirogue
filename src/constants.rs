use tileset::TileSet;

pub struct TileDecl {
    pub name: &'static str,
    pub x: u32,
    pub y: u32,
}

pub const TILE_SPACE: TileDecl = TileDecl {
    name: " ",
    x: 0,
    y: 0,
};

pub const TILE_CAP_A: TileDecl = TileDecl {
    name: "A",
    x: 1,
    y: 2,
};
pub const TILE_CAP_B: TileDecl = TileDecl {
    name: "B",
    x: 2,
    y: 2,
};
pub const TILE_CAP_C: TileDecl = TileDecl {
    name: "C",
    x: 3,
    y: 2,
};
pub const TILE_CAP_D: TileDecl = TileDecl {
    name: "D",
    x: 4,
    y: 2,
};
pub const TILE_CAP_E: TileDecl = TileDecl {
    name: "E",
    x: 5,
    y: 2,
};
pub const TILE_CAP_F: TileDecl = TileDecl {
    name: "F",
    x: 6,
    y: 2,
};
pub const TILE_CAP_G: TileDecl = TileDecl {
    name: "G",
    x: 7,
    y: 2,
};
pub const TILE_CAP_H: TileDecl = TileDecl {
    name: "H",
    x: 8,
    y: 2,
};
pub const TILE_CAP_I: TileDecl = TileDecl {
    name: "I",
    x: 9,
    y: 2,
};
pub const TILE_CAP_J: TileDecl = TileDecl {
    name: "J",
    x: 10,
    y: 2,
};
pub const TILE_CAP_K: TileDecl = TileDecl {
    name: "K",
    x: 11,
    y: 2,
};
pub const TILE_CAP_L: TileDecl = TileDecl {
    name: "L",
    x: 12,
    y: 2,
};
pub const TILE_CAP_M: TileDecl = TileDecl {
    name: "M",
    x: 13,
    y: 2,
};
pub const TILE_CAP_N: TileDecl = TileDecl {
    name: "N",
    x: 14,
    y: 2,
};
pub const TILE_CAP_O: TileDecl = TileDecl {
    name: "O",
    x: 15,
    y: 2,
};
pub const TILE_CAP_P: TileDecl = TileDecl {
    name: "P",
    x: 16,
    y: 2,
};
pub const TILE_CAP_Q: TileDecl = TileDecl {
    name: "Q",
    x: 17,
    y: 2,
};
pub const TILE_CAP_R: TileDecl = TileDecl {
    name: "R",
    x: 18,
    y: 2,
};
pub const TILE_CAP_S: TileDecl = TileDecl {
    name: "S",
    x: 19,
    y: 2,
};
pub const TILE_CAP_T: TileDecl = TileDecl {
    name: "T",
    x: 20,
    y: 2,
};
pub const TILE_CAP_U: TileDecl = TileDecl {
    name: "U",
    x: 21,
    y: 2,
};
pub const TILE_CAP_V: TileDecl = TileDecl {
    name: "V",
    x: 22,
    y: 2,
};
pub const TILE_CAP_W: TileDecl = TileDecl {
    name: "W",
    x: 23,
    y: 2,
};
pub const TILE_CAP_X: TileDecl = TileDecl {
    name: "X",
    x: 24,
    y: 2,
};
pub const TILE_CAP_Y: TileDecl = TileDecl {
    name: "Y",
    x: 25,
    y: 2,
};
pub const TILE_CAP_Z: TileDecl = TileDecl {
    name: "Z",
    x: 26,
    y: 2,
};

pub const TILE_LOWER_A: TileDecl = TileDecl {
    name: "a",
    x: 1,
    y: 3,
};
pub const TILE_LOWER_B: TileDecl = TileDecl {
    name: "b",
    x: 2,
    y: 3,
};
pub const TILE_LOWER_C: TileDecl = TileDecl {
    name: "c",
    x: 3,
    y: 3,
};
pub const TILE_LOWER_D: TileDecl = TileDecl {
    name: "d",
    x: 4,
    y: 3,
};
pub const TILE_LOWER_E: TileDecl = TileDecl {
    name: "e",
    x: 5,
    y: 3,
};
pub const TILE_LOWER_F: TileDecl = TileDecl {
    name: "f",
    x: 6,
    y: 3,
};
pub const TILE_LOWER_G: TileDecl = TileDecl {
    name: "g",
    x: 7,
    y: 3,
};
pub const TILE_LOWER_H: TileDecl = TileDecl {
    name: "h",
    x: 8,
    y: 3,
};
pub const TILE_LOWER_I: TileDecl = TileDecl {
    name: "i",
    x: 9,
    y: 3,
};
pub const TILE_LOWER_J: TileDecl = TileDecl {
    name: "j",
    x: 10,
    y: 3,
};
pub const TILE_LOWER_K: TileDecl = TileDecl {
    name: "k",
    x: 11,
    y: 3,
};
pub const TILE_LOWER_L: TileDecl = TileDecl {
    name: "l",
    x: 12,
    y: 3,
};
pub const TILE_LOWER_M: TileDecl = TileDecl {
    name: "m",
    x: 13,
    y: 3,
};
pub const TILE_LOWER_N: TileDecl = TileDecl {
    name: "n",
    x: 14,
    y: 3,
};
pub const TILE_LOWER_O: TileDecl = TileDecl {
    name: "o",
    x: 15,
    y: 3,
};
pub const TILE_LOWER_P: TileDecl = TileDecl {
    name: "p",
    x: 16,
    y: 3,
};
pub const TILE_LOWER_Q: TileDecl = TileDecl {
    name: "q",
    x: 17,
    y: 3,
};
pub const TILE_LOWER_R: TileDecl = TileDecl {
    name: "r",
    x: 18,
    y: 3,
};
pub const TILE_LOWER_S: TileDecl = TileDecl {
    name: "s",
    x: 19,
    y: 3,
};
pub const TILE_LOWER_T: TileDecl = TileDecl {
    name: "t",
    x: 20,
    y: 3,
};
pub const TILE_LOWER_U: TileDecl = TileDecl {
    name: "u",
    x: 21,
    y: 3,
};
pub const TILE_LOWER_V: TileDecl = TileDecl {
    name: "v",
    x: 22,
    y: 3,
};
pub const TILE_LOWER_W: TileDecl = TileDecl {
    name: "w",
    x: 23,
    y: 3,
};
pub const TILE_LOWER_X: TileDecl = TileDecl {
    name: "x",
    x: 24,
    y: 3,
};
pub const TILE_LOWER_Y: TileDecl = TileDecl {
    name: "y",
    x: 25,
    y: 3,
};
pub const TILE_LOWER_Z: TileDecl = TileDecl {
    name: "z",
    x: 26,
    y: 3,
};

pub const TILE_NUM_0: TileDecl = TileDecl {
    name: "0",
    x: 16,
    y: 1,
};
pub const TILE_NUM_1: TileDecl = TileDecl {
    name: "1",
    x: 17,
    y: 1,
};
pub const TILE_NUM_2: TileDecl = TileDecl {
    name: "2",
    x: 18,
    y: 1,
};
pub const TILE_NUM_3: TileDecl = TileDecl {
    name: "3",
    x: 19,
    y: 1,
};
pub const TILE_NUM_4: TileDecl = TileDecl {
    name: "4",
    x: 20,
    y: 1,
};
pub const TILE_NUM_5: TileDecl = TileDecl {
    name: "5",
    x: 21,
    y: 1,
};
pub const TILE_NUM_6: TileDecl = TileDecl {
    name: "6",
    x: 22,
    y: 1,
};
pub const TILE_NUM_7: TileDecl = TileDecl {
    name: "7",
    x: 23,
    y: 1,
};
pub const TILE_NUM_8: TileDecl = TileDecl {
    name: "8",
    x: 24,
    y: 1,
};
pub const TILE_NUM_9: TileDecl = TileDecl {
    name: "9",
    x: 25,
    y: 1,
};

pub const TILE_SPEC_EXCLM: TileDecl = TileDecl {
    name: "!",
    x: 1,
    y: 1,
};
pub const TILE_SPEC_DBLQT: TileDecl = TileDecl {
    name: "\"",
    x: 2,
    y: 1,
};
pub const TILE_SPEC_POUND: TileDecl = TileDecl {
    name: "#",
    x: 3,
    y: 1,
};
pub const TILE_SPEC_DOLLAR: TileDecl = TileDecl {
    name: "$",
    x: 4,
    y: 1,
};
pub const TILE_SPEC_PCT: TileDecl = TileDecl {
    name: "%",
    x: 5,
    y: 1,
};
pub const TILE_SPEC_AMPSND: TileDecl = TileDecl {
    name: "&",
    x: 6,
    y: 1,
};
pub const TILE_SPEC_SNGQT: TileDecl = TileDecl {
    name: "'",
    x: 7,
    y: 1,
};
pub const TILE_SPEC_LPAREN: TileDecl = TileDecl {
    name: "(",
    x: 8,
    y: 1,
};
pub const TILE_SPEC_RPAREN: TileDecl = TileDecl {
    name: ")",
    x: 9,
    y: 1,
};
pub const TILE_SPEC_ASTRK: TileDecl = TileDecl {
    name: "*",
    x: 10,
    y: 1,
};
pub const TILE_SPEC_PLUS: TileDecl = TileDecl {
    name: "+",
    x: 11,
    y: 1,
};
pub const TILE_SPEC_COMMA: TileDecl = TileDecl {
    name: ",",
    x: 12,
    y: 1,
};
pub const TILE_SPEC_MINUS: TileDecl = TileDecl {
    name: "-",
    x: 13,
    y: 1,
};
pub const TILE_SPEC_PERIOD: TileDecl = TileDecl {
    name: ".",
    x: 14,
    y: 1,
};
pub const TILE_SPEC_FWDSLASH: TileDecl = TileDecl {
    name: "/",
    x: 15,
    y: 1,
};
pub const TILE_SPEC_COLON: TileDecl = TileDecl {
    name: ":",
    x: 26,
    y: 1,
};
pub const TILE_SPEC_SEMICOLON: TileDecl = TileDecl {
    name: ";",
    x: 27,
    y: 1,
};
pub const TILE_SPEC_LAGLBKT: TileDecl = TileDecl {
    name: "<",
    x: 28,
    y: 1,
};
pub const TILE_SPEC_EQSIGN: TileDecl = TileDecl {
    name: "=",
    x: 29,
    y: 1,
};
pub const TILE_SPEC_RAGLBKT: TileDecl = TileDecl {
    name: ">",
    x: 30,
    y: 1,
};
pub const TILE_SPEC_QMARK: TileDecl = TileDecl {
    name: "?",
    x: 31,
    y: 1,
};

pub const TILE_ROOM_BTMLEFT: TileDecl = TileDecl {
    name: "room_bottom_left",
    x: 8,
    y: 6,
};
pub const TILE_ROOM_BTMRIGHT: TileDecl = TileDecl {
    name: "room_bottom_right",
    x: 28,
    y: 5,
};
pub const TILE_ROOM_SIDELR: TileDecl = TileDecl {
    name: "room_side_lr",
    x: 26,
    y: 5,
};
pub const TILE_ROOM_SIDETB: TileDecl = TileDecl {
    name: "room_side_tb",
    x: 13,
    y: 6,
};
pub const TILE_ROOM_TOPLEFT: TileDecl = TileDecl {
    name: "room_top_left",
    x: 27,
    y: 5,
};
pub const TILE_ROOM_TOPRIGHT: TileDecl = TileDecl {
    name: "room_top_right",
    x: 9,
    y: 6,
};
pub const TILE_ROOM_WALL: TileDecl = TileDecl {
    name: "wall",
    x: 17,
    y: 5,
};
pub const TILE_ROOM_FLRSCHVY: TileDecl = TileDecl {
    name: "floor_scatter_heavy",
    x: 13,
    y: 7,
};
pub const TILE_ROOM_FLRSCLGT: TileDecl = TileDecl {
    name: "floor_scatter_light",
    x: 14,
    y: 7,
};
pub const TILE_ROOM_GRASS: TileDecl = TileDecl {
    name: "grass",
    x: 27,
    y: 7,
};
pub const TILE_ROOM_SOLID: TileDecl = TileDecl {
    name: "solid",
    x: 27,
    y: 6,
};
pub const TILE_PATH: TileDecl = TileDecl {
    name: "path",
    x: 27,
    y: 6,
};

const TILES: [TileDecl; 96] = [
    TILE_SPACE,
    TILE_CAP_A,
    TILE_CAP_B,
    TILE_CAP_C,
    TILE_CAP_D,
    TILE_CAP_E,
    TILE_CAP_F,
    TILE_CAP_G,
    TILE_CAP_H,
    TILE_CAP_I,
    TILE_CAP_J,
    TILE_CAP_K,
    TILE_CAP_L,
    TILE_CAP_M,
    TILE_CAP_N,
    TILE_CAP_O,
    TILE_CAP_P,
    TILE_CAP_Q,
    TILE_CAP_R,
    TILE_CAP_S,
    TILE_CAP_T,
    TILE_CAP_U,
    TILE_CAP_V,
    TILE_CAP_W,
    TILE_CAP_X,
    TILE_CAP_Y,
    TILE_CAP_Z,
    TILE_LOWER_A,
    TILE_LOWER_B,
    TILE_LOWER_C,
    TILE_LOWER_D,
    TILE_LOWER_E,
    TILE_LOWER_F,
    TILE_LOWER_G,
    TILE_LOWER_H,
    TILE_LOWER_I,
    TILE_LOWER_J,
    TILE_LOWER_K,
    TILE_LOWER_L,
    TILE_LOWER_M,
    TILE_LOWER_N,
    TILE_LOWER_O,
    TILE_LOWER_P,
    TILE_LOWER_Q,
    TILE_LOWER_R,
    TILE_LOWER_S,
    TILE_LOWER_T,
    TILE_LOWER_U,
    TILE_LOWER_V,
    TILE_LOWER_W,
    TILE_LOWER_X,
    TILE_LOWER_Y,
    TILE_LOWER_Z,
    TILE_NUM_0,
    TILE_NUM_1,
    TILE_NUM_2,
    TILE_NUM_3,
    TILE_NUM_4,
    TILE_NUM_5,
    TILE_NUM_6,
    TILE_NUM_7,
    TILE_NUM_8,
    TILE_NUM_9,
    TILE_SPEC_EXCLM,
    TILE_SPEC_DBLQT,
    TILE_SPEC_POUND,
    TILE_SPEC_DOLLAR,
    TILE_SPEC_PCT,
    TILE_SPEC_AMPSND,
    TILE_SPEC_SNGQT,
    TILE_SPEC_LPAREN,
    TILE_SPEC_RPAREN,
    TILE_SPEC_ASTRK,
    TILE_SPEC_PLUS,
    TILE_SPEC_COMMA,
    TILE_SPEC_MINUS,
    TILE_SPEC_PERIOD,
    TILE_SPEC_FWDSLASH,
    TILE_SPEC_COLON,
    TILE_SPEC_SEMICOLON,
    TILE_SPEC_LAGLBKT,
    TILE_SPEC_EQSIGN,
    TILE_SPEC_RAGLBKT,
    TILE_SPEC_QMARK,
    TILE_ROOM_BTMLEFT,
    TILE_ROOM_BTMRIGHT,
    TILE_ROOM_SIDELR,
    TILE_ROOM_SIDETB,
    TILE_ROOM_TOPLEFT,
    TILE_ROOM_TOPRIGHT,
    TILE_ROOM_WALL,
    TILE_ROOM_FLRSCHVY,
    TILE_ROOM_FLRSCLGT,
    TILE_ROOM_GRASS,
    TILE_ROOM_SOLID,
    TILE_PATH,
];

pub fn register_tiles(ts: &mut TileSet) -> Result<(), ()> {
    for tile in (&TILES).iter() {
        ts.register_tile(tile.name, (tile.x, tile.y))?;
    }

    Ok(())
}
