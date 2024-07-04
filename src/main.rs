use std::{ascii::AsciiExt, io, ops::Add, ops::Sub, env};


#[derive(Clone,Debug)]
enum PieceType{
    None,
    Pawn,
    Knight,
    Bishop,
    Rook  ,
    Queen ,
    King  ,
}

impl PieceType{
    fn to_str(&self) -> &str{
        match self{
            PieceType::None   => {"None"},
            PieceType::Pawn   => {"Pawn"},
            PieceType::Knight => {"Knight"},
            PieceType::Bishop => {"Bishop"},
            PieceType::Rook   => {"Rook"},
            PieceType::Queen  => {"Queen"},
            PieceType::King   => {"King"},
        }
    }
}

fn print_possible_moves(piece :&Piece){
    println!("The selected Piece is {}({}): dumping moves\n{{",

             piece.id,piece.piece_type.to_str());
    for possible_move in &piece.possible_moves{
        print!("{}{}, ",possible_move.file.to_str(),possible_move.rank.to_str());
    }
    println!("}};");

}


fn piece_string(piece : &Piece) -> String{
    match piece.piece_type{
                PieceType::Pawn  => {
                    match piece.color{
                        PieceColor::Black =>  ansi_term::Color::Green.paint("P").to_string(),
                        PieceColor::White =>  ansi_term::Color::White.paint("P").to_string(),
                    }
                }
                PieceType::Knight  => {
                    match piece.color{
                        PieceColor::Black =>  ansi_term::Color::Green.paint("N").to_string(),
                        PieceColor::White =>  ansi_term::Color::White.paint("N").to_string(),
                    }
                }
                PieceType::Bishop  => {
                    match piece.color{
                        PieceColor::Black =>  ansi_term::Color::Green.paint("I").to_string(),
                        PieceColor::White =>  ansi_term::Color::White.paint("I").to_string(),
                    }
                }
                PieceType::Rook  => {
                    match piece.color{
                        PieceColor::Black =>  ansi_term::Color::Green.paint("R").to_string(),
                        PieceColor::White =>  ansi_term::Color::White.paint("R").to_string(),
                    }
                }
                PieceType::Queen  => {
                    match piece.color{
                        PieceColor::Black =>  ansi_term::Color::Green.paint("Q").to_string(),
                        PieceColor::White =>  ansi_term::Color::White.paint("Q").to_string(),
                    }
                }
                PieceType::King  => {
                    match piece.color{
                        PieceColor::Black =>  ansi_term::Color::Green.paint("K").to_string(),
                        PieceColor::White =>  ansi_term::Color::White.paint("K").to_string(),
                    }
                }
                 _ => String::from(""), //TODO: What to do in this case. Will this ever happen???
            }
}

fn paint_piece(piece : &Piece){
    ////TODO: Verify this is correct and delete redundant code
    //below
    //print!("{}",piece_string(piece)); 
    match piece.piece_type{
                PieceType::Pawn  => {
                    match piece.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("P")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("P")),
                    }
                }
                PieceType::Knight  => {
                    match piece.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("N")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("N")),
                    }
                }
                PieceType::Bishop  => {
                    match piece.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("I")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("I")),
                    }
                }
                PieceType::Rook  => {
                    match piece.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("R")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("R")),
                    }
                }
                PieceType::Queen  => {
                    match piece.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("Q")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("Q")),
                    }
                }
                PieceType::King  => {
                    match piece.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("K")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("K")),
                    }
                }
                 _ => print!(""),
            }
}

#[derive(Clone,Debug,PartialEq)]
enum PieceColor{
    Black,
    White,
}


trait SquareTrait{
    fn from(val : u8) -> Self;
    fn from_char(val : char) -> Self;
    fn to_str(&self) -> &str;
    fn to_u8(&self) -> u8;
}


//TODO: Need to implement iterators or ranges in some way
#[derive(Clone,Debug,Eq,PartialEq, PartialOrd,Copy)]
enum SquareFile{
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Iterator for SquareFile{
    type Item = SquareFile;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.to_u8();
        if u >= Self::H.to_u8() {
            Some(Self::from(u+1))
        }else{
            None
        }
    }
        
}

impl Add<u8> for SquareFile{
    fn add(self, rhs: u8) -> Self{
        Self::from(self.to_u8() + rhs)
    }
    type Output = Self;
}

impl Sub<u8> for SquareFile{
    fn sub(self, rhs: u8) -> Self{
        Self::from(self.to_u8() - rhs)
    }
    type Output = Self;
}

