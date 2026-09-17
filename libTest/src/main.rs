use chess::{Color::White, Edgecase::no_moved, Rank::Bishop, *};

fn main() {

    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.fill_board();

}