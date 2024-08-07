use std::{ascii::AsciiExt, env, io, ops::Add, ops::Sub,fs};

#[derive(Clone, Debug)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn to_str(&self) -> &str {
        match self {
            PieceType::None => "None",
            PieceType::Pawn => "Pawn",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        }
    }
}

pub fn print_possible_moves(piece: &Piece) {
    println!(
        "The selected Piece is {}({}): dumping moves\n{{",
        piece.id,
        piece.piece_type.to_str()
    );
    for possible_move in &piece.possible_moves {
        print!(
            "{}{}, ",
            possible_move.file.to_str(),
            possible_move.rank.to_str()
        );
    }
    println!("}};");
}

pub fn piece_string(piece: &Piece) -> String {
    match piece.piece_type {
        PieceType::Pawn => match piece.color {
            PieceColor::Black => ansi_term::Color::Green.paint("P").to_string(),
            PieceColor::White => ansi_term::Color::White.paint("P").to_string(),
        },
        PieceType::Knight => match piece.color {
            PieceColor::Black => ansi_term::Color::Green.paint("N").to_string(),
            PieceColor::White => ansi_term::Color::White.paint("N").to_string(),
        },
        PieceType::Bishop => match piece.color {
            PieceColor::Black => ansi_term::Color::Green.paint("I").to_string(),
            PieceColor::White => ansi_term::Color::White.paint("I").to_string(),
        },
        PieceType::Rook => match piece.color {
            PieceColor::Black => ansi_term::Color::Green.paint("R").to_string(),
            PieceColor::White => ansi_term::Color::White.paint("R").to_string(),
        },
        PieceType::Queen => match piece.color {
            PieceColor::Black => ansi_term::Color::Green.paint("Q").to_string(),
            PieceColor::White => ansi_term::Color::White.paint("Q").to_string(),
        },
        PieceType::King => match piece.color {
            PieceColor::Black => ansi_term::Color::Green.paint("K").to_string(),
            PieceColor::White => ansi_term::Color::White.paint("K").to_string(),
        },
        _ => String::from(""), //TODO: What to do in this case. Will this ever happen???
    }
}

pub fn paint_piece(piece: &Piece) {
    ////TODO: Verify this is correct and delete redundant code
    //below
    //print!("{}",piece_string(piece));
    match piece.piece_type {
        PieceType::Pawn => match piece.color {
            PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("P")),
            PieceColor::White => print!("{}", ansi_term::Color::White.paint("P")),
        },
        PieceType::Knight => match piece.color {
            PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("N")),
            PieceColor::White => print!("{}", ansi_term::Color::White.paint("N")),
        },
        PieceType::Bishop => match piece.color {
            PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("I")),
            PieceColor::White => print!("{}", ansi_term::Color::White.paint("I")),
        },
        PieceType::Rook => match piece.color {
            PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("R")),
            PieceColor::White => print!("{}", ansi_term::Color::White.paint("R")),
        },
        PieceType::Queen => match piece.color {
            PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("Q")),
            PieceColor::White => print!("{}", ansi_term::Color::White.paint("Q")),
        },
        PieceType::King => match piece.color {
            PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("K")),
            PieceColor::White => print!("{}", ansi_term::Color::White.paint("K")),
        },
        _ => print!(""),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

trait SquareTrait {
    fn from(val: u8) -> Self;
    fn from_char(val: char) -> Self;
    fn to_str(&self) -> &str;
    fn to_u8(&self) -> u8;
}

//TODO: Need to implement iterators or ranges in some way
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Copy)]
pub enum SquareFile {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Iterator for SquareFile {
    type Item = SquareFile;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.to_u8();
        if u >= Self::H.to_u8() {
            Some(Self::from(u + 1))
        } else {
            None
        }
    }
}

impl Add<u8> for SquareFile {
    fn add(self, rhs: u8) -> Self {
        Self::from(self.to_u8() + rhs)
    }
    type Output = Self;
}

impl Sub<u8> for SquareFile {
    fn sub(self, rhs: u8) -> Self {
        Self::from(self.to_u8() - rhs)
    }
    type Output = Self;
}

enum SquareError{
    InvalidFile,
    InvalidRank,
}

type SquareResult = Result<(),SquareError>;

impl SquareFile {
    pub fn from(val: u8) -> Self {
        match val {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            //8 => Self::H, //TODO: This weird little hack...
            _ => unreachable!("{} exceeds File value!", val),
        }
    }
    pub fn from_char(val: char) -> Self {
        match val {
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            'G' | 'g' => Self::G,
            'H' | 'h' => Self::H,
            _ => unreachable!("{} exceeds File value!", val),
        }
    }

    pub fn from_checked(val: u8) -> Option<Self> {
        match val {
            1 => Some(Self::A),
            2 => Some(Self::B),
            3 => Some(Self::C),
            4 => Some(Self::D),
            5 => Some(Self::E),
            6 => Some(Self::F),
            7 => Some(Self::G),
            8 => Some(Self::H),
            //8 => Self::H, //TODO: This weird little hack...
            _ => None,
        }
    }
    pub fn from_char_checked(val: char) -> Option<Self> {
        match val {
            'A' | 'a' => Some(Self::A),
            'B' | 'b' => Some(Self::B),
            'C' | 'c' => Some(Self::C),
            'D' | 'd' => Some(Self::D),
            'E' | 'e' => Some(Self::E),
            'F' | 'f' => Some(Self::F),
            'G' | 'g' => Some(Self::G),
            'H' | 'h' => Some(Self::H),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::A => 1,
            Self::B => 2,
            Self::C => 3,
            Self::D => 4,
            Self::E => 5,
            Self::F => 6,
            Self::G => 7,
            Self::H => 8,
        }
    }
}

