pub mod moves;
pub mod state;
pub mod utils;

use state::State;
use state::state_to_img::export_state_to_image;

use crate::moves::{convert_moves_to_string, convert_string_to_moves};

fn main() {
    // println!(
    //     "the size of this extremely unoptimized state is: {} bytes",
    //     std::mem::size_of::<State>(),
    // );

    let s = State::new();
    // dbg!(s.is_self_valid());
    s.assert_self_is_valid();

    export_state_to_image(&s, "out.png");

    let ms = convert_string_to_moves("B2 L2 Dw D2 Bw D' Dw' Rw2");

    dbg!(convert_moves_to_string(&ms));
    dbg!(ms);
}
