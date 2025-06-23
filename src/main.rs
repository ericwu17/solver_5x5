pub mod state;
pub mod utils;

use state::State;

use crate::state::state_to_img::export_state_to_image;

fn main() {
    println!(
        "the size of this extremely unoptimized state is: {} bytes",
        std::mem::size_of::<State>(),
    );

    let mut s = State::new();
    dbg!(s.is_self_valid());
    s.assert_self_is_valid();

    s.midges_perm = [2, 0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    s.midges_ori[4] = 1;
    s.midges_ori[2] = 1;
    export_state_to_image(&s, "out.png");
}
