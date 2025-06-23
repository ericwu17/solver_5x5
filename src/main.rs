use crate::utils::is_permutation;

pub mod utils;

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
struct State {
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

impl State {
    fn is_self_valid(&self) -> bool {
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

    fn assert_self_is_valid(&self) {
        assert!(self.is_self_valid());
    }

    fn new() -> Self {
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

fn main() {
    println!(
        "the size of this extremely unoptimized state is: {} bytes",
        std::mem::size_of::<State>(),
    );

    let s = State::new();
    dbg!(s.is_self_valid());
    s.assert_self_is_valid();
}
