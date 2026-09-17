use crate::{Board, Color::{Black, White}, Edgecase::{has_moved, no_moved}, Rank::{King, Rook}};

#[test]
fn testing_check() {
    let mut gameboard = Board::init_board();
    // init boardstate
    gameboard.set_demon(5, White, King, no_moved);
    gameboard.set_demon(29, Black, Rook, has_moved);
    // Checking if any pieces are checking the king..
    let result = gameboard.is_king_in_check(White).len();
    // If any pieces are checking the king, it works.
    assert_ne!(result, 0);
}

#[test]
fn testing_check_same_color() {
    let mut gameboard = Board::init_board();
    // init boardstate
    gameboard.set_demon(5, White, King, no_moved);
    gameboard.set_demon(29, White, Rook, has_moved);
    // Checking if any pieces are checking the king..
    let result = gameboard.is_king_in_check(White).len();
    // White rook should not be able to check king.
    assert_eq!(result, 0);
}

#[test]
fn check_mate_not_true() {
    let mut gameboard = Board::init_board();
    // init boardstate
    gameboard.set_demon(5, White, King, no_moved);
    gameboard.set_demon(29, Black, Rook, has_moved);
    // testing checkmate.

    let danger_pieces = gameboard.is_king_in_check(White);
    assert_eq!(gameboard.king_checkmate(White, danger_pieces), false)
}

#[test]
fn check_mate_true() {
    let mut gameboard = Board::init_board();
    // init boardstate
    gameboard.set_demon(4, White, King, no_moved);
    gameboard.set_demon(29, Black, Rook, has_moved);
    gameboard.set_demon(28, Black, Rook, has_moved);
    gameboard.set_demon(27, Black, Rook, has_moved);
    // testing checkmate.

    let danger_pieces = gameboard.is_king_in_check(White);
    assert_eq!(gameboard.king_checkmate(White, danger_pieces), true)
}

#[test]
fn check_mate_false_saving_angel() {
    let mut gameboard = Board::init_board();
    // init boardstate
    gameboard.set_demon(4, White, King, no_moved);
    gameboard.set_demon(29, Black, Rook, has_moved);
    gameboard.set_demon(28, Black, Rook, has_moved);
    gameboard.set_demon(27, Black, Rook, has_moved);
    // placing saving angel
    gameboard.set_demon(22, White, Rook, has_moved);
    // testing checkmate.

    let danger_pieces = gameboard.is_king_in_check(White);
    assert_eq!(gameboard.king_checkmate(White, danger_pieces), false)
}