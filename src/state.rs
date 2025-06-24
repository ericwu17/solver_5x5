pub mod state_to_img;

use crate::{
    letters_arr,
    moves::{Face, MoveDir, MoveType, MoveUnpkd},
    utils::{apply_orbit_with_dir_to_array, is_permutation},
};

/// Encodes the state of a 5x5 cube,
/// with data for different piece types stored separately.
///
/// x-centers, +-centers, and wings are ordered according to the speffz lettering scheme
///
/// Corners are ordered as follows:
///
///     +---+
///     |0 1|
///     |3 2|
/// +---+---+---+---+
/// |   |   |   |   |
/// |   |   |   |   |
/// +---+---+---+---+
///     |4 5|
///     |7 6|
///     +---+
///
/// And midges are ordered as follows:
///
///     +---+
///     | 0 |
///     |3 1|
///     | 2 |
/// +---+---+---+---+
/// |   |   |   |   |
/// |   |5 4|   |7 6|
/// |   |   |   |   |
/// +---+---+---+---+
///     |  8  |
///     |11  9|
///     |  10 |
///     +-----+
///
/// The typical orientation is white top, green front
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    /// A permutation of the numbers 0 through 8, where `corners_perm[i]` represents the piece at position `i`.
    pub corners_perm: [u8; 8],
    /// Each orientation number is either {0,1,2},
    /// and the sum of orientations must be 0 mod 3.
    ///
    /// The orientation number denotes the number of clockwise twists that a corner needs before the U/D sticker faces U/D.
    pub corners_ori: [u8; 8],
    /// A permutation of the numbers 0 through 11, where `midges_perm[i]` represents the piece at position `i`.
    pub midges_perm: [u8; 12],
    /// Each orientation is either {0,1},
    /// and the sum of orientations must be 0 mod 3.
    ///
    /// The edge orientation is defined to be 0 if it is a "good edge" (solvable without doing F or B moves), and 1 otherwise.
    pub midges_ori: [u8; 12],
    /// A permutation of the numbers 0 through 23, where `wings[i]` represents the piece at position `i`.
    pub wings: [u8; 24],
    /// Each number is either {0,1,2,3,4,5},
    /// where `centers_x[i]` represents the color of the center at position `i`.
    /// (0 = white, 1 = orange, 2 = green, 3 = red, 4 = blue, 5 = yellow)
    pub centers_x: [u8; 24],
    /// Each number is either {0,1,2,3,4,5},
    /// where `centers_plus[i]` represents the color of the center at position `i`.
    /// (0 = white, 1 = orange, 2 = green, 3 = red, 4 = blue, 5 = yellow)
    pub centers_plus: [u8; 24],
}

const CORNER_ORBITS: [[usize; 4]; 6] = [
    [0, 1, 2, 3], // U
    [0, 3, 4, 7], // L
    [3, 2, 5, 4], // F
    [2, 1, 6, 5], // R
    [1, 0, 7, 6], // B
    [4, 5, 6, 7], // D
];
const CORNER_ORIENTATION_CHANGES: [[u8; 4]; 6] = [
    [0, 0, 0, 0], // U
    [1, 2, 1, 2], // L
    [1, 2, 1, 2], // F
    [1, 2, 1, 2], // R
    [1, 2, 1, 2], // B
    [0, 0, 0, 0], // D
];

const MIDGE_ORBITS: [[usize; 4]; 6] = [
    [0, 1, 2, 3],   // U
    [3, 5, 11, 6],  // L
    [2, 4, 8, 5],   // F
    [1, 7, 9, 4],   // R
    [0, 6, 10, 7],  // B
    [8, 9, 10, 11], // D
];

const WING_ORBITS_OUTER: [([usize; 4], [usize; 4]); 6] = [
    (letters_arr!("ABCD"), letters_arr!("EQMI")), // U
    (letters_arr!("EFGH"), letters_arr!("DLXR")), // L
    (letters_arr!("IJKL"), letters_arr!("CPUF")), // F
    (letters_arr!("MNOP"), letters_arr!("BTVJ")), // R
    (letters_arr!("QRST"), letters_arr!("AHWN")), // B
    (letters_arr!("UVWX"), letters_arr!("KOSG")), // D
];
const WING_ORBITS_WIDE: [[usize; 4]; 6] = [
    letters_arr!("LHTP"), // U
    letters_arr!("QCKW"), // L
    letters_arr!("BOXE"), // F
    letters_arr!("ASUI"), // R
    letters_arr!("MDGV"), // B
    letters_arr!("JNRF"), // D
];

const CENTER_ORBITS_WIDE_X: [([usize; 4], [usize; 4]); 6] = [
    (letters_arr!("FRNJ"), letters_arr!("EQMI")), // U
    (letters_arr!("AIUS"), letters_arr!("DLXR")), // L
    (letters_arr!("DMVG"), letters_arr!("CPUF")), // F
    (letters_arr!("CQWK"), letters_arr!("BTVJ")), // R
    (letters_arr!("BEXO"), letters_arr!("AHWN")), // B
    (letters_arr!("LPTH"), letters_arr!("KOSG")), // D
];

