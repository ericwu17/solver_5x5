use crate::moves::MoveDir;

pub fn is_permutation<T>(slice: &[T]) -> bool
where
    T: Copy,
    usize: From<T>,
{
    let n = slice.len();

    // Use a boolean array to track which numbers we've seen
    let mut seen = vec![false; n];

    for val in slice {
        let idx = usize::from(*val);

        if idx >= n {
            return false;
        }

        if seen[idx] {
            // We've seen this number before, so it's not a permutation
            return false;
        }
        seen[idx] = true;
    }

    // If we get here, all numbers 0 to n-1 appeared exactly once
    true
}

pub fn apply_orbit_with_dir_to_array<T>(arr: &mut [T], orbit: [usize; 4], dir: MoveDir)
where
    T: Copy,
{
    match dir {
        #[rustfmt::skip]
        MoveDir::CW => {
            (arr[orbit[0]], arr[orbit[1]], arr[orbit[2]], arr[orbit[3]]) = (arr[orbit[3]], arr[orbit[0]], arr[orbit[1]], arr[orbit[2]]);
        }
        #[rustfmt::skip]
        MoveDir::CCW => {
            (arr[orbit[0]], arr[orbit[1]], arr[orbit[2]], arr[orbit[3]]) = (arr[orbit[1]], arr[orbit[2]], arr[orbit[3]], arr[orbit[0]]);
        }
        #[rustfmt::skip]
        MoveDir::Dub => {
            (arr[orbit[0]], arr[orbit[1]], arr[orbit[2]], arr[orbit[3]]) = (arr[orbit[2]], arr[orbit[3]], arr[orbit[0]], arr[orbit[1]]);
        }
    }
}

pub fn apply_orbit_with_dir_to_packed_u16(arr_packed: &mut u16, orbit: [usize; 4], dir: MoveDir) {
    let mut arr: [u8; 4] = [
        (*arr_packed >> orbit[0]) as u8 & 1,
        (*arr_packed >> orbit[1]) as u8 & 1,
        (*arr_packed >> orbit[2]) as u8 & 1,
        (*arr_packed >> orbit[3]) as u8 & 1,
    ];

    match dir {
        #[rustfmt::skip]
        MoveDir::CW => {
            (arr[0], arr[1], arr[2], arr[3]) = (arr[3], arr[0], arr[1], arr[2]);
        }
        #[rustfmt::skip]
        MoveDir::CCW => {
            (arr[0], arr[1], arr[2], arr[3]) = (arr[1], arr[2], arr[3], arr[0]);
        }
        #[rustfmt::skip]
        MoveDir::Dub => {
            (arr[0], arr[1], arr[2], arr[3]) = (arr[2], arr[3], arr[0], arr[1]);
        }
    }
    *arr_packed &= !((1 << (orbit[0])) | (1 << (orbit[1])) | (1 << (orbit[2])) | (1 << (orbit[3])));
    *arr_packed |= ((arr[0] as u16) << orbit[0])
        | ((arr[1] as u16) << orbit[1])
        | ((arr[2] as u16) << orbit[2])
        | ((arr[3] as u16) << orbit[3]);
}

pub fn apply_orbit_with_dir_to_double_packed_u16(
    arr_packed: &mut u16,
    orbit: [usize; 4],
    dir: MoveDir,
) {
    let mut arr: [u8; 4] = [
        (*arr_packed >> (2 * orbit[0])) as u8 & 3,
        (*arr_packed >> (2 * orbit[1])) as u8 & 3,
        (*arr_packed >> (2 * orbit[2])) as u8 & 3,
        (*arr_packed >> (2 * orbit[3])) as u8 & 3,
    ];

    match dir {
        #[rustfmt::skip]
        MoveDir::CW => {
            (arr[0], arr[1], arr[2], arr[3]) = (arr[3], arr[0], arr[1], arr[2]);
        }
        #[rustfmt::skip]
        MoveDir::CCW => {
            (arr[0], arr[1], arr[2], arr[3]) = (arr[1], arr[2], arr[3], arr[0]);
        }
        #[rustfmt::skip]
        MoveDir::Dub => {
            (arr[0], arr[1], arr[2], arr[3]) = (arr[2], arr[3], arr[0], arr[1]);
        }
    }
    *arr_packed &= !((3 << (2 * orbit[0]))
        | (3 << (2 * orbit[1]))
        | (3 << (2 * orbit[2]))
        | (3 << (2 * orbit[3])));
    *arr_packed |= ((arr[0] as u16) << (2 * orbit[0]))
        | ((arr[1] as u16) << (2 * orbit[1]))
        | ((arr[2] as u16) << (2 * orbit[2]))
        | ((arr[3] as u16) << (2 * orbit[3]));
}

/// Converts a string of uppercase letters into an array of numbers where A=0, B=1, ..., Z=25
///
/// This macro was written by Claude.ai
///
/// # Examples
/// ```
/// let result = letters_arr!("ABCD");
/// assert_eq!(result, [0, 1, 2, 3]);
///
/// let result = letters_arr!("IJEG");
/// assert_eq!(result, [8, 9, 4, 6]);
/// ```
#[macro_export]
macro_rules! letters_arr {
    ($s:expr) => {{
        const INPUT: &str = $s;
        const INPUT_BYTES: &[u8] = INPUT.as_bytes();
        const LEN: usize = INPUT_BYTES.len();

        const fn char_to_number(c: u8) -> usize {
            match c {
                b'A'..=b'Z' => (c - b'A') as usize,
                _ => panic!("Character is not an uppercase ASCII letter"),
            }
        }

        // We need to use a const block to create the array
        const RESULT: [usize; LEN] = {
            let mut result = [0usize; LEN];
            let mut i = 0;
            while i < LEN {
                result[i] = char_to_number(INPUT_BYTES[i]);
                i += 1;
            }
            result
        };

        RESULT
    }};
}
