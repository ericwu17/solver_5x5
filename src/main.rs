pub mod state;
pub mod utils;

use state::State;

use crate::state::state_to_img::export_state_to_image;

fn main() {
    println!(
        "the size of this extremely unoptimized state is: {} bytes",
        std::mem::size_of::<State>(),
    );

    let s = State::new();
    dbg!(s.is_self_valid());
    s.assert_self_is_valid();

    export_state_to_image(&s, "out.png");
}
