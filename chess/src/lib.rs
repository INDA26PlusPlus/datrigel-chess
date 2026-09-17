//     ################################
//     #                              #
//     #         Structs/Enum         #
//     #                              #
//     ################################

#[cfg(test)]
mod tests;
use std::io::empty;

use crate::{Color::{Black, White}, Edgecase::{no_moved, npassant}, Rank::{Bishop, Empty, King, Knight, Pawn, Queen, Rook}};

// Specifing what colors exist.
    // derive -> compiler can add prewritten implements.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    Empty,
}
// Specifing what Rank a piece can be in.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rank {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Empty,
}
// Used for edge cases, castling and anpassant
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Edgecase {
    no_moved,
    npassant,
    has_moved,
}

// Struct for the pieces.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    color: Color,
    rank: Rank,
    edgecase: Edgecase,
}
// Struct for the board.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    pub squares: [Piece; 64],
    turn_order: Color,
}

//     ################################
//     #                              #
//     #      Static Variables        #
//     #                              #
//     ################################


// Creates a null object = Empty, Empty
static NULLTOKEN: Piece = Piece { 
    color: Color::Empty, 
    rank: Rank::Empty,
    edgecase: Edgecase::has_moved,
};


//     ################################
//     #                              #
//     #           Methods            #
//     #                              #
//     ################################


// All chess logic.
impl Board {
    // Creates a board object with nothing on it.
    pub fn init_board() -> Board {
        // Creates the board with demon on one square and rest null.
        let gameboard = Board { 
            squares: [NULLTOKEN; 64],
            turn_order: White,
        };    
        return gameboard;
    }

    // fills board with all pieces.
    pub fn fill_board(&mut self) {
        // resets board fully
        self.squares.fill(NULLTOKEN);
        // set turn order to White
        self.turn_order = White;

        //     ################################
        //     #        Init boardstate       #
        //     ################################ 
        
        // Places white rooks onto the board.
        self.set_demon(0, White, Rook, no_moved);
        self.set_demon(7, White, Rook, no_moved);
        
        // Places white bishop onto the board.
        self.set_demon(2, White, Bishop, no_moved);
        self.set_demon(5, White, Bishop, no_moved);

        // Places white knights onto the board.
        self.set_demon(1, White, Knight, no_moved);
        self.set_demon(6, White, Knight, no_moved);

        // Places white queen and king
        self.set_demon(3, White, Queen, no_moved);
        self.set_demon(4, White, King, no_moved);
        
        // Places white pawns on the front row.
        for i in 0..=7 {
            let white_pawn_index = i as usize + 8;
            self.set_demon(white_pawn_index, White, Pawn, no_moved);
        }
        
        // Places black rooks onto the board.
        self.set_demon(56, Black, Rook, no_moved);
        self.set_demon(63, Black, Rook, no_moved);
        
        // Places black bishop onto the board.
        self.set_demon(58, Black, Bishop, no_moved);
        self.set_demon(61, Black, Bishop, no_moved);

        // Places black knights onto the board.
        self.set_demon(57, Black, Knight, no_moved);
        self.set_demon(62, Black, Knight, no_moved);

        // Places black queen and king
        self.set_demon(59, Black, Queen, no_moved);
        self.set_demon(60, Black, King, no_moved);
        
        // Places white pawns on the front row.
        for i in 0..=7 {
            let white_pawn_index = i as usize + 48;
            self.set_demon(white_pawn_index, Black, Pawn, no_moved);
        }
    }

    // Places a demon (Or any piece) onto the board.
    fn set_demon(&mut self, start: usize, color_piece: Color, rank_piece: Rank, edgecase_piece: Edgecase) {
         // Creates a demon object = White,Demon
        let demon = Piece { 
            color: color_piece,
            rank: rank_piece,
            edgecase: edgecase_piece,
        }; 
        // Takes in a created Board object and sets the input square to the demon.      
        self.squares[start as usize] = demon;
    }