impl State {
    pub fn is_self_valid(&self) -> bool {
        // CORNERS
        if !is_permutation(&self.corners_perm) {
            return false;
        }
        for x in self.corners_ori {
            if x >= 3 {
                return false;
            }
        }
        let corners_ori_sum: usize = self.corners_ori.iter().map(|x| *x as usize).sum();
        if corners_ori_sum % 3 != 0 {
            return false;
        }

        // MIDGES
        if !is_permutation(&self.midges_perm) {
            return false;
        }
        for x in self.midges_ori {
            if x >= 2 {
                return false;
            }
        }
        let midges_ori_sum: usize = self.midges_ori.iter().map(|x| *x as usize).sum();
        if midges_ori_sum % 2 != 0 {
            return false;
        }

        // WINGS
        if !is_permutation(&self.wings) {
            return false;
        }

        // + CENTERS
        for x in self.centers_plus {
            if x >= 6 {
                return false;
            }
        }
        for num in 0..6 {
            let n_tiles_of_num = self.centers_plus.iter().filter(|x| **x == num).count();
            if n_tiles_of_num != 4 {
                return false;
            }
        }

        // X CENTERS
        for x in self.centers_x {
            if x >= 6 {
                return false;
            }
        }
        for num in 0..6 {
            let n_tiles_of_num = self.centers_x.iter().filter(|x| **x == num).count();
            if n_tiles_of_num != 4 {
                return false;
            }
        }

        true
    }

    pub fn assert_self_is_valid(&self) {
        assert!(self.is_self_valid());
    }

    pub fn new() -> Self {
        State {
            corners_perm: [0, 1, 2, 3, 4, 5, 6, 7],
            corners_ori: [0; 8],
            midges_perm: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            midges_ori: [0; 12],
            wings: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23,
            ],
            centers_x: [
                0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
            ],
            centers_plus: [
                0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
            ],
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MoveableState {
    fn make_move<T>(&mut self, m: T)
    where
        T: Into<MoveUnpkd>;
}

impl MoveableState for State {
    fn make_move<T>(&mut self, m: T)
    where
        T: Into<MoveUnpkd>,
    {
        let m: MoveUnpkd = m.into();
        let face = u8::from(m.face);

        // CORNERS:
        let (cp, co) = (&mut self.corners_perm, &mut self.corners_ori);
        let c_orbit = CORNER_ORBITS[face as usize];
        let co_changes = CORNER_ORIENTATION_CHANGES[face as usize];
        apply_orbit_with_dir_to_array(cp, c_orbit, m.dir);
        apply_orbit_with_dir_to_array(co, c_orbit, m.dir);
        if m.dir != MoveDir::Dub {
            for i in 0..4 {
                co[c_orbit[i]] = (co[c_orbit[i]] + co_changes[i]) % 3;
            }
        }

        // MIDGES
        let (mp, mo) = (&mut self.midges_perm, &mut self.midges_ori);
        let m_orbit: [usize; 4] = MIDGE_ORBITS[face as usize];
        apply_orbit_with_dir_to_array(mp, m_orbit, m.dir);
        apply_orbit_with_dir_to_array(mo, m_orbit, m.dir);
        if m.dir != MoveDir::Dub && (m.face == Face::F || m.face == Face::B) {
            for i in 0..4 {
                mo[m_orbit[i]] = (mo[m_orbit[i]] + 1) % 2;
            }
        }

        // WINGS
        let w = &mut self.wings;
        let (w_outer_orbit_1, w_outer_orbit_2) = WING_ORBITS_OUTER[face as usize];
        let w_wide_orbit = WING_ORBITS_WIDE[face as usize];
        apply_orbit_with_dir_to_array(w, w_outer_orbit_1, m.dir);
        apply_orbit_with_dir_to_array(w, w_outer_orbit_2, m.dir);
        if m.type_ == MoveType::Wide {
            apply_orbit_with_dir_to_array(w, w_wide_orbit, m.dir);
        }

        // PLUS CENTERS
        let centers_plus = &mut self.centers_plus;
        // here we exploit a coincidence that the wing orbits happen to be the same as the + center orbits (in speffz)
        let center_plus_orbit_outer = w_outer_orbit_1;
        let center_plus_orbit_wide = w_outer_orbit_2;
        apply_orbit_with_dir_to_array(centers_plus, center_plus_orbit_outer, m.dir);
        if m.type_ == MoveType::Wide {
            apply_orbit_with_dir_to_array(centers_plus, center_plus_orbit_wide, m.dir);
        }

        // X CENTERS
        let centers_x = &mut self.centers_x;
        let center_x_orbit_outer = w_outer_orbit_1;
        let (center_x_orbit_wide_1, center_x_orbit_wide_2) = CENTER_ORBITS_WIDE_X[face as usize];
        apply_orbit_with_dir_to_array(centers_x, center_x_orbit_outer, m.dir);
        if m.type_ == MoveType::Wide {
            apply_orbit_with_dir_to_array(centers_x, center_x_orbit_wide_1, m.dir);
            apply_orbit_with_dir_to_array(centers_x, center_x_orbit_wide_2, m.dir);
        }
    }
}
