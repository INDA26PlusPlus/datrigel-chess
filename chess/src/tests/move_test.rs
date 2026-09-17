use crate::{Board, Color::{Black, White}, Edgecase::{has_moved, no_moved}, NULLTOKEN, Rank::{Bishop, King, Knight, Pawn, Queen, Rook}};


#[test]
fn dubble_move_pawn() {
    // init boardstate.
    let mut gameboard = Board::init_board();

    gameboard.set_demon(12, White, Pawn, no_moved);
    gameboard.move_piece(12, 28);
    assert_eq!(gameboard.check_square(12), NULLTOKEN);
    assert_eq!(gameboard.check_square(28).rank, Pawn);
}

#[test]
fn move_pawn() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.set_demon(12, White, Pawn, no_moved);
    // first movement.
    gameboard.move_piece(12, 20);
    assert_eq!(gameboard.check_square(12), NULLTOKEN);
    assert_eq!(gameboard.check_square(20).rank, Pawn);
    // repeated movement are not allowed.
    gameboard.move_piece(20, 28);
    assert_eq!(gameboard.check_square(20).rank, Pawn);
    assert_eq!(gameboard.check_square(28), NULLTOKEN);
}

#[test]
fn take_with_pawn() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.set_demon(12, White, Pawn, no_moved);
    gameboard.set_demon(19, Black, Pawn, has_moved);
    // first movement.
    gameboard.move_piece(12, 19);
    assert_eq!(gameboard.check_square(12), NULLTOKEN);
    assert_eq!(gameboard.check_square(19).rank, Pawn);
}

#[test]
fn teamkill_pawn() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.set_demon(12, White, Pawn, no_moved);
    gameboard.set_demon(19, White, King, has_moved);
    // Pawn tries to take same color.
    gameboard.move_piece(12, 19);
    assert_eq!(gameboard.check_square(12).rank, Pawn);
    assert_eq!(gameboard.check_square(19).rank, King);
}

#[test]
fn move_test_rook() {
    // init boardstate.
    let mut gameboard = Board::init_board();

    gameboard.set_demon(28, White, Rook, no_moved);
    let mut move_list_result = gameboard.fetch_movelist(28);
    let mut move_list_expect: Vec<usize> = vec![29,30,31,27,26,25,24,20,12,4,36,44,52,60];
    
    move_list_expect.sort();
    move_list_result.sort();

    assert_eq!(move_list_expect,move_list_result)
}

#[test]
fn move_test_bishop() {
    // init boardstate.
    let mut gameboard = Board::init_board();

    gameboard.set_demon(28, White, Bishop, no_moved);
    let mut move_list_result = gameboard.fetch_movelist(28);
    
    print!("{:?}", move_list_result);

    let mut move_list_expect: Vec<usize> = vec![37,46,55,35,42,49,56,21,14,7,19,10,1];
    
    move_list_expect.sort();
    move_list_result.sort();

    assert_eq!(move_list_expect,move_list_result)
}

#[test]
fn move_test_knight() {
    // init boardstate.
    let mut gameboard = Board::init_board();

    gameboard.set_demon(28, White, Knight, no_moved);
    let mut move_list_result = gameboard.fetch_movelist(28);
    let mut move_list_expect: Vec<usize> = vec![43,45,34,18,11,13,38,22];
    
    move_list_expect.sort();
    move_list_result.sort();

    assert_eq!(move_list_expect,move_list_result)

}

#[test]
fn move_test_queen() {
    // init boardstate.
    let mut gameboard = Board::init_board();

    gameboard.set_demon(28, White, Queen, no_moved);
    let mut move_list_result = gameboard.fetch_movelist(28);
    let mut move_list_expect: Vec<usize> = vec![37,46,55,35,42,49,56,21,14,7,19,10,1,29,30,31,27,26,25,24,20,12,4,36,44,52,60];
    
    move_list_expect.sort();
    move_list_result.sort();

    assert_eq!(move_list_expect,move_list_result)
}

#[test]
fn move_test_king() {
    // init boardstate.
    let mut gameboard = Board::init_board();

    gameboard.set_demon(28, White, King, no_moved);
    let mut move_list_result = gameboard.fetch_movelist(28);
    let mut move_list_expect: Vec<usize> = vec![29,27,36,35,37,20,19,21];
    
    move_list_expect.sort();
    move_list_result.sort();

    assert_eq!(move_list_expect,move_list_result)
}

#[test]
fn move_pawn_full_board() {
    // init boardstate.
    let mut gameboard = Board::init_board();
    gameboard.fill_board();
    // tests turn_order and moving some pieces.
    gameboard.move_piece(12, 28);
    gameboard.move_piece(51, 35);
    gameboard.move_piece(28, 35);
    // moves pawn, move oposite pawn, take
    assert_eq!(gameboard.check_square(28), NULLTOKEN);
    assert_eq!(gameboard.check_square(35).rank, Pawn);
    assert_eq!(gameboard.check_square(35).color, White);
    // takes pawn with queen
    gameboard.move_piece(59, 35);
    assert_eq!(gameboard.check_square(35).rank, Queen);
    assert_eq!(gameboard.check_square(35).color, Black);
    // move for check with bishop, try to move other piece outside check.
    gameboard.move_piece(5, 41);
    gameboard.move_piece(54, 46);
    assert_ne!(gameboard.check_square(54), NULLTOKEN);

    // take with queen, check if black king is outside of check.
    gameboard.move_piece(35, 41);
    assert_eq!(gameboard.is_king_in_check(Black).len(), 0);

}