    // Translates, traditional chess inputs to program board notation. (A5 -> 32)
    pub fn notation_translation(start: &str) -> usize{
        // Separates the first and second characters of a string input. 
        let first_char: char = start.chars().nth(0).unwrap().to_ascii_uppercase();
        let second_char: char = start.chars().nth(1).unwrap();
        
        // Converts ascci to boardnotation
        let sum = ((first_char as u32) - ('A' as u32) + (((second_char as u32) - ('1' as u32)) * 8)) as usize;
        // to check that no out of bounds inputs are registerd.
        if sum > 63 {
            panic!("Invalid input")
        }
        return sum;
    }

    
    // Returns a list of valid moves that a given piece can make.
    pub fn fetch_movelist(&mut self, start: usize) -> Vec<usize> {
        // Matches what rank is on the starting square to fetch logic.
        match self.squares[start] {
            // Matches for move with rook
            Piece {
                color: Color,
                rank: Rank::Rook,
                edgecase: Edgecase,
            } => return self.move_rook(start),
            // Matches for move with bishop
            Piece {
                color: Color,
                rank: Rank::Bishop,
                edgecase: Edgecase,
            } => return self.move_bishop(start),
            Piece {
                color: Color,
                rank: Rank::Knight,
                edgecase: Edgecase,
            } => return self.move_knight(start),
            Piece {
                color: Color,
                rank: Rank::Queen,
                edgecase: Edgecase,
            } => return self.move_queen(start), 
            Piece {
                color: Color,
                rank: Rank::King,
                edgecase: Edgecase,
            } => return self.move_king(start),
            Piece {
                color: Color,
                rank: Rank::Pawn,
                edgecase: Edgecase,
            } => return self.move_pawn(start),
            _=> panic!("Empty square selected"),
        }
    }

    //     ################################
    //     #      Core Move Logic         #
    //     ################################

    // EXTERNAL MOVEMENT FUNCTION. 
    pub fn move_piece(&mut self, start: usize, dest: usize) -> bool {
        // turn order and if so danger_pieces.
        let danger_pieces = self.is_king_in_check(self.turn_order);
        // Check if start is same color as turn_order
        if self.check_square(start).color != self.turn_order {
            // invalid move / wrong color piece.
            return false;
        }
        // fetches move list of start.
        let move_list = self.fetch_movelist(start);
        // Check if king is in check, then if king is checkmated.
        if danger_pieces.len() != 0 {
            if self.king_checkmate(self.turn_order, danger_pieces) {
                // gameover.
                return true;
            } 
            // tests move and if move makes king stay in check, invalid.
            else {
                let board_copy: &mut Board = self;
                board_copy.move_demon(start, dest);
                if board_copy.is_king_in_check(board_copy.turn_order).len() != 0 {
                    // King still in check
                    return false;
                }
            }
        }
        // Checks if dest_usize is inside of the move_list. If so moves the piece.
        if move_list.contains(&dest) == true {
            self.move_demon(start, dest);
            if self.turn_order == White {
                self.turn_order = Black;
            } else {
                self.turn_order = White;
            }
            return false;
        } else {
            // invalid move.
            return false;
        }

    }

