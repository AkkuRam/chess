use std::fmt;
use std::io::{self, Write};
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pieces {
    King(Color),
    Queen(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Pawn(Color),
    Empty,
}

#[derive(Debug, Clone)]
struct Board {
    board: [[Pieces; 8]; 8],
}

impl Board {
    fn new() -> Board {
        let mut board = [[Pieces::Empty; 8]; 8];

        // Initialize board for white pieces
        board[1] = [Pieces::Pawn(Color::White); 8];
        board[0] = [
            Pieces::Rook(Color::White),
            Pieces::Knight(Color::White),
            Pieces::Bishop(Color::White),
            Pieces::King(Color::White),
            Pieces::Queen(Color::White),
            Pieces::Bishop(Color::White),
            Pieces::Knight(Color::White),
            Pieces::Rook(Color::White),
        ];

        // Initialize board for black pieces
        board[6] = [Pieces::Pawn(Color::Black); 8];
        board[7] = [
            Pieces::Rook(Color::Black),
            Pieces::Knight(Color::Black),
            Pieces::Bishop(Color::Black),
            Pieces::King(Color::Black),
            Pieces::Queen(Color::Black),
            Pieces::Bishop(Color::Black),
            Pieces::Knight(Color::Black),
            Pieces::Rook(Color::Black),
        ];

        Board { board }
    }

    fn get_color(&self, index: (usize, usize)) -> Option<Color> {
        match self.board[index.0][index.1] {
            Pieces::King(color)
            | Pieces::Queen(color)
            | Pieces::Rook(color)
            | Pieces::Knight(color)
            | Pieces::Bishop(color)
            | Pieces::Pawn(color) => Some(color),
            Pieces::Empty => None,
        }
    }

    fn check_piece_color(&mut self, from_index: (usize, usize), to_index: (usize, usize)) -> bool {
        let capturing_piece = self.board[from_index.0][from_index.1];

        if self.get_color(from_index) != self.get_color(to_index) {
            self.board[to_index.0][to_index.1] = capturing_piece;
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        }
        return false;
    }

    fn move_pawn(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.coordinate_to_index(from);
        let to_index = self.coordinate_to_index(to);

        // print!("{:?}", from_index);
        // print!("{:?}", to_index);
    
        if self.board[from_index.0][from_index.1] != Pieces::Pawn(Color::White)
            && self.board[from_index.0][from_index.1] != Pieces::Pawn(Color::Black)
        {
            return false;
        }

        if self.board[to_index.0][to_index.1] != Pieces::Empty {
            if (to_index.0).abs_diff(from_index.0) == 1 && (to_index.1).abs_diff(from_index.1) == 1
            {
                self.check_piece_color(from_index, to_index);
            }
            return false;
        }

        let is_first_move = match self.board[from_index.0][from_index.1] {
            Pieces::Pawn(Color::White) if from_index.0 == 1 => true,
            Pieces::Pawn(Color::Black) if from_index.0 == 6 => true,
            _ => false,
        };

        if is_first_move && ((to_index.0).abs_diff(from_index.0) <= 2) && to_index.1 == from_index.1
        {
            // obstacle detection
            if self.board[from_index.0][from_index.1] == Pieces::Pawn(Color::White) {
                if self.board[from_index.0 + 1][from_index.1] != Pieces::Empty {
                    return false;
                }
            } else {
                if self.board[from_index.0 - 1][from_index.1] != Pieces::Empty {
                    return false;
                }
            }

            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            true
        } else if !is_first_move
            && ((to_index.0).abs_diff(from_index.0) == 1)
            && to_index.1 == from_index.1
        {
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            true
        } else {
            false
        }
    }

    fn move_knight(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.coordinate_to_index(from);
        let to_index = self.coordinate_to_index(to);

        print!("{:?}", from_index);
        print!("{:?}", to_index);

        if self.board[from_index.0][from_index.1] != Pieces::Knight(Color::White)
            && self.board[from_index.0][from_index.1] != Pieces::Knight(Color::Black)
        {
            return false;
        }
        if self.board[to_index.0][to_index.1] != Pieces::Empty {
            self.check_piece_color(from_index, to_index);
        }

        let dx = (to_index.0).abs_diff(from_index.0);
        let dy = (to_index.1).abs_diff(from_index.1);

        if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        } else {
            return false;
        }
    }

    fn move_king(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.coordinate_to_index(from);
        let to_index = self.coordinate_to_index(to);

        if self.board[from_index.0][from_index.1] != Pieces::King(Color::White)
            && self.board[from_index.0][from_index.1] != Pieces::King(Color::Black)
        {
            return false;
        }

        if self.board[to_index.0][to_index.1] != Pieces::Empty {
            self.check_piece_color(from_index, to_index);
        }

        if ((from_index.0).abs_diff(to_index.0)) == 1 || ((from_index.1).abs_diff(to_index.1)) == 1
        {
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        }
        return false;
    }

    fn move_bishop(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.coordinate_to_index(from);
        let to_index = self.coordinate_to_index(to);

        if self.board[from_index.0][from_index.1] != Pieces::Bishop(Color::White)
            && self.board[from_index.0][from_index.1] != Pieces::Bishop(Color::Black)
        {
            return false;
        }
        if self.board[to_index.0][to_index.1] != Pieces::Empty {
            self.check_piece_color(from_index, to_index);
        }

        let dx = (to_index.0).abs_diff(from_index.0);
        let dy = (to_index.1).abs_diff(from_index.1);

        if dx == dy {
            let x_step = if to_index.0 > from_index.0 { 1 } else { -1 };
            let y_step = if to_index.1 > from_index.1 { 1 } else { -1 };
        
            let mut current_x = from_index.0 as isize + x_step as isize;
            let mut current_y = from_index.1 as isize + y_step as isize;
        
            while (current_x as usize) != to_index.0 || (current_y as usize) != to_index.1 {
                if current_x < 0 || current_y < 0 || current_x >= 8 || current_y >= 8 { 
                    return false;
                }
        
                if self.board[current_x as usize][current_y as usize] != Pieces::Empty {
                    return false;
                }
        
                current_x += x_step as isize;
                current_y += y_step as isize;
            }
        
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        }
        
        

        false
    }

    fn move_rook(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.coordinate_to_index(from);
        let to_index = self.coordinate_to_index(to);

        if self.board[from_index.0][from_index.1] != Pieces::Rook(Color::White)
            && self.board[from_index.0][from_index.1] != Pieces::Rook(Color::Black)
        {
            return false;
        }

        if self.board[to_index.0][to_index.1] != Pieces::Empty {
            self.check_piece_color(from_index, to_index);
        }

        let dx = (to_index.0).abs_diff(from_index.0);
        let dy = (to_index.1).abs_diff(from_index.1);

        if (dx > 0 && dy == 0) || (dx == 0 && dy > 0) {
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        }

        return false;
    }

    fn move_queen(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.coordinate_to_index(from);
        let to_index = self.coordinate_to_index(to);

        if self.board[from_index.0][from_index.1] != Pieces::Queen(Color::White)
            && self.board[from_index.0][from_index.1] != Pieces::Queen(Color::Black)
        {
            return false;
        }
        if self.board[to_index.0][to_index.1] != Pieces::Empty {
            self.check_piece_color(from_index, to_index);
        }

        let dx = (to_index.0).abs_diff(from_index.0);
        let dy = (to_index.1).abs_diff(from_index.1);

        if dx == dy {
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        }

        if (dx > 0 && dy == 0) || (dx == 0 && dy > 0) {
            self.board[to_index.0][to_index.1] = self.board[from_index.0][from_index.1];
            self.board[from_index.0][from_index.1] = Pieces::Empty;
            return true;
        }

        return false;
    }

    fn coordinate_to_index(&self, coordinate: &str) -> (usize, usize) {
        let file = coordinate.chars().nth(0).unwrap() as usize - 'a' as usize;
        let rank = coordinate.chars().nth(1).unwrap() as usize - '1' as usize;
        (rank, file)
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match *self {
            Pieces::King(Color::White) => '♔',
            Pieces::Queen(Color::White) => '♕',
            Pieces::Rook(Color::White) => '♖',
            Pieces::Knight(Color::White) => '♘',
            Pieces::Bishop(Color::White) => '♗',
            Pieces::Pawn(Color::White) => '♙',
            Pieces::King(Color::Black) => '♚',
            Pieces::Queen(Color::Black) => '♛',
            Pieces::Rook(Color::Black) => '♜',
            Pieces::Knight(Color::Black) => '♞',
            Pieces::Bishop(Color::Black) => '♝',
            Pieces::Pawn(Color::Black) => '♟',
            Pieces::Empty => '·',
        };

        write!(f, "{}", symbol)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for piece in row {
                write!(f, "{} ", piece)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::new();
    loop {
        println!("Initial Board:");
        println!("{}", board);

        let mut from = String::new();
        print!("Enter your move (From): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut from)
            .expect("Failed to read line");
        from = from.trim().to_string();

        let mut to = String::new();
        print!("Enter your move (To): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut to).expect("Failed to read line");
        to = to.trim().to_string();

        if board.move_pawn(&from, &to)
            || board.move_bishop(&from, &to)
            || board.move_knight(&from, &to)
            || board.move_rook(&from, &to)
            || board.move_king(&from, &to)
            || board.move_queen(&from, &to)
        {
            println!("Valid move.\n");
        } else {
            println!("Invalid move.");
        }
    }
}
