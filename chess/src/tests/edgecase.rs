use crate::{Board, Color::{Black, White}, Edgecase::{has_moved, no_moved}, NULLTOKEN, Rank::{King, Pawn, Queen, Rook}};

#[test]
fn npassant() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    // places pieces on board
    gameboard.set_demon(12, White, Pawn, no_moved);
    gameboard.set_demon(27, Black, Pawn, has_moved);
    // Move white pawn upp.
    gameboard.move_piece(12, 28);
    // Take white pawn, with npassant, with black pawn.
    gameboard.move_piece(27, 20);

    assert_eq!(gameboard.check_square(28), NULLTOKEN);
    assert_eq!(gameboard.check_square(20).color, Black); 

}

#[test]
fn castling_white() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.set_demon(4, White, King, no_moved);
    gameboard.set_demon(0, White, Rook, no_moved);
    // castle move with king.
    gameboard.move_piece(4,2);

    assert_eq!(gameboard.check_square(2).rank, King);
    assert_eq!(gameboard.check_square(3).rank, Rook)
}

#[test]
fn castling_black() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.set_demon(60, Black, King, no_moved);
    gameboard.set_demon(56, Black, Rook, no_moved);
    // Need to set turn order to black
    gameboard.turn_order = Black;
    // castle move with king.
    gameboard.move_piece(60,58);

    assert_eq!(gameboard.check_square(58).rank, King);
    assert_eq!(gameboard.check_square(59).rank, Rook)
}

#[test]
fn upgrade_black_pawn() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.set_demon(12, Black, Pawn, has_moved);
    // Need to set turn order to black
    gameboard.turn_order = Black;
    // castle move with king.
    gameboard.move_piece(12,4);
    gameboard.upgrade_pawn(4, Queen);

    assert_eq!(gameboard.check_square(12), NULLTOKEN);
    assert_eq!(gameboard.check_square(4).rank, Queen);
    assert_eq!(gameboard.check_square(4).color, Black);
}