    // INTERNAL MOVEMENT FUNCTION.
    // Main method of moving pieces, Moves by copying to new square, setting old to nulltoken.
    fn move_demon(&mut self, start: usize, dest: usize) {
        // Checking for specific pawn behaviour. Npassnt logic.
        if self.squares[start].rank == Pawn {
            if self.squares[start].edgecase == no_moved && (start/8).abs_diff(dest/8) == 2 {
                self.squares[start].edgecase = Edgecase::npassant;
                self.squares[dest] = self.squares[start];
                self.squares[start] = NULLTOKEN;
                return;

            } else if self.squares[dest] == NULLTOKEN && start % 8 != dest % 8 {
                self.squares[dest] = self.squares[start];
                self.squares[start] = NULLTOKEN;
                // Checks what color pawn is than, sets the piece to NULLTOKEN, through npassant.
                if self.squares[dest].color == White {
                    self.squares[dest - 8] = NULLTOKEN;
                } else {
                    self.squares[dest + 8] = NULLTOKEN;
                }
                return;
            }
        } 
        // Checking for specific king behaviour. Castling logic.
        if self.squares[start].rank == King && start.abs_diff(dest) == 2 {
            // Checks if it is to the right or left.
            if start.saturating_sub(dest) != 0 {
                self.squares[dest] = self.squares[start];
                self.squares[start] = NULLTOKEN;
                // switches the rook
                self.squares[dest + 1] = self.squares[dest - 2];
                self.squares[dest - 2] = NULLTOKEN;
            } else {
                self.squares[dest] = self.squares[start];
                self.squares[start] = NULLTOKEN;
                // switches the rook
                self.squares[dest - 1] = self.squares[dest + 1];
                self.squares[dest + 1] = NULLTOKEN;
            }
            return;
        }

        self.squares[start].edgecase = Edgecase::has_moved;
        self.squares[dest] = self.squares[start];
        self.squares[start] = NULLTOKEN;
        
    }

    // Checks a given square and then returns it.
    pub fn check_square(&self, dest: usize) -> Piece {
        self.squares[dest]
    }

    //     ################################
    //     #     Piece Move Logic         #
    //     ################################

// .abs_diff(self, other: usize) -> usize , can be used to calculate the absolut difference.


    // Move logic for the Rook rank.
    fn move_rook(&mut self, start:usize) -> Vec<usize> {
        // Init list of all possible moves.
        let mut move_list: Vec<usize> = Vec::new();
        // To know starting piece color.
        let color:Color = self.check_square(start).color;

        // Init variable for counting loops.
        let mut next_index:usize = 0;
        // Move y-axis positiv
        loop {
            next_index += 8;
            if next_index + start > 63 {
                break;
            }
            if self.check_square(next_index + start) == NULLTOKEN {
                move_list.push(next_index + start);
            } else if color != self.check_square(next_index + start).color {
                move_list.push(next_index + start);
                break;
            } else {
                break;
            }
        }
        // Move y-axis negativ
        next_index = 0;
        loop {  
            next_index += 8;
            if (start.saturating_sub(next_index)) == 0 && start != next_index {
                break;
            }
            if self.check_square(start - next_index) == NULLTOKEN {
                move_list.push(start - next_index);
            } else if color != self.check_square( start - next_index).color {
                move_list.push(start - next_index);
                break;
            } else {
                break;
            }
        }
        // Move x-axis positiv
        next_index = 0;
        loop {
            next_index += 1;
            if start / 8 != (start + next_index) / 8 {
                break;
            }
            if self.check_square(next_index + start) == NULLTOKEN {
                move_list.push(next_index + start);
            } else if color != self.check_square(next_index + start).color {
                move_list.push(next_index + start);
                break;
            } else {
                break;
            }
        }
        // move x-axis negativ
        next_index = 0;
        loop {  
            next_index += 1;
            if (start / 8 != start.saturating_sub(next_index) / 8 && start != next_index) || start.saturating_sub(next_index) != start.abs_diff(next_index)  {
                break;
            }
            if self.check_square(start - next_index) == NULLTOKEN {
                move_list.push(start - next_index);
            } else if color != self.check_square( start - next_index).color {
                move_list.push(start - next_index);
                break;
            } else {
                break;
            }
        }
        return move_list;
    }