//TODO Need to implement iterators or ranges in some way
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub enum SquareRank {
    FIRST = 1,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    SIXTH,
    SEVENTH,
    EIGHTH,
}

impl Iterator for SquareRank {
    type Item = SquareRank;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.to_u8();
        if u >= Self::EIGHTH.to_u8() {
            Some(Self::from(u + 1))
        } else {
            None
        }
    }
}

impl Add<u8> for SquareRank {
    fn add(self, rhs: u8) -> Self {
        Self::from(self.to_u8() + rhs)
    }
    type Output = Self;
}

impl Sub<u8> for SquareRank {
    fn sub(self, rhs: u8) -> Self {
        Self::from(self.to_u8() - rhs)
    }
    type Output = Self;
}

impl SquareRank {
    pub fn from(val: u8) -> Self {
        match val {
            1 => Self::FIRST,
            2 => Self::SECOND,
            3 => Self::THIRD,
            4 => Self::FOURTH,
            5 => Self::FIFTH,
            6 => Self::SIXTH,
            7 => Self::SEVENTH,
            8 => Self::EIGHTH,
            //8 => Self::EIGHTH, //TODO: This weird little hack
            _ => unreachable!("{} exceeds Rank value!", val),
        }
    }
    pub fn from_char(val: char) -> Self {
        match val {
            '1' => Self::FIRST,
            '2' => Self::SECOND,
            '3' => Self::THIRD,
            '4' => Self::FOURTH,
            '5' => Self::FIFTH,
            '6' => Self::SIXTH,
            '7' => Self::SEVENTH,
            '8' => Self::EIGHTH,
            _ => unreachable!("{} exceeds Rank value!", val),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::FIRST => "1",
            Self::SECOND => "2",
            Self::THIRD => "3",
            Self::FOURTH => "4",
            Self::FIFTH => "5",
            Self::SIXTH => "6",
            Self::SEVENTH => "7",
            Self::EIGHTH => "8",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::FIRST => 1,
            Self::SECOND => 2,
            Self::THIRD => 3,
            Self::FOURTH => 4,
            Self::FIFTH => 5,
            Self::SIXTH => 6,
            Self::SEVENTH => 7,
            Self::EIGHTH => 8,
        }
    }
}

type SquareTuple = (SquareRank, SquareFile);

#[derive(Clone, Debug, Copy)]
pub struct Square {
    pub rank: SquareRank,
    pub file: SquareFile,
}

impl Square {
    pub fn from(tup: SquareTuple) -> Self {
        Self {
            rank: tup.0,
            file: tup.1,
        }
    }

    pub fn as_tup(&self) -> SquareTuple {
        (self.rank, self.file)
    }

    pub fn new(rank: SquareRank, file: SquareFile) -> Self {
        Self { rank, file }
    }

    pub fn eq(&self, other: &Self) -> bool {
        return (self.file == other.file) && (self.rank == other.rank);
    }

    pub fn all() -> Vec<Vec<Square>> {
        let mut ranks = vec![];
        for rank in 1..9 {
            let mut squares = vec![];
            for file in 1..9 {
                squares.push(Self::new(SquareRank::from(rank), SquareFile::from(file)));
            }
            ranks.push(squares);
        }
        ranks
    }
}

#[derive(Clone, Debug)]
pub enum TurnColor {
    WhitesTurn = 0,
    BlacksTurn,
}

#[derive(Clone, Debug)]
pub struct Board {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Vec<Square>>,
    pub turn: TurnColor,
    pub is_white_king_checked: bool,
    pub is_black_king_checked: bool,
}

pub fn get_piece_id_at(b: &Board, new_pos: &Square) -> Option<i64> {
    if let Some(piece) = get_piece_at(b, new_pos.rank, new_pos.file) {
        //println!("Selected piece is {}",piece.piece_type.to_str());
        Some(piece.id)
    } else {
        None
    }
}

