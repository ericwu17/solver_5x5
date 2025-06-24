use itertools::Itertools;
use num_enum::{FromPrimitive, IntoPrimitive};

pub type MovePkd = u8;

#[derive(Debug)]
pub struct MoveUnpkd {
    pub face: Face,
    pub type_: MoveType,
    pub dir: MoveDir,
}

impl From<MovePkd> for MoveUnpkd {
    fn from(value: u8) -> Self {
        let face = value & 0b00000111;
        let type_ = (value & 0b00001000) >> 3;
        let dir = (value & 0b00110000) >> 4;
        MoveUnpkd {
            face: Face::from(face),
            type_: MoveType::from(type_),
            dir: MoveDir::from(dir),
        }
    }
}

impl From<MoveUnpkd> for MovePkd {
    fn from(value: MoveUnpkd) -> Self {
        let face = u8::from(value.face);
        let type_ = u8::from(value.type_);
        let dir = u8::from(value.dir);
        face | (type_ << 3) | (dir << 4)
    }
}

impl std::fmt::Display for MoveUnpkd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}

impl From<&MoveUnpkd> for String {
    fn from(value: &MoveUnpkd) -> Self {
        let mut result = String::with_capacity(3);
        match value.face {
            Face::U => result.push('U'),
            Face::L => result.push('L'),
            Face::F => result.push('F'),
            Face::R => result.push('R'),
            Face::B => result.push('B'),
            Face::D => result.push('D'),
        };
        match value.type_ {
            MoveType::Outer => {}
            MoveType::Wide => result.push('w'),
        }
        match value.dir {
            MoveDir::CW => {}
            MoveDir::CCW => result.push('\''),
            MoveDir::Dub => result.push('2'),
        }
        result
    }
}

impl From<&str> for MoveUnpkd {
    fn from(value: &str) -> Self {
        let chars: Vec<char> = value.chars().collect();
        if chars.len() < 1 {
            panic!("invalid string to be converted into MoveUnpkd")
        }

        let face = match chars[0] {
            'U' => Face::U,
            'L' => Face::L,
            'F' => Face::F,
            'R' => Face::R,
            'B' => Face::B,
            'D' => Face::D,
            _ => panic!("invalid string to be converted into MoveUnpkd"),
        };

        let type_ = if chars.len() >= 2 && chars[1] == 'w' {
            MoveType::Wide
        } else {
            MoveType::Outer
        };

        let last_char = chars[chars.len() - 1];
        let dir = match last_char {
            '\'' => MoveDir::CCW,
            '2' => MoveDir::Dub,
            _ => MoveDir::CW,
        };
        MoveUnpkd { face, type_, dir }
    }
}

pub fn convert_string_to_moves(s: &str) -> Vec<MoveUnpkd> {
    if s.is_empty() {
        return Vec::new();
    }

    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| MoveUnpkd::from(s))
        .collect()
}

pub fn convert_moves_to_string(moves: &Vec<MoveUnpkd>) -> String {
    moves.iter().map(|x| String::from(x)).join(" ")
}

#[derive(IntoPrimitive, FromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Face {
    #[num_enum(default)]
    U = 0,
    L = 1,
    F = 2,
    R = 3,
    B = 4,
    D = 5,
}

#[derive(IntoPrimitive, FromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum MoveType {
    #[num_enum(default)]
    Outer = 0,
    Wide = 1,
}

#[derive(IntoPrimitive, FromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum MoveDir {
    #[num_enum(default)]
    CW = 0,
    CCW = 1,
    Dub = 2,
}