    // Move logic for the Bishop.
    fn move_bishop(&mut self, start: usize) -> Vec<usize> {
        // Init list of all possible moves.
        let mut move_list: Vec<usize> = Vec::new();
        // To know starting piece color.
        let color:Color = self.check_square(start).color;

        // Init variable for counting loops.
        let mut next_index:usize = 0;

        // Move up and right.
        loop {
            next_index += 9;
            if (start / 8) + (next_index / 9) != (start + next_index) / 8 || start + next_index > 63 {
                break;
            }
            if self.check_square(start + next_index) == NULLTOKEN{
                move_list.push(start + next_index);
            } else if color != self.check_square(start).color {
                move_list.push(start + next_index);
                break;
            } else {
                break;
            }
        }
        // Move up and left.
        next_index = 0;
        loop {
            next_index += 7;
            if (start / 8) + (next_index / 7) != (start + next_index) / 8 || start + next_index > 63 {
                break;
            }
            if self.check_square(start + next_index) == NULLTOKEN{
                move_list.push(start + next_index);
            } else if color != self.check_square(start).color {
                move_list.push(start + next_index);
                break;
            } else {
                break;
            }
        }
        // Down and right.
        next_index = 0;
        loop {
            next_index += 7;
            if (start / 8).abs_diff(next_index / 7) != start.saturating_sub(next_index) / 8 || (start.saturating_sub(next_index) == 0 && start != next_index) {
                break;
            }
            if self.check_square(start - next_index) == NULLTOKEN{
                move_list.push(start - next_index);
            } else if color != self.check_square(start).color {
                move_list.push(start - next_index);
                break;
            } else {
                break;
            }
        }
        // Down and left.
        next_index = 0;
        loop {
            next_index += 9;
            // if offset other that one left on down -> break
            if (start / 8).saturating_sub(next_index / 9) != start.saturating_sub(next_index) / 8 {
                break;
            } else if start.saturating_sub(next_index) == 0 && start != next_index {
                break;
            }
            if self.check_square(start - next_index) == NULLTOKEN{
                move_list.push(start - next_index);
            } else if color != self.check_square(start).color {
                move_list.push(start - next_index);
                break;
            } else {
                break;
            }
        }
        return move_list;
    }
    
    // Move logic for the Knight
    fn move_knight(&mut self, start: usize) -> Vec<usize>{
        // Init list of all possible moves.
        let mut move_list: Vec<usize> = Vec::new();
        // To know starting piece color.
        let color:Color = self.check_square(start).color;

        // Positiv knight movement to the right.
        for i in 0..=1 {
            let l_shape = (((2 - i) * 8) + (1 + i)) as usize;
            if (l_shape + start) / 8 != (start / 8) + (2 - i) {
                continue;
            }
            if start + l_shape < 64 && self.check_square(l_shape + start).color != color {
                move_list.push(l_shape + start)
            } 
        }
        // Positiv knight movement to the left.
        for i in 0..=1 {
            let l_shape = (((2 - i) * 8) - (1 + i)) as usize;
            if (l_shape + start) / 8 != (start / 8) + (2 - i) {
                continue;
            }
            if start + l_shape < 64 && self.check_square(l_shape + start).color != color {
                move_list.push(l_shape + start)
            } 
        }
        // negativ knight movement to the left.
        for i in 0..=1 {
            let l_shape = (((2 - i) * 8) + (1 + i)) as usize;
            if start.saturating_sub(l_shape) == 0 && start != l_shape {
                continue;
            }
            if (start - l_shape) / 8 != (start / 8).saturating_sub(2 - i) {
                continue;
            }
            if self.check_square(start - l_shape).color != color {
                move_list.push(start - l_shape)
            } 
        }
        // negativ knight movement to the right
        for i in 0..=1 {
            let l_shape = (((2 - i) * 8) - (1 + i)) as usize;
            if start.saturating_sub(l_shape) == 0 && start != l_shape {
                continue;
            }
            if (start - l_shape) / 8 != (start / 8).saturating_sub(2 - i) {
                continue;
            }
            if self.check_square(start - l_shape).color != color {
                move_list.push(start - l_shape)
            } 
        }

        return move_list;
    }
    // Move logic for the Queen.
    fn move_queen(&mut self, start: usize) -> Vec<usize>{
        // Moves like a bishop or rook.
        let mut bishop_move = self.move_bishop(start);
        let mut rook_move = self.move_rook(start);
        // appends bishop and rook move lists 
        bishop_move.append(&mut rook_move);
        return bishop_move;
    }