impl SquareFile{
    fn from(val : u8) -> Self{
        match val{
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            //8 => Self::H, //TODO: This weird little hack...

            _ => unreachable!("{} exceeds File value!",val),
        }
    }
    fn from_char(val : char) -> Self{
        match val{
            'A' | 'a' => Self::A,
            'B' | 'b'=> Self::B,
            'C' | 'c'=> Self::C,
            'D' | 'd'=> Self::D,
            'E' | 'e'=> Self::E,
            'F' | 'f'=> Self::F,
            'G' | 'g'=> Self::G,
            'H' | 'h'=> Self::H,
            _ => unreachable!("{} exceeds File value!",val),
        }
    }
    fn to_str(&self) -> &str{
        match self {
            Self::A => "A" ,
            Self::B => "B" ,
            Self::C => "C" ,
            Self::D => "D" ,
            Self::E => "E" ,
            Self::F => "F" ,
            Self::G => "G" ,
            Self::H => "H" ,
        }
    }

    fn to_u8(&self) -> u8{
        match self {
            Self::A => 1 ,
            Self::B=> 2 ,
            Self::C=> 3 ,
            Self::D=> 4 ,
            Self::E=> 5 ,
            Self::F=> 6 ,
            Self::G=> 7 ,
            Self::H=> 8 ,
        }
    }
}

//TODO Need to implement iterators or ranges in some way
#[derive(Clone,Debug,PartialEq,PartialOrd,Copy)]
enum SquareRank{
    FIRST = 1,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    SIXTH,
    SEVENTH,
    EIGHTH,
}

impl Iterator for SquareRank{
    type Item = SquareRank;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.to_u8();
        if u >= Self::EIGHTH.to_u8() {
            Some(Self::from(u+1))
        }else{
            None
        }
    }
        
}


impl Add<u8> for SquareRank{
    fn add(self, rhs: u8) -> Self{
        Self::from(self.to_u8() + rhs)
    }
    type Output = Self;
}

impl Sub<u8> for SquareRank{
    fn sub(self, rhs: u8) -> Self{
        Self::from(self.to_u8() - rhs)
    }
    type Output = Self;
}

impl SquareRank{
   
   
    fn from(val : u8) -> Self{
        match val{
            1 => Self::FIRST,
            2 => Self::SECOND,
            3 => Self::THIRD,
            4 => Self::FOURTH,
            5 => Self::FIFTH,
            6 => Self::SIXTH,
            7 => Self::SEVENTH,
            8 => Self::EIGHTH,
            //8 => Self::EIGHTH, //TODO: This weird little hack

            _ => unreachable!("{} exceeds Rank value!",val),
        }
    }
    fn from_char(val : char) -> Self{
        match val{
            '1' => Self::FIRST,
            '2' => Self::SECOND,
            '3' => Self::THIRD,
            '4' => Self::FOURTH,
            '5' => Self::FIFTH,
            '6' => Self::SIXTH,
            '7' => Self::SEVENTH,
            '8' => Self::EIGHTH,
            _ => unreachable!("{} exceeds Rank value!",val),
        }
    }

    fn to_str(&self) -> &str{
        match self {
            Self::FIRST => "1" ,
            Self::SECOND => "2" ,
            Self::THIRD => "3" ,
            Self::FOURTH => "4" ,
            Self::FIFTH => "5" ,
            Self::SIXTH => "6" ,
            Self::SEVENTH => "7" ,
            Self::EIGHTH => "8" ,
        }
    }

    fn to_u8(&self) -> u8{
        match self {
            Self::FIRST => 1 ,
            Self::SECOND => 2 ,
            Self::THIRD => 3 ,
            Self::FOURTH => 4 ,
            Self::FIFTH => 5 ,
            Self::SIXTH => 6 ,
            Self::SEVENTH => 7 ,
            Self::EIGHTH => 8 ,
        }
    }
}

type SquareTuple = (SquareRank,SquareFile);

#[derive(Clone,Debug,Copy)]
struct Square{
    rank : SquareRank,
    file : SquareFile,
}

impl Square {

    fn from (tup: SquareTuple) -> Self{
        Self{rank:tup.0,file: tup.1}
    }

    fn as_tup(&self) -> SquareTuple{
        (self.rank,self.file)
    }

    fn new(rank : SquareRank, file : SquareFile) -> Self{
        Self{ rank ,  file }
    }

    fn eq(&self,other : &Self) -> bool{
        return (self.file ==  other.file) && (self.rank == other.rank);
    }