//Helper because borrank checker is really mad
//
pub fn get_valid_moves_for_piece(piece: &Piece, board: &Board) -> Vec<Square> {
    let mut valid_moves = vec![];
    //TODO: Captures are not considered neither are "blocks"
    match &piece.piece_type {
        PieceType::Pawn => {
            let file = &piece.position.file;
            let rank = &piece.position.rank;
            match piece.color {
                //TODO: Remove all these clones....
                PieceColor::Black => {
                    if piece.position.rank == SquareRank::SEVENTH {
                        //Pawn has not moved
                        if !square_contains_piece_square(
                            board,
                            piece.position.rank - 2,
                            piece.position.file,
                        ) {
                            valid_moves.push(Square {
                                rank: piece.position.rank - 2,
                                file: piece.position.file,
                            });
                        }
                    }

                    if !square_contains_piece_square(
                        board,
                        piece.position.rank - 1,
                        piece.position.file,
                    ) {
                        valid_moves.push(Square {
                            rank: piece.position.rank - 1,
                            file: piece.position.file,
                        });
                    }

                    //Check for capturing pieces
                    //TODO: Add enpassant logic
                    if piece.position.file < SquareFile::G {
                        if let Some(ref mut target_piece) =
                            get_piece_at(board, piece.position.rank - 1, piece.position.file + 1)
                        {
                            if target_piece.color == PieceColor::White {
                                target_piece.set_targeted(true);
                                valid_moves.push(Square {
                                    rank: target_piece.position.rank - 1,
                                    file: target_piece.position.file + 1,
                                });
                            }
                        }
                    }
                    if piece.position.file >= SquareFile::B {
                        if let Some(ref mut target_piece) =
                            get_piece_at(board, piece.position.rank - 1, piece.position.file - 1)
                        {
                            if target_piece.color == PieceColor::White {
                                target_piece.set_targeted(true);
                                valid_moves.push(Square {
                                    rank: target_piece.position.rank - 1,
                                    file: target_piece.position.file + 1,
                                });
                            }
                            //valid_moves.push(Square{rank: piece.position.rank-1,file:piece.position.file-1}); // does this need to be here??
                        }
                    }
                }
                PieceColor::White => {
                    if piece.position.rank == SquareRank::SECOND {
                        //Pawn has not moved
                        if !square_contains_piece_square(
                            board,
                            piece.position.rank + 2,
                            piece.position.file,
                        ) {
                            valid_moves.push(Square {
                                rank: piece.position.rank + 2,
                                file: piece.position.file,
                            });
                        }
                    }

                    if !square_contains_piece_square(
                        board,
                        piece.position.rank + 1,
                        piece.position.file,
                    ) {
                        valid_moves.push(Square {
                            rank: piece.position.rank + 1,
                            file: piece.position.file,
                        });
                    }

                    //Check for capturing pieces
                    //TODO: Add enpassant logic
                    if piece.position.file < SquareFile::G {
                        if let Some(ref mut target_piece) =
                            get_piece_at(board, piece.position.rank + 1, piece.position.file + 1)
                        {
                            if target_piece.color != PieceColor::White {
                                target_piece.set_targeted(true);
                                valid_moves.push(Square {
                                    rank: target_piece.position.rank + 1,
                                    file: target_piece.position.file + 1,
                                });
                            }
                        }
                    }

                    if piece.position.file >= SquareFile::B {
                        if let Some(ref mut target_piece) =
                            get_piece_at(board, piece.position.rank + 1, piece.position.file - 1)
                        {
                            if target_piece.color != PieceColor::White {
                                target_piece.set_targeted(true);
                                valid_moves.push(Square {
                                    rank: target_piece.position.rank,
                                    file: target_piece.position.file,
                                });
                            }
                            //valid_moves.push(Square{rank: piece.position.rank+1,file:piece.position.file-1}); // I think this is extra
                        }
                    }
                }
            }
        }
        PieceType::Knight => {
            let file = piece.position.file;
            let rank = piece.position.rank;

            //println!("Knight Starting point {}{}",file.to_str(),rank.to_str());

            //TODO: Some of these are out of bounds
            if rank <= SquareRank::SIXTH && file <= SquareFile::G {
                if let Some(ref mut other) = get_piece_at(board, rank + 2, file + 1) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file + 1,
                            rank: rank + 2,
                        });
                    }
                } else {
                    //square is empty
                    valid_moves.push(Square {
                        file: file + 1,
                        rank: rank + 2,
                    });
                }
            }

            if rank >= SquareRank::THIRD && file <= SquareFile::G {
                if let Some(ref mut other) = get_piece_at(board, rank - 2, file + 1) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file + 1,
                            rank: rank - 2,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file + 1,
                        rank: rank - 2,
                    });
                }
            }

            if file >= SquareFile::B && rank <= SquareRank::SIXTH {
                if let Some(ref mut other) = get_piece_at(board, rank + 2, file - 1) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file - 1,
                            rank: rank + 2,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file - 1,
                        rank: rank + 2,
                    });
                }
            }

            if file >= SquareFile::A && rank >= SquareRank::THIRD {
                if let Some(ref mut other) = get_piece_at(board, rank - 2, file - 1) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file - 1,
                            rank: rank - 2,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file - 1,
                        rank: rank - 2,
                    });
                }
            }

            if file >= SquareFile::C && rank <= SquareRank::SIXTH {
                if let Some(ref mut other) = get_piece_at(board, rank + 1, file - 2) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file - 2,
                            rank: rank + 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file - 2,
                        rank: rank + 1,
                    });
                }
            }

            if file >= SquareFile::C && rank >= SquareRank::SECOND {
                if let Some(ref mut other) = get_piece_at(board, rank - 2, file - 2) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file - 2,
                            rank: rank - 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file - 2,
                        rank: rank - 1,
                    });
                }
            }

            if file <= SquareFile::F && rank <= SquareRank::SEVENTH {
                if let Some(ref mut other) = get_piece_at(board, rank + 1, file + 2) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file + 2,
                            rank: rank + 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file + 2,
                        rank: rank + 1,
                    });
                }
            }

            if rank >= SquareRank::SECOND && file <= SquareFile::F {
                if let Some(ref mut other) = get_piece_at(board, rank - 1, file + 2) {
                    if other.color != piece.color {
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file + 2,
                            rank: rank - 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file + 2,
                        rank: rank - 1,
                    });
                }
            }
        }

        PieceType::Bishop => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;
            //println!("Bishop Starting point {}{}",cur_file.to_str(),cur_rank.to_str());

            //TODO: Some of these might be out of bounds. Verify
            //for each diagonal
            let mut rank = cur_rank.to_u8();
            let mut file = cur_file.to_u8();
            loop {
                if rank >= SquareRank::EIGHTH as u8 || file >= SquareFile::H.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                rank = rank + 1;
                file = file + 1;
                //println!("B1: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }

            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                if rank >= SquareRank::EIGHTH as u8 || file == SquareFile::A.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }

                }
                rank = rank + 1;
                file = file - 1;

                //println!("B2: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }

            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                if rank == SquareRank::FIRST as u8 || file >= SquareFile::H.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                rank = rank - 1;
                file = file + 1;

                //println!("B3: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }

            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                if rank == SquareRank::FIRST as u8 || file == SquareFile::A.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                rank = rank - 1;
                file = file - 1;

                //println!("B4: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
        }
        PieceType::Rook => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;
            let mut rank = cur_rank.to_u8();
            let mut file = cur_file.to_u8();

            //println!("Rook Starting point {}{}",cur_file.to_str(),cur_rank.to_str());

            loop {
                if file == SquareFile::A.to_u8() || rank == SquareRank::FIRST as u8 {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                file = file - 1 as u8;
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                if rank == SquareRank::FIRST as u8 || file == SquareFile::A.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                rank = rank - 1 as u8;
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                file = file + 1 as u8;
                if file >= SquareFile::H.to_u8() || rank >= SquareRank::EIGHTH as u8 {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                rank = rank + 1 as u8;
                //TODO: what is going on here
                if rank >= 7 || file >= 7 {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
        }
        PieceType::Queen => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;

            //println!("Queen Starting point {}{}",cur_file.to_str(),cur_rank.to_str());
            //TODO: Some of these might be out of bounds. Verify
            //for each diagonal
            let mut rank = cur_rank.to_u8();
            let mut file = cur_file.to_u8();
            loop {
                rank = rank + 1 as u8;
                file = file + 1 as u8;
                if rank > SquareRank::EIGHTH as u8 || file > SquareFile::H.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();

            loop {
                rank = rank + 1 as u8;
                file = file - 1 as u8;
                if rank > SquareRank::EIGHTH as u8 || file == SquareFile::A.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();

            loop {
                rank = rank - 1;
                file = file + 1;
                if rank <= SquareRank::FIRST as u8 || file >= SquareFile::H.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }

            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                rank = rank - 1;
                file = file - 1;
                if rank <= SquareRank::FIRST as u8 || file == SquareFile::A.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }

                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                file = file - 1;
                if file <= SquareFile::A.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                rank = rank - 1;
                if rank <= SquareRank::FIRST as u8 {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                file = file + 1;
                if file > SquareFile::H.to_u8() {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                rank = rank + 1 as u8;
                if rank > SquareRank::EIGHTH as u8 {
                    break;
                }

                if let Some(ref mut other) =
                    get_piece_at(board, SquareRank::from(rank), SquareFile::from(file))
                {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: SquareFile::from(file),
                            rank: SquareRank::from(rank),
                        });
                        break;
                    }else{
                        if other.id != piece.id{
                            println!("I think I see a same color piece as me in square {}{} with id {} (I am {}{} ({})",
                            other.position.rank.to_str(),other.position.file.to_str(),other.id,
                            piece.position.rank.to_str(),piece.position.file.to_str(),piece.id);
                            break;
                        };
                    }
                }
                valid_moves.push(Square {
                    file: SquareFile::from(file),
                    rank: SquareRank::from(rank),
                });
            }
        }
        PieceType::King => {
            let file = piece.position.file;
            let rank = piece.position.rank;

            //println!("King Starting point {}{}",file.to_str(),rank.to_str());

            //TODO: Some of these moves are invalid
            if file <= SquareFile::G && rank <= SquareRank::SEVENTH {
                if let Some(ref mut other) = get_piece_at(board, rank + 1, file + 1) {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file + 1,
                            rank: rank + 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file + 1,
                        rank: rank + 1,
                    });
                }
            }

            if file <= SquareFile::G {
                if let Some(ref mut other) = get_piece_at(board, rank, file + 1) {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file + 1,
                            rank,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file + 1,
                        rank,
                    });
                }
            }

            if rank <= SquareRank::SEVENTH {
                if let Some(ref mut other) = get_piece_at(board, rank + 1, file) {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file,
                            rank: rank + 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file,
                        rank: rank + 1,
                    });
                }
            }

            if rank >= SquareRank::SECOND {
                if file <= SquareFile::G {
                    if let Some(ref mut other) = get_piece_at(board, rank - 1, file + 1) {
                        if other.color != piece.color {
                            other.set_targeted(true);
                            valid_moves.push(Square {
                                file: file + 1,
                                rank: rank - 1,
                            });
                        }
                    } else {
                        valid_moves.push(Square {
                            file: file + 1,
                            rank: rank - 1,
                        });
                    }
                    //valid_moves.push(Square{file: file,rank : rank - 1});
                }
            }
            if file >= SquareFile::A {
                if rank <= SquareRank::SEVENTH {
                    if let Some(ref mut other) = get_piece_at(board, rank + 1, file + 1) {
                        if other.color != piece.color {
                            other.set_targeted(true);
                            valid_moves.push(Square {
                                file: file - 1,
                                rank: rank + 1,
                            });
                        }
                    } else {
                        valid_moves.push(Square {
                            file: file - 1,
                            rank: rank + 1,
                        });
                    }
                }
                //valid_moves.push(Square{file: file - 1 ,rank: rank});
            }

            if rank > SquareRank::SECOND && file >= SquareFile::A {
                if let Some(ref mut other) = get_piece_at(board, rank - 1, file - 1) {
                    if other.color != piece.color {
                        other.set_targeted(true);
                        valid_moves.push(Square {
                            file: file - 1,
                            rank: rank - 1,
                        });
                    }
                } else {
                    valid_moves.push(Square {
                        file: file - 1,
                        rank: rank - 1,
                    });
                }
            }
        }
        _ => todo!("implement possible moves for {:?}", piece.piece_type),
    }

    valid_moves
}
pub fn square_contains_piece_square(b: &Board, rank: SquareRank, file: SquareFile) -> bool {
    for piece in b.pieces.as_slice() {
        match piece.piece_type {
            _ => { /* do noething*/ }
        }
        if piece.position.rank == rank && piece.position.file == file {
            return true;
        }
    }
    false
}

