_INDA26PlusPlus-datrigel-chess_

# Assignment 2: Chess Library

Hello! This is a guide to my chess library.

## High-Level Overview

The chess library is based on manipulating a `Board` object, which is an array containing 64 spots. Each spot contains a `Piece`.

A `Piece` is a struct with three fields:

- `rank`
- `color`
- `edgecase`

The board is represented internally using the following grid:

```text
| 56, 57, 58, 59, 60, 61, 62, 63 |
| ...                            |
| ...                            |
| ...                            |
| ...                            |
| ...                            |
|  8,  9, 10, 11, 12, 13, 14, 15 |
|  0,  1,  2,  3,  4,  5,  6,  7 |
```
## Main Methods: 

The methods used for manipulating the grid are:

- `init_board()`
- `fill_board(&mut self)`
- `move_piece(&mut self, usize, usize)`  
<br>

`init_board()` is used to create a board object. Takes no argumments

`fill_board()` is used to fill the board with standard chess layout. It can also be used to reset a board that is already being played on. Acts on a Board object.

`move_piece()` is the main function for playing the game. The function handelse all chess logic related to movement (except for pawn promotion). Acts on a board object, takes to usize arguments, the first is the position of the piece that you want to move, the second is where you want to move it to. 

> A invalid movement will neither change the boardstate nor count as a move for the purpose of turn_order. The move function will return true if a player tries to move but no piece has any valid moves left (checkmate occours on the reciving players turn).

## Additional functionality:

Additional functionality avalible, if wanted.

- `notation_translation(&str)` 
- `fetch_movelist(&mut self, usize)`
- `upgrade_pawn(&mut self, usize, Rank)`
- `check_square(self, usize)`  
<br>

`notation_translation()` takes in a str of two characters, ex "A5" to translate it into internal program notation. It does not care if the first letter is upper- or lowercase. Takes in a string, format "<char><number>" i.e "<a,b,c...,z><1,2,3...,9>".

`fetch_movelist()` returns a vector of all possible movements a piece can make. (this ignores if a piece is pinned or the king is checked and is written in internal program notation). Acts on a board object, takes a usize argumment that represents the position of a given piece.

`upgrade_pawn()` Can be used to upgrade a pawn if pawn has made it to the opposite side. Acts on a board object, takes in a usize argumment representing the position of the pawn, and a Rank argumment that is the Rank it is upgrading to.

`check_square()` is used to query a specific location on the board. the function uses Internal notation. It acts on a board object and takes in a usize argumment. The usize is used to specifiy what square wanted to be queried.

## How to get started:

``` rust
// Exampel start of a program.
use chess::*;

// Init a full board.
fn main() {

    // init a board object.
    let mut gameboard = Board::init_board();

    // fills the created board object.
    gameboard.fill_board();   
}
```

## Moving pieces:

``` rust
    // Translating "traditional" notation to program notation.
    let start = Board::notation_translation("e2");
    let dest = Board::notation_translation("e4");

    // Moves the piece on e2 to e4 if allowed.
    gameboard.move_piece(start, dest);
```