    // Move logic for the king.
    fn move_king(&mut self, start: usize) -> Vec<usize>{
        // Init list of all possible moves.
        let mut move_list: Vec<usize> = Vec::new();
        // To know starting piece color.
        let color:Color = self.check_square(start).color;

        // Checking if upper right, straight and left can be valid moves.
        for i in 1..=3 {
            if (start + 6 + i) < 64 && self.check_square(start + 6 + i).color != color {
                move_list.push(start + 6 + i);
            }
        }
        // Check move left.
        if start.saturating_sub(1) != 0 || start == 1 {
            if self.check_square(start - 1).color != color {
                move_list.push(start - 1);
            }
        }
        // Check move right.
        if start + 1 < 64 {
            if self.check_square(start + 1).color != color {
                move_list.push(start + 1);
            }
        }
        // Check if lower right, straight and left can be valid moves.
        for i in 1..=3 {
            if start.saturating_sub(6+i) == 0 && start != 6+i{
                continue;
            }
            if self.check_square(start - (6 + i)).color != color{
                move_list.push(start - (6 + i));
                }
        }
        // castling logic.
        if self.check_square(start).edgecase == no_moved {
            // Right side check
            let mut next_index: usize = 0;
            loop {
                next_index += 1;
                if (start + next_index) / 8 != start / 8 {
                    break;
                }
                if self.check_square(start + next_index) == NULLTOKEN {
                    continue;
                } else if self.check_square(start + next_index).rank == Rook && self.check_square(start + next_index).edgecase == no_moved {
                    move_list.push(start + 2)
                }
            }
            // Left side check
            next_index = 0;
            loop {
                next_index += 1;
                if (start.saturating_sub(next_index) == 0 && start != next_index) || ((start - next_index) / 8 != start / 8) {
                    break;
                }
                if self.check_square(start - next_index) == NULLTOKEN {
                    continue;
                } else if self.check_square(start - next_index).rank == Rook && self.check_square(start - next_index).edgecase == no_moved {
                    move_list.push(start - 2)
                }
            }

        }
        return move_list;
    }

    // Move logic pawn.
    fn move_pawn(&mut self, start: usize) -> Vec<usize> {
        // Init list of all possible moves.
        let mut move_list: Vec<usize> = Vec::new();
        
        let color:Color = self.check_square(start).color;
        if color == Color::White {            
            // To keep pawns inbounds.
            if start + 8 > 63 {
                return move_list;
            }
            // Checks if forward movement is allowed.
            if self.check_square(start + 8) == NULLTOKEN {
                move_list.push(start + 8);
                // Checks if the pawn has moved, if not -> can move twice.
                if self.check_square(start).edgecase == no_moved && self.check_square(start + 16) == NULLTOKEN {
                    move_list.push(start + 16);
                }
            }

            // hard coded left and right take.
            if (start + 9) / 8 == (start / 8) + 1 && self.check_square(start + 9).color == Black {
                move_list.push(start + 9)
            }
            if (start + 7) / 8 == (start / 8) + 1 && self.check_square(start + 7).color == Black {
                move_list.push(start + 7)
            }
            // npassant for right and left.
            if self.check_square(start - 1).edgecase == npassant && self.check_square(start + 7) == NULLTOKEN {
                move_list.push(start + 7)
            } 
            if self.check_square(start + 1).edgecase == npassant && self.check_square(start + 9) == NULLTOKEN {
                move_list.push(start + 9);
            }
        } 
        // If pawn is not white than it is Black.
        else {
            // To keep pawns inbounds.
            if start.saturating_sub(8) != start.abs_diff(8) {
                return move_list;
            }
            // Checks if forward movement is allowed.
            if self.check_square(start - 8) == NULLTOKEN {
                move_list.push(start - 8);
                // Checks if the pawn has moved, if not -> can move twice.
                if self.check_square(start).edgecase == no_moved && self.check_square(start - 16) == NULLTOKEN {
                    move_list.push(start - 16);
                }
            } 
            // hard coded left and right take. Check so capture can not be done from one side of the board to the other.
            if (start - 9) / 8 == (start / 8) - 1 && self.check_square(start - 9).color == White {
                move_list.push(start - 9)
            }
            if (start - 7) / 8 == (start / 8) - 1 && self.check_square(start - 7).color == White {
                move_list.push(start - 7)
            }
            // npassant for left and right
            if self.check_square(start - 1).edgecase == npassant && self.check_square(start - 9) == NULLTOKEN {
                move_list.push(start - 9);
            } 
            if self.check_square(start + 1).edgecase == npassant && self.check_square(start - 7) == NULLTOKEN {
                move_list.push(start - 7);
            }
        }
        return move_list;
    }