pub fn get_piece_at(b: &Board, rank: SquareRank, file: SquareFile) -> Option<Piece> {
    for piece in b.pieces.as_slice() {
        if piece.position.rank == rank && piece.position.file == file {
            return Some(piece.clone());
        }
    }
    None
}

#[derive(Debug)]
pub enum MoveError{
    InvalidMove,
    IllegalMove,
    NoPiece,

}

//Should this rturn bool or Result or something?
pub fn move_piece(b: &mut Board, piece_id: i64, new_pos: &Square) -> Result<(),MoveError>{
    let mut selected_piece: Option<&mut Piece> = None;
    let mut target = 0;

                    //before we move piece lets remove
                    if let Some(target_piece) = get_piece_at(
                        &b,
                        new_pos.rank,
                        new_pos.file,
                        ){
                        target = target_piece.id;
                    }
    for piece in &mut b.pieces {
        if piece.id == piece_id {
            selected_piece = Some(piece);
            break; //TODO: Investigate why we fall in this loop more than one...
        }
    }

    let mut next_move: Option<&Square> = None;
    let mut found_move = false;

    match selected_piece {
        Some(ref mut piece) => {
            print_possible_moves(piece);
            for valid_move in piece.possible_moves.as_slice() {
                if valid_move.eq(new_pos) {
                    //piece.update_pos(new_pos); //<- We cant borrank from piece again as it is
                    //borranked in the loop. defer position update

                    //check for capture before updating piece position, lest we delete the piece we just moved...
                    next_move = Some(new_pos);
                    piece.position.rank = new_pos.rank;
                    piece.position.file = new_pos.file;



                    found_move = true;

                }
            }

            if !found_move {
                println!("selected move not in list of valid moves for selected piece");
                print_possible_moves(piece);
                return Err(MoveError::InvalidMove);
            }
        }
        None => {
            println!("Could not resolve Piece_id {}", piece_id);
            return Err(MoveError::NoPiece);
            //return;
        }
    }

    if found_move {
        match next_move {
            Some(_move_) => {
            },
            None => {
                println!("Illegal move");
                return Err(MoveError::IllegalMove);
            }
        }
    }
    if target > 0 {
        //TODO: This needs to be refactored into lib
        //By contract we know that if a valid move is a capture that the piece colors are opposite. See get_valid_moves_for_piece
        b.delete_piece(target); //TODO: Currently this deletes a piece right after moving it...
    }

    Ok(())

    //for valid_move in &mut piece.possible_moves{
    //    //if we have a valid move. move the piece
    //    if valid_move.eq(new_pos){
    //        piece.update_pos(new_pos);
    //        self.recalculate_valid_moves(piece);
    //        return; // break for performance
    //    }
    //}
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Piece::all(), //vec![Piece::new();16],
            squares: Square::all(),
            turn: TurnColor::WhitesTurn,
            is_white_king_checked: false,
            is_black_king_checked: false,
        }
    }

    //TODO: Here we might want to add notation, switch clocks, or some other events that may happen
    //at the end of a turn
    pub fn advance_turn(&mut self) {
        match self.turn {
            TurnColor::WhitesTurn => self.turn = TurnColor::BlacksTurn,
            TurnColor::BlacksTurn => self.turn = TurnColor::WhitesTurn,
        }
    }

    pub fn print_pieces(&self) {
        for piece in &self.pieces {
            println!(
                "({}): {}, [{}{}]",
                piece.id,
                piece.piece_type.to_str(),
                piece.position.file.to_str(),
                piece.position.rank.to_str()
            );
        }
    }

    pub fn to_string(&self) -> String {
        let mut BoardString = String::new();

        for (idx_r, rank) in (1..9).rev().enumerate() {
            //POV: Playing as white pieces
            //for (idx_r,rank) in (1..9).enumerate(){ //POV: Playing as black pieces
            for (idx_c, file) in (1..9).enumerate() {
                //let square = chess_board.get(idx_r).expect("idx_r to be in bounds").get(idx_c).expect("idx_c to be in bounds");

                let mut square_color = ansi_term::Color::White;
                if idx_r % 2 == 0 {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::White;
                    } else {
                        square_color = ansi_term::Color::Green;
                    }
                } else {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::Green;
                    } else {
                        square_color = ansi_term::Color::White;
                    }
                }
                if square_contains_piece_square(
                    self,
                    SquareRank::from(rank),
                    SquareFile::from(file),
                ) {
                    let cur_piece =
                        get_piece_at(self, SquareRank::from(rank), SquareFile::from(file))
                            .expect("square_contains_piece_square to work as intended");
                    BoardString.push_str(piece_string(&cur_piece).as_str());
                } else {
                    BoardString.push_str(square_color.paint("#").to_string().as_str())
                }
            }
            println!("");
        }

        BoardString
    }

    pub fn try_move_piece (
        &mut self,
        src_rank: char,
        src_file: char,
        dst_rank: char,
        dst_file: char,
    ) -> Result<(),MoveError> {
        if square_contains_piece_square(
            &self,
            SquareRank::from_char(src_rank),
            SquareFile::from_char(src_file), //
        ) {
            //println!("You have selected a valid piece");
            if let Some(piece_id) = get_piece_id_at(
                &self,
                &Square {
                    rank: SquareRank::from_char(src_rank),
                    file: SquareFile::from_char(src_file), //
                },
            ){

            move_piece(
                self,
                piece_id,
                &Square {
                    file: SquareFile::from_char(dst_file),
                    rank: SquareRank::from_char(dst_rank),
                },
            )?;
            //
            //if we successfully move a piece then we tick a turn
           Ok(()) }
            else{
            Err(MoveError::NoPiece)
            }
        } else {
            Err(MoveError::NoPiece)
        }
    }

    pub fn delete_piece(&mut self, piece_id: i64) {
        let mut selected_index: usize = 0;
        let mut found = false;
        for (index, piece) in self.pieces.iter().enumerate() {
            if piece.id == piece_id {
                selected_index = index;
                found = true;
            }
        }

        if found {
            self.pieces.remove(selected_index);
        }
    }

    //TODO: Do these make sense being associated with board? Right now I think so
    //TODO: See if we can do without allocating here
    pub fn encode_board_ascii<'a>(board: Vec<ansi_term::ANSIString<'a>>) -> String {
        let mut encoded = String::new();
        for term in board {
            if let Some(term_color) = term.style_ref().foreground {
                match term_color {
                    ansi_term::Color::Green => {
                        encoded.push('b');
                    }
                    ansi_term::Color::White => {
                        encoded.push('w');
                    }
                    _ => { /*we only care about green(black) and white colors*/ }
                }
            }
            //Now that we have encoded the color, lets encode character
            encoded.push_str(term.to_ascii_uppercase().as_str());
        }
        encoded
    }

    pub fn decode_board_ascii<'a>(encoded_board: String) -> ansi_term::ANSIString<'a> {
        let mut with_color = String::new();
        let mut next_color = ansi_term::Color::White;
        for term in encoded_board.chars() {
            match term {
                'b' => {
                    next_color = ansi_term::Color::Green;
                }
                'w' => {
                    next_color = ansi_term::Color::White;
                }
                _ => {
                    //TODO: SPEED
                    with_color.push_str(&next_color.paint(String::from(term)).to_string())
                }
            }
        }

        with_color.into()
    }

    pub fn paint_board(){
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub id: i64, // For ease of search and Id
    pub position: Square,
    pub threatened: bool,
    pub possible_moves: Vec<Square>,
    pub color: PieceColor,
    pub piece_type: PieceType,
}

