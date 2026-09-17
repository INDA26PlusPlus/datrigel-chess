use crate::Board;

// Checks if board_notation_translation works for "a1".
#[test]
fn translation_base_test() {
    let result = Board::notation_translation("a1");
    assert_eq!(result, 0)
}

#[test]
fn it_works() {
    assert_eq!(1, 1)
}