    fn all() -> Vec<Vec<Square>> {
        let mut ranks = vec![];
        for rank in 1..9{
            let mut squares = vec![];
            for file in 1..9{

                squares.push(Self::new(SquareRank::from(rank),SquareFile::from(file)));
            }
            ranks.push(squares);
        }
        ranks
    }
}


#[derive(Clone,Debug)]
enum TurnColor{
    WhitesTurn =0,
    BlacksTurn,
}

#[derive(Clone,Debug)]
struct Board {
    pieces : Vec<Piece>,
    squares : Vec<Vec<Square>>,
    turn : TurnColor,
    is_white_king_checked: bool,
    is_black_king_checked: bool,
}

fn get_piece_id_at(b: &Board, new_pos : &Square) -> i64{
    if let Some(piece)  = get_piece_at(b,new_pos.rank, new_pos.file){
        println!("Selected piece is {}",piece.piece_type.to_str());
        piece.id
    }else{
        -255 // ERROR value ?
    }
}

//Helper because borrank checker is really mad
//
fn get_valid_moves_for_piece(piece : &Piece, board: &Board) -> Vec<Square> {
    let mut valid_moves = vec![];
    //TODO: Captures are not considered neither are "blocks"
    match &piece.piece_type{
        PieceType::Pawn  => {
            let file = &piece.position.file;
            let rank = &piece.position.rank;
            match piece.color{
                //TODO: Remove all these clones....
                PieceColor::Black => {
                    if piece.position.rank == SquareRank::SEVENTH{ //Pawn has not moved
                        if !square_contains_piece_square(board,piece.position.rank-2,piece.position.file){
                            valid_moves.push(Square{rank: piece.position.rank-2,file:piece.position.file});
                        }
                    }

                    if !square_contains_piece_square(board,piece.position.rank-1,piece.position.file){
                        valid_moves.push(Square{rank: piece.position.rank-1,file:piece.position.file});
                    }

                    //Check for capturing pieces
                    //TODO: Add enpassant logic
                    if let Some(piece) = get_piece_at(board,piece.position.rank-1,piece.position.file+1){
                            if piece.color == PieceColor::White{
                                valid_moves.push(Square{rank: piece.position.rank-1,file:piece.position.file+1});
                            }
                    }
                    if let Some(piece) = get_piece_at(board,piece.position.rank-1,piece.position.file-1){
                        if piece.color == PieceColor::White{
                            valid_moves.push(Square{rank: piece.position.rank-1,file:piece.position.file+1});
                        }
                        valid_moves.push(Square{rank: piece.position.rank-1,file:piece.position.file-1});
                    }
                },
                PieceColor::White => {
                    if piece.position.rank == SquareRank::SECOND{ //Pawn has not moved
                        if !square_contains_piece_square(board,piece.position.rank+2,piece.position.file){
                            valid_moves.push(Square{rank: piece.position.rank+2,file:piece.position.file});
                        }
                    }

                    if !square_contains_piece_square(board,piece.position.rank+1,piece.position.file){
                        valid_moves.push(Square{rank: piece.position.rank+1,file:piece.position.file});
                    }

                    //Check for capturing pieces
                    //TODO: Add enpassant logic
                    if let Some(piece) = get_piece_at(board,piece.position.rank+1,piece.position.file+1){
                            if piece.color == PieceColor::White{
                                valid_moves.push(Square{rank: piece.position.rank+1,file:piece.position.file+1});
                            }
                    }
                    if let Some(piece) = get_piece_at(board,piece.position.rank+1,piece.position.file-1){
                        if piece.color == PieceColor::White{
                            valid_moves.push(Square{rank: piece.position.rank+1,file:piece.position.file+1});
                        }
                        valid_moves.push(Square{rank: piece.position.rank+1,file:piece.position.file-1});
                    }
                },
            }
        },
        PieceType::Knight  => {
            let file = piece.position.file;
            let rank = piece.position.rank;

            println!("Knight Starting point {}{}",file.to_str(),rank.to_str());

            //TODO: Some of these are out of bounds
            if rank <= SquareRank::SIXTH && file <= SquareFile::G{
                if let Some(ref mut other) = get_piece_at(board, rank + 2, file + 1)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file + 1, rank: rank + 2});
                    }
                }else{
                    //square is empty
                    valid_moves.push(Square{file: file + 1, rank: rank + 2});
                }
            }

            if rank >= SquareRank::THIRD && file <= SquareFile::G {
                if let Some(ref mut other) = get_piece_at(board, rank-2, file+1)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file + 1, rank: rank - 2});
                    }
                }else{
                    valid_moves.push(Square{file: file + 1, rank: rank - 2});
                }
            }

            if file >= SquareFile::B && rank <= SquareRank::SIXTH{
                if let Some(ref mut other) = get_piece_at(board, rank+2, file-1)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file - 1, rank: rank + 2});
                    }
                }else{
                    valid_moves.push(Square{file: file - 1, rank: rank + 2});
                }
            }

            if file >= SquareFile::A && rank>= SquareRank::THIRD {
                if let Some(ref mut other) = get_piece_at(board, rank-2, file-1)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file - 1, rank: rank - 2});
                    }
                }else{
                    valid_moves.push(Square{file: file -1 , rank: rank - 2});
                }
            }

            if file >= SquareFile::C && rank <= SquareRank::SIXTH{
                if let Some(ref mut other) = get_piece_at(board, rank+1, file-2)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file -2, rank: rank +1});
                    }
                }else{
                    valid_moves.push(Square{file: file - 2, rank: rank + 1});
                }
            }

            if file >= SquareFile::C && rank   >= SquareRank::SECOND {
                if let Some(ref mut other) = get_piece_at(board, rank-2, file-2)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file -2 , rank: rank -1 });
                    }
                }else{
                    valid_moves.push(Square{file: file - 2 , rank: rank - 1});
                }
            }

            if file <= SquareFile::F && rank <= SquareRank::SEVENTH{
                if let Some(ref mut other) = get_piece_at(board, rank+1, file+2)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file + 2, rank: rank + 1});
                    }
                }else{
                    valid_moves.push(Square{file: file + 2, rank: rank + 1});
                }
            }

            if rank >= SquareRank::SECOND && file <= SquareFile::F{
                if let Some(ref mut other) = get_piece_at(board, rank-1, file+2)  {
                    if other.color != piece.color{
                        //only add a valid move if the piece in the target square is different clor
                        //TODO: Add code to make a piece targeted/pinned here
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file + 2, rank: rank -1 });
                    }
                }else{
                    valid_moves.push(Square{file: file + 2, rank: rank - 1});
                }
            }
        },

        PieceType::Bishop  => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;
            println!("Bishop Starting point {}{}",cur_file.to_str(),cur_rank.to_str());

            //TODO: Some of these might be out of bounds. Verify
            //for each diagonal
                let mut rank = cur_rank.to_u8() ;
                let mut file = cur_file.to_u8() ;
            loop {
                if rank >= SquareRank::EIGHTH as u8   || file >= SquareFile::H.to_u8()  {
                    break;

                }
                
                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                rank = rank + 1 ;
                file = file + 1 ;
                println!("B1: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
            }

                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {
                if rank >= SquareRank::EIGHTH as u8  || file == SquareFile::A.to_u8()  {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                rank = rank + 1;
                file = file - 1;

                println!("B2: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
            }

                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {
                if rank == SquareRank::FIRST as u8 || file >= SquareFile::H.to_u8() {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                rank = rank - 1;
                file = file + 1;

                println!("B3: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
            }
           
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {
                if rank == SquareRank::FIRST as u8  || file == SquareFile::A.to_u8() {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                rank = rank - 1;
                file = file - 1;

                println!("B4: adding {}{}",SquareFile::from(file).to_str(),SquareRank::from(rank).to_str());
                valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
            }
           
        },
        PieceType::Rook  => {
            let cur_file = piece.position.file ;
            let cur_rank = piece.position.rank ;
            let mut rank = cur_rank.to_u8();
            let mut file = cur_file.to_u8();

            println!("Rook Starting point {}{}",cur_file.to_str(),cur_rank.to_str());
           

            loop {
                if file == SquareFile::A.to_u8()  || rank == SquareRank::FIRST as u8{
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                file = file - 1 as u8;
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {

                if rank == SquareRank::FIRST as u8 || file == SquareFile::A.to_u8(){
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                rank = rank - 1 as u8;
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {
                file = file + 1 as u8;
                if file >= SquareFile::H.to_u8()  || rank >= SquareRank::EIGHTH  as u8{
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
            rank = cur_rank.to_u8();
            file = cur_file.to_u8();
            loop {

                rank = rank + 1 as u8;
                //TODO: what is going on here
                if rank >= 7  || file >= 7 {
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
           
        },
        PieceType::Queen  => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;

            println!("Queen Starting point {}{}",cur_file.to_str(),cur_rank.to_str());
            //TODO: Some of these might be out of bounds. Verify
            //for each diagonal
                let mut rank = cur_rank.to_u8() ;
                let mut file = cur_file.to_u8() ;
            loop {
                rank = rank + 1 as u8;
                file = file + 1 as u8;
                if rank > SquareRank::EIGHTH  as  u8 || file > SquareFile::H.to_u8()  {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();

            loop {
                rank = rank + 1 as u8;
                file = file - 1 as u8;
                if rank > SquareRank::EIGHTH  as u8 || file == SquareFile::A.to_u8()  {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();

            loop {
                rank = rank - 1 ;
                file = file + 1 ;
                if rank <= SquareRank::FIRST  as u8 || file >= SquareFile::H.to_u8()  {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
           
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {
                rank = rank - 1 ;
                file = file - 1 ;
                if rank <= SquareRank::FIRST  as u8 || file == SquareFile::A.to_u8()  {
                    break;

                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }

                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {

                file = file - 1 ;
                if file <= SquareFile::A.to_u8()  {
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {

                rank = rank - 1 ;
                if rank <= SquareRank::FIRST  as u8 {
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {

                file = file + 1 ;
                if file > SquareFile::H.to_u8()  {
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }
                rank = cur_rank.to_u8();
                file = cur_file.to_u8();
            loop {

                rank = rank + 1 as u8;
                if rank > SquareRank::EIGHTH  as u8 {
                    break;
                }

                if let Some(ref mut other) = get_piece_at(board, SquareRank::from(rank), SquareFile::from(file)){
                    if other.color != piece.color{
                        other.set_targeted(true); 
                        valid_moves.push(Square{file: SquareFile::from(file) ,rank: SquareRank::from(rank)});
                        break;
                    }
                }
                valid_moves.push(Square{file: SquareFile::from(file),rank: SquareRank::from(rank)});
            }

        },
        PieceType::King  => {
            let file = piece.position.file;
            let rank = piece.position.rank;

            println!("King Starting point {}{}",file.to_str(),rank.to_str());

            //TODO: Some of these moves are invalid
            if file <= SquareFile::G && rank <= SquareRank::SEVENTH{
                if let Some(ref mut other) = get_piece_at(board, rank+1, file+1){
                    if other.color != piece.color{
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file + 1 , rank: rank + 1});
                    }
                }else{
                    valid_moves.push(Square{file: file + 1 , rank: rank + 1});
                }
            }

            if file <= SquareFile::G {
                if let Some(ref mut other) = get_piece_at(board, rank, file+1){
                    if other.color != piece.color{
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file + 1 , rank});
                    }
                }else{
                    valid_moves.push(Square{file: file + 1 ,rank});
                }
            }

            if rank <= SquareRank::SEVENTH{
                if let Some(ref mut other) = get_piece_at(board, rank + 1, file){
                    if other.color != piece.color{
                        other.set_targeted(true);
                        valid_moves.push(Square{file, rank: rank + 1 });
                    }
                }else{
                    valid_moves.push(Square{file,rank : rank + 1});
                }
            }

            if rank >= SquareRank::SECOND{
                if file <= SquareFile::G {
                    if let Some(ref mut other) = get_piece_at(board, rank - 1, file+1){
                        if other.color != piece.color{
                            other.set_targeted(true);
                            valid_moves.push(Square{file: file + 1 , rank: rank - 1});
                        }
                    }else{
                        valid_moves.push(Square{file: file + 1 , rank: rank - 1});
                    }
                //valid_moves.push(Square{file: file,rank : rank - 1});
                }
            }
            if file >= SquareFile::A{
                if rank <= SquareRank::SEVENTH{
                    if let Some(ref mut other) = get_piece_at(board, rank + 1, file+1){
                        if other.color != piece.color{
                            other.set_targeted(true);
                            valid_moves.push(Square{file: file - 1 , rank: rank + 1 });
                        }
                    }else{
                        valid_moves.push(Square{file: file - 1 , rank: rank + 1});
                    }
                }
                //valid_moves.push(Square{file: file - 1 ,rank: rank});
            }

            if rank > SquareRank::SECOND && file >= SquareFile::A{
                if let Some(ref mut other) = get_piece_at(board, rank - 1, file-1){
                    if other.color != piece.color{
                        other.set_targeted(true);
                        valid_moves.push(Square{file: file - 1 , rank: rank - 1 });
                    }
                }else{
                    valid_moves.push(Square{file: file - 1 , rank: rank - 1});
                }
            }
        },
        _ => todo!("implement possible moves for {:?}", piece.piece_type),

    }

    valid_moves
}
    fn square_contains_piece_square(b : &Board, rank : SquareRank, file : SquareFile) -> bool{
        for piece in b.pieces.as_slice(){
            match piece.piece_type {
                _ => {/* do noething*/}
            }
            if piece.position.rank == rank && piece.position.file == file{
                return true;
            }
        }
        false
    }


    fn get_piece_at(b : &Board, rank : SquareRank, file :SquareFile) -> Option<Piece>{
        for piece in b.pieces.as_slice(){
            if piece.position.rank == rank && piece.position.file == file{
                return Some(piece.clone());
            }
        }
        None
    }

    //Should this rturn bool or Result or something?
fn move_piece(b: &mut Board, piece_id : i64, new_pos : &Square){
    let mut selected_piece : Option<&mut Piece> = None;
    for piece in &mut b.pieces{
        if piece.id == piece_id{
            selected_piece = Some(piece);
            break; //TODO: Investigate why we fall in this loop more than one...
        }
    }
   
    let mut next_move : Option<&Square> = None;
    match selected_piece {
        Some(ref mut piece) => {
            let mut found_move = false;
            for valid_move in piece.possible_moves.as_slice(){
                if valid_move.eq(new_pos) {
                    //piece.update_pos(new_pos); //<- We cant borrank from piece again as it is
                                                //borranked in the loop. defer position update
                    next_move = Some(new_pos);
                    piece.position.rank = new_pos.rank;
                    piece.position.file = new_pos.file;

                    found_move = true;

                }
            }

            if !found_move{
                println!("selected move not in list of valid moves for selected piece");
                print_possible_moves(piece);
            }

            // reclaculate moves here. Calling another self, mut function is not working and this
            // is probably the only place where it is needed
            //piece.possible_moves = vec![Square{rank : (piece.position.rank + 1)%8,file : (piece.position.file + 1)%8 }];
           
        },
        None => {
            println!("Could not resolve Piece_id {}",piece_id);
            //return;
        }
    }

    match next_move {
        Some(_move_) => {
            //todo!("recalculate_valid_moves is not implemented yet");
        },
        None => {
            println!("Illegal move");
            return;
        }
    }

                //for valid_move in &mut piece.possible_moves{
                //    //if we have a valid move. move the piece
                //    if valid_move.eq(new_pos){
                //        piece.update_pos(new_pos);
                //        self.recalculate_valid_moves(piece);
                //        return; // break for performance
                //    }
                //}
    }

impl Board{
 
    fn new() -> Self{
        Self{
            pieces : Piece::all(),//vec![Piece::new();16],
            squares : Square::all(),
            turn : TurnColor::WhitesTurn,
            is_white_king_checked: false,
            is_black_king_checked: false,
        }
    }
   
    //TODO: Here we might want to add notation, switch clocks, or some other events that may happen
    //at the end of a turn
    fn advance_turn(&mut self){
        match self.turn{
            TurnColor::WhitesTurn => {self.turn = TurnColor::BlacksTurn},
            TurnColor::BlacksTurn => {self.turn = TurnColor::WhitesTurn},
        }
    }

    fn print_pieces(&self){
        for piece in &self.pieces{
            println!("({}): {}, [{}{}]",piece.id, piece.piece_type.to_str(), piece.position.file.to_str(), piece.position.rank.to_str());
        }
    }

    fn to_string(&self) -> String {

        let mut BoardString = String::new();

        for (idx_r,rank) in (1..9).rev().enumerate(){ //POV: Playing as white pieces
                                                      //for (idx_r,rank) in (1..9).enumerate(){ //POV: Playing as black pieces
            for (idx_c,file) in (1..9).enumerate(){
                //let square = chess_board.get(idx_r).expect("idx_r to be in bounds").get(idx_c).expect("idx_c to be in bounds");

                let mut square_color = ansi_term::Color::White;
                if idx_r % 2 == 0 {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::White;
                    }else{
                        square_color = ansi_term::Color::Green;
                    }
                }else{
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::Green;
                    }else{
                        square_color = ansi_term::Color::White;
                    }
                }
                if square_contains_piece_square(self, SquareRank::from(rank), SquareFile::from(file)){

                    let cur_piece = get_piece_at(self,SquareRank::from(rank), SquareFile::from(file)).expect("square_contains_piece_square to work as intended");
                    BoardString.push_str(piece_string(&cur_piece).as_str());
                }else{
                    BoardString.push_str(square_color.paint("#").to_string().as_str())
                }
            }
            println!("");

        }

        BoardString

        }

}

#[derive(Clone,Debug)]
struct Piece{
    id : i64, // For ease of search and Id
    position : Square,
    threatened : bool,
    possible_moves : Vec<Square>,
    color : PieceColor,
    piece_type : PieceType,
}

//TODO: tHis will be our global UUID for pieces methinks
//static CUR_PIECE_NUM = AtomicU8::new(0);

impl Piece{
    fn default() -> Self{
        Self{
            id : 0, //TODO: should we track this better, probably..
            position : Square {
                file : SquareFile::from(0),
                rank : SquareRank::from(0),
            },
            possible_moves : vec![],
            threatened : false,
            color : PieceColor::Black,
            piece_type : PieceType::None,
        }
    }

    fn new(rank : SquareRank, file : SquareFile, _color : PieceColor, _type : PieceType) -> Self{
        //CUR_PIECE_NUM.fetch_add;
        Self{
            position : Square::new(rank,file),
            color : _color,
            possible_moves : vec![],
            threatened : false,
            id : ((rank.to_u8())as i64 *100) + (file.to_u8() as i64), //TODO: Implement global UUID
            piece_type : _type,
        }
    }

    fn all() -> Vec<Self> {
        let mut pieces = vec![];
            for file in 1..9{

                pieces.push(Self::new(SquareRank::SECOND,SquareFile::from(file),PieceColor::White,PieceType::Pawn));
                pieces.push(Self::new(SquareRank::SEVENTH,SquareFile::from(file),PieceColor::Black,PieceType::Pawn));
            }
            pieces.push(Self::new(SquareRank::FIRST,SquareFile::A,PieceColor::White,PieceType::Rook));
            pieces.push(Self::new(SquareRank::FIRST,SquareFile::H,PieceColor::White,PieceType::Rook));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::A,PieceColor::Black,PieceType::Rook));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::H,PieceColor::Black,PieceType::Rook));

            pieces.push(Self::new(SquareRank::FIRST,SquareFile::B,PieceColor::White,PieceType::Knight));
            pieces.push(Self::new(SquareRank::FIRST,SquareFile::G,PieceColor::White,PieceType::Knight));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::B,PieceColor::Black,PieceType::Knight));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::G,PieceColor::Black,PieceType::Knight));

            pieces.push(Self::new(SquareRank::FIRST,SquareFile::C,PieceColor::White,PieceType::Bishop));
            pieces.push(Self::new(SquareRank::FIRST,SquareFile::F,PieceColor::White,PieceType::Bishop));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::C,PieceColor::Black,PieceType::Bishop));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::F,PieceColor::Black,PieceType::Bishop));

            pieces.push(Self::new(SquareRank::FIRST,SquareFile::E,PieceColor::White,PieceType::King));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::E,PieceColor::Black,PieceType::King));

            pieces.push(Self::new(SquareRank::FIRST,SquareFile::D,PieceColor::White,PieceType::Queen));
            pieces.push(Self::new(SquareRank::EIGHTH,SquareFile::D,PieceColor::Black,PieceType::Queen));
        pieces
    }
   
    //NOTE: Board is in charge of validating the move, by contract,
    //      we assume that at this point the move is valid
    //      despite Pieces owning their possible valid moves
    fn update_pos(&mut self, new_pos : &Square){
            self.position.file = new_pos.file;
            self.position.rank = new_pos.rank;
    }
    fn set_targeted(&mut self, is_targeted: bool){
        self.threatened = is_targeted;
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut chess_board = Board::new();
    //init board
    //TODO: I;m sure there is a better way to do this, doing it the quick and dirty way for now
    let chess_board_clone = chess_board.clone();
    for piece in &mut chess_board.pieces{
        let mut moves = get_valid_moves_for_piece(&piece.clone(),&chess_board_clone);
        piece.possible_moves.append(&mut moves);
    }
   
    'game : loop{
    //Draw board. Refactor?
    let mut cur_pos : Vec<(u8,u8)> = vec![]; 
    for (idx_r,rank) in (1..9).rev().enumerate(){ //POV: Playing as white pieces
    //for (idx_r,rank) in (1..9).enumerate(){ //POV: Playing as black pieces
        print!("{}. ", ansi_term::Color::Purple.paint((rank).to_string()));
        for (idx_c,file) in (1..9).enumerate(){
            //let square = chess_board.get(idx_r).expect("idx_r to be in bounds").get(idx_c).expect("idx_c to be in bounds");
           
            let mut square_color = ansi_term::Color::White;
            if idx_r % 2 == 0 {
                if idx_c % 2 == 0 {
                    square_color = ansi_term::Color::White;
                }else{
                    square_color = ansi_term::Color::Green;
                }
            }else{
                if idx_c % 2 == 0 {
                    square_color = ansi_term::Color::Green;
                }else{
                    square_color = ansi_term::Color::White;
                }
            }
            if square_contains_piece_square(&chess_board, SquareRank::from(rank), SquareFile::from(file)){
               
                let cur_piece = get_piece_at(&chess_board,SquareRank::from(rank), SquareFile::from(file)).expect("square_contains_piece_square to work as intended");
                cur_pos.push((file,rank));
                paint_piece(&cur_piece);
            }else{
                print!("{}",square_color.paint("#"));
            }
        }
            println!("");

        }
            print!("   "); // padding
            for (_,file) in (1..9).enumerate(){
                print!("{}", ansi_term::Color::Purple.paint(SquareFile::from(file).to_str()));
            }
            println!("");
            //chess_board.print_pieces();

            let mut input_buffer = String::new();
            let result = io::stdin().read_line(&mut input_buffer);
            if !result.is_ok() {
                unreachable!("We have to get something from stdin...");
            }
            if input_buffer == String::from("END\n"){
                break 'game;
            }
           
            //if a move is being input
            if input_buffer.starts_with("P") ||
                input_buffer.starts_with("N") ||
                input_buffer.starts_with("I") ||
                input_buffer.starts_with("R") ||
                input_buffer.starts_with("Q") ||
                input_buffer.starts_with("K") {
                    if input_buffer.len() >= 5{
                        //invalid input. valid input e.g. 'Pe4e5g,
                        let source_square = input_buffer.get(1..3).expect("input buffer to havee 1..3");
                        let dest_square =  input_buffer.get(3..5).expect("input buffer to have 3..5");
                       
                       
                        let src_file = source_square.chars().nth(0).expect("source square to have .nth(1)");
                        let src_rank = source_square.chars().nth(1).expect("source square to have .nth(2)");

                        let dst_file = dest_square.chars().nth(0).expect("dest square to have .nth(3)");
                        let dst_rank = dest_square.chars().nth(1).expect("des square to have .nth(4)");
                                             
                       
                        if square_contains_piece_square(
                            &chess_board,
                           SquareRank::from_char(src_rank)  ,
                           SquareFile::from_char(src_file)  //
                            ){

                            //println!("You have selected a valid piece");
                            let piece_id = get_piece_id_at(
                                &chess_board,
                                &Square{
                                    rank: SquareRank::from_char(src_rank)  ,
                                    file: SquareFile::from_char(src_file)  //
                                });
                                                                   
                            move_piece(&mut chess_board, piece_id,
                                &Square{file : SquareFile::from_char(dst_file)  ,
                                       rank: SquareRank::from_char(dst_rank)    
                                       });
                            //
                            //if we successfully move a piece then we tick a turn
                            chess_board.advance_turn();

                        }else{
                            println!("You selected an empty square, plese select a square with a piece in it");
                        }
                    }else{
                        println!("invalid input lneght {}",input_buffer.len());
                    }

            }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    //TODO: verify
    fn board_as_string_initial(){
        let mut expected = String::new();
        let chess_board = Board::new();
        for (idx_r,rank) in (1..9).rev().enumerate(){ //POV: Playing as white pieces
            for (idx_c,file) in (1..9).enumerate(){
                //let square = chess_board.get(idx_r).expect("idx_r to be in bounds").get(idx_c).expect("idx_c to be in bounds");

                let mut square_color = ansi_term::Color::White;
                if idx_r % 2 == 0 {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::White;
                    }else{
                        square_color = ansi_term::Color::Green;
                    }
                }else{
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::Green;
                    }else{
                        square_color = ansi_term::Color::White;
                    }
                }
                if square_contains_piece_square(&chess_board, SquareRank::from(rank), SquareFile::from(file)){

                    let cur_piece = get_piece_at(&chess_board,SquareRank::from(rank), SquareFile::from(file)).expect("square_contains_piece_square to work as intended");
                    expected.push_str(piece_string(&cur_piece).as_str());
                }else{
                    expected.push_str(square_color.paint("#").to_string().as_str());
                }
            }
            expected.push_str("");

        }
            assert_eq!(expected,chess_board.to_string())
        }
}