//TODO: tHis will be our global UUID for pieces methinks
//static CUR_PIECE_NUM = AtomicU8::new(0);

impl Piece {
    pub fn default() -> Self {
        Self {
            id: 0, //TODO: should we track this better, probably..
            position: Square {
                file: SquareFile::from(0),
                rank: SquareRank::from(0),
            },
            possible_moves: vec![],
            threatened: false,
            color: PieceColor::Black,
            piece_type: PieceType::None,
        }
    }

    pub fn new(rank: SquareRank, file: SquareFile, _color: PieceColor, _type: PieceType) -> Self {
        //CUR_PIECE_NUM.fetch_add;
        Self {
            position: Square::new(rank, file),
            color: _color,
            possible_moves: vec![],
            threatened: false,
            id: ((rank.to_u8()) as i64 * 100) + (file.to_u8() as i64), //TODO: Implement global UUID
            piece_type: _type,
        }
    }

    pub fn all() -> Vec<Self> {
        let mut pieces = vec![];
        for file in 1..9 {
            pieces.push(Self::new(
                SquareRank::SECOND,
                SquareFile::from(file),
                PieceColor::White,
                PieceType::Pawn,
            ));
            pieces.push(Self::new(
                SquareRank::SEVENTH,
                SquareFile::from(file),
                PieceColor::Black,
                PieceType::Pawn,
            ));
        }
        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::A,
            PieceColor::White,
            PieceType::Rook,
        ));
        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::H,
            PieceColor::White,
            PieceType::Rook,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::A,
            PieceColor::Black,
            PieceType::Rook,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::H,
            PieceColor::Black,
            PieceType::Rook,
        ));

        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::B,
            PieceColor::White,
            PieceType::Knight,
        ));
        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::G,
            PieceColor::White,
            PieceType::Knight,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::B,
            PieceColor::Black,
            PieceType::Knight,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::G,
            PieceColor::Black,
            PieceType::Knight,
        ));

        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::C,
            PieceColor::White,
            PieceType::Bishop,
        ));
        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::F,
            PieceColor::White,
            PieceType::Bishop,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::C,
            PieceColor::Black,
            PieceType::Bishop,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::F,
            PieceColor::Black,
            PieceType::Bishop,
        ));

        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::E,
            PieceColor::White,
            PieceType::King,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::E,
            PieceColor::Black,
            PieceType::King,
        ));

        pieces.push(Self::new(
            SquareRank::FIRST,
            SquareFile::D,
            PieceColor::White,
            PieceType::Queen,
        ));
        pieces.push(Self::new(
            SquareRank::EIGHTH,
            SquareFile::D,
            PieceColor::Black,
            PieceType::Queen,
        ));
        pieces
    }

    //NOTE: Board is in charge of validating the move, by contract,
    //      we assume that at this point the move is valid
    //      despite Pieces owning their possible valid moves
    pub fn update_pos(&mut self, new_pos: &Square) {
        self.position.file = new_pos.file;
        self.position.rank = new_pos.rank;
    }
    pub fn set_targeted(&mut self, is_targeted: bool) {
        self.threatened = is_targeted;
    }
}

