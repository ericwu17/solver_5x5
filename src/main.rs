pub mod moves;
pub mod state;
pub mod utils;

use state::State;
use state::state_to_img::export_state_to_image;
use std::io::BufRead;

use crate::{moves::convert_string_to_moves, state::MoveableState};

fn main() {
    // test: Fw' R' B Dw Uw Rw Lw' R D2 B2 R' D2 R' Fw2 R' Fw Bw2 Uw' Rw D' L' Bw2 F2 Uw2 Bw' Uw' Rw2 L2 Fw' F' R' Fw' R' Fw' B L2 Bw L2 Bw2 F' R2 D Lw2 L Rw2 R Fw Uw2 Lw D Dw L2 Fw' D Fw2 Lw' Bw' D' Bw2 Rw'
    loop {
        let mut line = String::new();
        let stdin = std::io::stdin();
        stdin.lock().read_line(&mut line).unwrap();

        let line = line.strip_suffix("\n").unwrap();
        let moves = convert_string_to_moves(line);

        let mut s = State::new();
        for m in moves {
            s.make_move(m);
        }
        export_state_to_image(&s, "out.png");
    }
}