    // Pawn upgrade logic. Applied when wanting to upgrade a given pawn.
    pub fn upgrade_pawn(&mut self, start: usize, rank: Rank) {
        let pawn_piece = self.check_square(start);
        if pawn_piece.rank == Pawn || rank != Empty || rank != King || rank != Pawn {
            if self.check_square(start).color == Black && start / 8 == 0 {
                self.set_demon(start, pawn_piece.color, rank, pawn_piece.edgecase);
            } else if self.check_square(start).color == White && start / 8 == 7 {
                self.set_demon(start, pawn_piece.color, rank, pawn_piece.edgecase)
            }
        } 

        return;
    }

    //     ################################
    //     #     Checking logic           #
    //     ################################

    fn fetch_king_position(&mut self, king_color: Color) -> usize {
        // Creates a variable that tracks the king position.
        let mut king_position = 0;
        // interates over the board to find king piece.
        for i in 0..=63 {
            // searches the board for the king piece.
            if self.check_square(i).rank == King && self.check_square(i).color == king_color {
                king_position = i;
            }
        }
        return king_position;
    }

    // Checks if a given color king is in check, returns a list of each position that is checking the king.
    fn is_king_in_check(&mut self, king_color: Color) -> Vec<usize> {
        // uses function to find king position.
        let king_position = self.fetch_king_position(king_color);
        // Creates a vector of all pieces that put the king in check.
        let mut danger_pieces: Vec<usize> = Vec::new();

        for i in 0..=63 {
            // only checks move list of enemy pieces.
            if self.check_square(i).color != king_color && self.check_square(i).rank != Rank::Empty {
                // Checks if enemy pieces have the kings position on movement chart.
                if self.fetch_movelist(i).contains(&king_position) == true {
                    danger_pieces.push(i);
                }
            }
        }
        return danger_pieces;
    }

    //     ################################
    //     #     Checkmate logic          #
    //     ################################

    // Checks if one move can set the king out of check.
    // function works by adding is_king_in_check to danger_pieces.
    fn king_checkmate(&mut self, king_color: Color, danger_pieces: Vec<usize>) -> bool {
        // uses function to find king position.
        let king_position = self.fetch_king_position(king_color);
    
        // sets king_moves to contain all kings possible moves (without checking considerd.)
        let king_moves = self.fetch_movelist(king_position);
        // Loops and checks if king is still in check after movement.
        for i in 0..king_moves.len(){
            // Copies the boardstate
            let board_copy = self.clone();
            // tests the movement.
            self.move_demon(king_position, king_moves[i]);
            // checks if move puts king out of check.
            if self.is_king_in_check(king_color).len() == 0 {
                *self = board_copy;
                return false;
            }
            // if move dosent but king out of check, restore the boar state.
            *self = board_copy;
        }
        // To check if any piece of the same color can break the check.
        
        for _i in 0..danger_pieces.len() {
            // copies the boardstate
            let board_copy = self.clone();
            // Checks all white pieces and if they can move to break check.
            for _i in 0..=63 {
                if self.check_square(_i).color == king_color {
                    let move_list = self.fetch_movelist(_i);
                    // Tests all possible moves, for a given piece.
                    for m_index in 0..move_list.len() {
                        self.move_demon(_i, move_list[m_index]);
                        if self.is_king_in_check(king_color).len() == 0 {
                            *self = board_copy;
                            return false;
                        } else {
                            *self = board_copy;
                        }
                    }
                }
            }
        }

        return true;
    }

}