pub fn auto(){
        let mut chess_board = Board::new();
        let chess_board_clone = chess_board.clone();
        for piece in &mut chess_board.pieces {
            let mut moves = get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
            piece.possible_moves.append(&mut moves);
        }

        let mut test_path_in = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path_in.push("src");
        test_path_in.push("test_ins");
        test_path_in.push("RuyLopez2_in");
        test_path_in.set_extension("txt");
        let expected_input =
            fs::read_to_string(test_path_in).unwrap_or("INVALID_TEST_INPUT".to_string());
        for move_in in expected_input.lines() {
            if move_in.len() < 5 {
                break;
            }
            //TODO: Figure out why the first character is blank and everything is offset by 1
            let piece = move_in
                .chars()
                .nth(1)
                .expect("Each move to have exactly 6 chars");
            let src_file = move_in
                .chars()
                .nth(2)
                .expect("Each move to have exactly 6 chars");
            let src_rank = move_in
                .chars()
                .nth(3)
                .expect("Each move to have exactly 6 chars");
            let dst_file = move_in
                .chars()
                .nth(4)
                .expect("Each move to have exactly 6 chars");
            let dst_rank = move_in
                .chars()
                .nth(5)
                .expect("Each move to have exactly 6 chars");

            //print!("P: {} | sf: {} | sr: {} | df: {} | dr: {}",piece,src_file,src_rank,dst_file,dst_rank);
            print!(
                "P: {} | sf: {} | sr: {} | df: {} | dr: {}",
                piece, src_file, src_rank, dst_file, dst_rank
            );

            if let Ok(_) = chess_board.try_move_piece(src_rank, src_file, dst_rank, dst_file) {
                chess_board.advance_turn();
            }
        }

        let mut test_path_out = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path_out.push("src");
        test_path_out.push("test_ins");
        test_path_out.push("RuyLopez2_out");
        test_path_out.set_extension("txt");
        let expected_output = fs::read_to_string(test_path_out);

        let mut expected = expected_output.unwrap_or("INVALID_TEST_INPUT".to_string());
        if expected.ends_with('\n') {
            expected.pop();
            if expected.ends_with('\r') {
                expected.pop();
            }
        }

        //print board
        let mut cur_pos: Vec<(u8, u8)> = vec![];
        for (idx_r, rank) in (1..9).rev().enumerate() {
            //POV: Playing as white pieces
            //for (idx_r,rank) in (1..9).enumerate(){ //POV: Playing as black pieces
            print!("{}. ", ansi_term::Color::Purple.paint((rank).to_string()));
            for (idx_c, file) in (1..9).enumerate() {
                //let square = chess_board.get(idx_r).expect("idx_r to be in bounds").get(idx_c).expect("idx_c to be in bounds");

                let mut square_color = ansi_term::Color::White;
                if idx_r % 2 == 0 {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::White;
                    } else {
                        square_color = ansi_term::Color::Green;
                    }
                } else if idx_c % 2 == 0 {
                    square_color = ansi_term::Color::Green;
                } else {
                    square_color = ansi_term::Color::White;
                }
                if square_contains_piece_square(
                    &chess_board,
                    SquareRank::from(rank),
                    SquareFile::from(file),
                ) {
                    let cur_piece = get_piece_at(
                        &chess_board,
                        SquareRank::from(rank),
                        SquareFile::from(file),
                    )
                    .expect("square_contains_piece_square to work as intended");
                    cur_pos.push((file, rank));
                    paint_piece(&cur_piece);
                } else {
                    print!("{}", square_color.paint("#"));
                }
            }
            println!();
        }
        print!("   "); // padding
        for (_, file) in (1..9).enumerate() {
            print!(
                "{}",
                ansi_term::Color::Purple.paint(SquareFile::from(file).to_str())
            );
        }
        println!();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    //TODO: verify
    fn board_as_string_initial() {
        let mut expected = String::new();
        let chess_board = Board::new();

        for (idx_r, rank) in (1..9).rev().enumerate() {
            //POV: Playing as white pieces
            for (idx_c, file) in (1..9).enumerate() {
                //let square = chess_board.get(idx_r).expect("idx_r to be in bounds").get(idx_c).expect("idx_c to be in bounds");

                let mut square_color = ansi_term::Color::White;
                if idx_r % 2 == 0 {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::White;
                    } else {
                        square_color = ansi_term::Color::Green;
                    }
                } else {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::Green;
                    } else {
                        square_color = ansi_term::Color::White;
                    }
                }
                if square_contains_piece_square(
                    &chess_board,
                    SquareRank::from(rank),
                    SquareFile::from(file),
                ) {
                    let cur_piece =
                        get_piece_at(&chess_board, SquareRank::from(rank), SquareFile::from(file))
                            .expect("square_contains_piece_square to work as intended");
                    expected.push_str(piece_string(&cur_piece).as_str());
                } else {
                    expected.push_str(square_color.paint("#").to_string().as_str());
                }
            }
            expected.push_str("");
        }
        assert_eq!(expected, chess_board.to_string())
    }

    #[test]
    fn pawn_test() {
        let mut chess_board = Board::new();
        let chess_board_clone = chess_board.clone();
        for piece in &mut chess_board.pieces {
            let mut moves = get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
            piece.possible_moves.append(&mut moves);
        }
        let expected_input = String::from("Pe2e4"); //TODO parse input string over hardcoding each
                                                    //char
        let src_file = 'e';
        let src_rank = '2';
        let dst_file = 'e';
        let dst_rank = '4';
        if let Ok(_) = chess_board.try_move_piece(src_rank, src_file, dst_rank, dst_file) {
            chess_board.advance_turn();
        }

        //TODO: Is there a better way?
        let mut test_path = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path.push("src");
        test_path.push("test_ins");
        test_path.push("test");
        test_path.set_extension("txt");
        let mut raw_expected =
            fs::read_to_string(test_path).unwrap_or("INVALID_TEST_INPUT".to_string());
        if raw_expected.ends_with('\n') {
            raw_expected.pop();
            if raw_expected.ends_with('\r') {
                raw_expected.pop();
            }
        }
        //let expected = raw_expected.replace(r"\\", r".");//.replace(r".",r"").replace("$",r"\"); //Goodness
        //why
        assert_eq!(raw_expected, chess_board.to_string());
    }
    #[test]
    fn ruy_lopez() {
        run_test_input("RuyLopez");
    }

    #[test]
    fn pawn_capture() {
        let mut chess_board = Board::new();
        let mut chess_board_clone = chess_board.clone();
        for piece in &mut chess_board.pieces {
            let mut moves = get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
            piece.possible_moves.append(&mut moves);
        }

        let mut test_path_in = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path_in.push("src");
        test_path_in.push("test_ins");
        test_path_in.push("Capture_pawn_in");
        test_path_in.set_extension("txt");
        let expected_input =
            fs::read_to_string(test_path_in).unwrap_or("INVALID_TEST_INPUT".to_string());
        for move_in in expected_input.lines() {
            if move_in.len() < 5 {
                break;
            }
            //TODO: Figure out why the first character is blank and everything is offset by 1
            let piece = move_in
                .chars()
                .nth(1)
                .expect("Each move to have exactly 6 chars");
            let src_file = move_in
                .chars()
                .nth(2)
                .expect("Each move to have exactly 6 chars");
            let src_rank = move_in
                .chars()
                .nth(3)
                .expect("Each move to have exactly 6 chars");
            let dst_file = move_in
                .chars()
                .nth(4)
                .expect("Each move to have exactly 6 chars");
            let dst_rank = move_in
                .chars()
                .nth(5)
                .expect("Each move to have exactly 6 chars");

            //print!("P: {} | sf: {} | sr: {} | df: {} | dr: {}",piece,src_file,src_rank,dst_file,dst_rank);
            print!(
                "P: {} | sf: {} | sr: {} | df: {} | dr: {}",
                piece, src_file, src_rank, dst_file, dst_rank
            );

            chess_board_clone = chess_board.clone();
            for piece in &mut chess_board.pieces {
                piece.possible_moves.clear();
                let mut moves = get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
                piece.possible_moves.append(&mut moves);
            }

            if let Ok(_) = chess_board.try_move_piece(src_rank, src_file, dst_rank, dst_file) {
                chess_board.advance_turn();
            }
        }

        let mut test_path_out = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path_out.push("src");
        test_path_out.push("test_ins");
        test_path_out.push("Capture_pawn_out");
        test_path_out.set_extension("txt");
        let expected_output = fs::read_to_string(test_path_out);

        let mut expected = expected_output.unwrap_or("INVALID_TEST_INPUT".to_string());
        if expected.ends_with('\n') {
            expected.pop();
            if expected.ends_with('\r') {
                expected.pop();
            }
        }


        assert_eq!(expected, chess_board.to_string());
    }

    fn run_test_input(test_name : &str){
        let mut chess_board = Board::new();
        let mut chess_board_clone = chess_board.clone();
        for piece in &mut chess_board.pieces {
            let mut moves = get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
            piece.possible_moves.append(&mut moves);
        }

        let mut test_path_in = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path_in.push("src");
        test_path_in.push("test_ins");
        test_path_in.push(test_name.to_string() + "_in");
        test_path_in.set_extension("txt");
        let expected_input =
            fs::read_to_string(test_path_in).unwrap_or("INVALID_TEST_INPUT".to_string());
        for move_in in expected_input.lines() {
            if move_in.len() < 5 {
                break;
            }
            //TODO: Figure out why the first character is blank and everything is offset by 1
            let piece = move_in
                .chars()
                .nth(1)
                .expect("Each move to have exactly 6 chars");
            let src_file = move_in
                .chars()
                .nth(2)
                .expect("Each move to have exactly 6 chars");
            let src_rank = move_in
                .chars()
                .nth(3)
                .expect("Each move to have exactly 6 chars");
            let dst_file = move_in
                .chars()
                .nth(4)
                .expect("Each move to have exactly 6 chars");
            let dst_rank = move_in
                .chars()
                .nth(5)
                .expect("Each move to have exactly 6 chars");

            //print!("P: {} | sf: {} | sr: {} | df: {} | dr: {}",piece,src_file,src_rank,dst_file,dst_rank);
            print!(
                "P: {} | sf: {} | sr: {} | df: {} | dr: {}",
                piece, src_file, src_rank, dst_file, dst_rank
            );

            chess_board_clone = chess_board.clone();
            for piece in &mut chess_board.pieces {
                piece.possible_moves.clear();
                let mut moves = get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
                piece.possible_moves.append(&mut moves);
            }

            if let Ok(_) = chess_board.try_move_piece(src_rank, src_file, dst_rank, dst_file) {
                chess_board.advance_turn();
            }
        }

        let mut test_path_out = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
        test_path_out.push("src");
        test_path_out.push("test_ins");
        test_path_out.push(test_name.to_string() + "_out");
        test_path_out.set_extension("txt");
        let expected_output = fs::read_to_string(test_path_out);

        let mut expected = expected_output.unwrap_or("INVALID_TEST_INPUT".to_string());
        if expected.ends_with('\n') {
            expected.pop();
            if expected.ends_with('\r') {
                expected.pop();
            }
        }


        assert_eq!(expected, chess_board.to_string());
    }

    #[test]
    fn invalid_queen_move() {
        run_test_input("QueenInvalid");
    }
}
