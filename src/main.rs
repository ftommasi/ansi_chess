use std::{ascii::AsciiExt, io};

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
    println!("The selected Piece is {}({}): \n{{",
             piece.id,piece.piece_type.to_str());
    for possible_move in &piece.possible_moves{
        print!("{}{}, ",SquareFile::from(possible_move.file).to_str(),SquareRank::from(possible_move.rank).to_str());
    }
    println!("}};");
}

fn paint_piece(piece : &Piece){
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

#[derive(Clone,Debug)]
enum PieceColor{
    Black,
    White,
}

enum SquareFile{
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl SquareFile{
    fn from(val : u8) -> Self{
        match val{
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
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
}

enum SquareRank{
    FIRST = 0,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    SIXTH,
    SEVENTH,
    EIGHTH,
}

impl SquareRank{
    fn from(val : u8) -> Self{
        match val{
            0 => Self::FIRST,
            1 => Self::SECOND,
            2 => Self::THIRD,
            3 => Self::FOURTH,
            4 => Self::FIFTH,
            5 => Self::SIXTH,
            6 => Self::SEVENTH,
            7 => Self::EIGHTH,
            _ => unreachable!("{} exceeds Rank value!",val),
        }
    }
    fn from_char(val : char) -> Self{
        match val{
            '0' => Self::FIRST,
            '1' => Self::SECOND,
            '2' => Self::THIRD,
            '3' => Self::FOURTH,
            '4' => Self::FIFTH,
            '5' => Self::SIXTH,
            '6' => Self::SEVENTH,
            '7' => Self::EIGHTH,
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
}


#[derive(Clone,Debug)]
struct Square{
    rank : u8,
    file : u8,
}

impl Square {

    fn new(rank : SquareRank, file : SquareFile) -> Self{
        Self{rank : rank as u8, file: file as u8}
    }

    fn eq(&self,other : &Self) -> bool{
        return self.file == other.file && self.rank == other.rank;
    }

    fn all() -> Vec<Vec<Square>> {
        let mut ranks = vec![];
        for rank in 0..8{
            let mut squares = vec![];
            for file in 0..8{
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
}

fn get_piece_id_at(b: &Board, new_pos : &Square) -> u8{
    if let Some(piece)  = get_piece_at(b,new_pos.rank, new_pos.file){
        piece.id
    }else{
        255 // ERROR value ?
    }
}

//Helper because borrank checker is really mad
//
fn get_valid_moves_for_piece(piece : &Piece, board: &Board) -> Vec<Square> {
    let mut valid_moves = vec![];
    //TODO: Captures are not considered neither are "blocks"
    match piece.piece_type{
        PieceType::Pawn  => {
            match piece.color{
                PieceColor::Black => {
                    if piece.position.rank == 6{ //Pawn has not moved
                        if !square_contains_piece_square(board,piece.position.rank-2,piece.position.file){
                            valid_moves.push(Square{rank:piece.position.rank-2,file:piece.position.file});
                        }
                    }

                    if !square_contains_piece_square(board,piece.position.rank-1,piece.position.file){
                        valid_moves.push(Square{rank:piece.position.rank-1,file:piece.position.file});
                    }
                },
                PieceColor::White => {
                    if piece.position.rank == 1{ //Pawn has not moved
                        if !square_contains_piece_square(board,piece.position.rank+2,piece.position.file){
                            valid_moves.push(Square{rank:piece.position.rank+2,file:piece.position.file});
                        }
                    }

                    if !square_contains_piece_square(board,piece.position.rank+1,piece.position.file){
                        valid_moves.push(Square{rank:piece.position.rank+1,file:piece.position.file});
                    }
                },
            }
        },
        PieceType::Knight  => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;

            //TODO: Some of these are out of bounds
            valid_moves.push(Square{file: cur_file + 1, rank: cur_rank + 2});

            if cur_rank >= 2 {
                valid_moves.push(Square{file: cur_file + 1, rank: cur_rank - 2});
            }


            if cur_file >= 1 {
                valid_moves.push(Square{file: cur_file - 1, rank: cur_rank + 2});
            }

            if cur_file >= 1 && cur_rank >= 2 {
                valid_moves.push(Square{file: cur_file - 1, rank: cur_rank - 2});
            }

            if cur_file >= 2{
                valid_moves.push(Square{file: cur_file - 2, rank: cur_rank + 1});
            }

            if cur_file >= 2 && cur_rank >= 1 {
                valid_moves.push(Square{file: cur_file - 2, rank: cur_rank - 1});
            }

            valid_moves.push(Square{file: cur_file + 2, rank: cur_rank + 1});
            if cur_rank >= 1{
                valid_moves.push(Square{file: cur_file + 2, rank: cur_rank - 1});
            }
        },

        PieceType::Bishop  => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;

            //TODO: Some of these might be out of bounds. Verify
            //for each diagonal
                let mut rank = cur_rank;
                let mut file = cur_file;
            loop { 
                if rank > SquareRank::EIGHTH as u8 || file > SquareFile::H as u8 {
                    break;

                }
                rank = rank + 1;
                file = file + 1;

                valid_moves.push(Square{file ,rank});
            }

                rank = cur_rank;
                file = cur_file;
            loop { 
                if rank > SquareRank::EIGHTH as u8 || file == SquareFile::A as u8 {
                    break;

                }
                rank = rank + 1;
                file = file - 1;

                valid_moves.push(Square{file ,rank});
            }

                rank = cur_rank;
                file = cur_file;
            loop { 
                if rank == SquareRank::FIRST as u8 || file > SquareFile::H as u8 {
                    break;

                }
                rank = rank - 1;
                file = file + 1;

                valid_moves.push(Square{file ,rank});
            }
           
                rank = cur_rank;
                file = cur_file;
            loop { 
                if rank == SquareRank::FIRST as u8 || file == SquareFile::A as u8 {
                    break;

                }
                rank = rank - 1;
                file = file - 1;

                valid_moves.push(Square{file ,rank});
            }

            
        },
        PieceType::Rook  => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;
            let mut rank = cur_rank;
            let mut file = cur_file;
            loop { 
                if file == SquareFile::A as u8 {
                    break;
                }
                file = file - 1;
                valid_moves.push(Square{file,rank});
            }
            rank = cur_rank;
            file = cur_file;
            loop { 

                if rank == SquareRank::FIRST as u8 {
                    break;
                }
                rank = rank - 1;
                valid_moves.push(Square{file,rank});
            }
            rank = cur_rank;
            file = cur_file;
            loop { 
                if file > SquareFile::H as u8 {
                    break;
                }
                file = file + 1;
                valid_moves.push(Square{file,rank});
            }
            rank = cur_rank;
            file = cur_file;
            loop { 

                if rank > SquareRank::EIGHTH as u8 {
                    break;
                }
                rank = rank + 1;
                valid_moves.push(Square{file,rank});
            }
            
        },
        PieceType::Queen  => {
            let cur_file = piece.position.file;
            let cur_rank = piece.position.rank;
            //TODO: Some of these might be out of bounds. Verify
            //for each diagonal
                let mut rank = cur_rank;
                let mut file = cur_file;
            loop { 
                if rank > SquareRank::EIGHTH as u8 || file > SquareFile::H as u8 {
                    break;

                }
                rank = rank + 1;
                file = file + 1;

                valid_moves.push(Square{file ,rank});
            }
                rank = cur_rank;
                file = cur_file;

            loop { 
                if rank > SquareRank::EIGHTH as u8 || file == SquareFile::A as u8 {
                    break;

                }
                rank = rank + 1;
                file = file - 1;

                valid_moves.push(Square{file ,rank});
            }
                rank = cur_rank;
                file = cur_file;

            loop { 
                if rank == SquareRank::FIRST as u8 || file > SquareFile::H as u8 {
                    break;

                }
                rank = rank - 1;
                file = file + 1;

                valid_moves.push(Square{file ,rank});
            }
           
                rank = cur_rank;
                file = cur_file;
            loop { 
                if rank == SquareRank::FIRST as u8 || file == SquareFile::A as u8 {
                    break;

                }
                rank = rank - 1;
                file = file - 1;

                valid_moves.push(Square{file ,rank});
            }
                rank = cur_rank;
                file = cur_file;
            loop { 

                if file == SquareFile::A as u8 {
                    break;
                }
                file = file - 1;
                valid_moves.push(Square{file,rank});
            }
                rank = cur_rank;
                file = cur_file;
            loop { 

                if rank == SquareRank::FIRST as u8 {
                    break;
                }
                rank = rank - 1;
                valid_moves.push(Square{file,rank});
            }
                rank = cur_rank;
                file = cur_file;
            loop { 

                if file > SquareFile::H as u8 {
                    break;
                }
                file = file + 1;
                valid_moves.push(Square{file,rank});
            }
                rank = cur_rank;
                file = cur_file;
            loop { 

                if rank > SquareRank::EIGHTH as u8 {
                    break;
                }
                rank = rank + 1;
                valid_moves.push(Square{file,rank});
            }

        },
        PieceType::King  => {
            let file = piece.position.file;
            let rank = piece.position.rank;

            //TODO: Some of these moves are invalid
            valid_moves.push(Square{file: file + 1 , rank: rank + 1});
            valid_moves.push(Square{file: file + 1 ,rank});
            valid_moves.push(Square{file ,rank : rank + 1});

            if rank >= 1{
                valid_moves.push(Square{file: file + 1 , rank: rank - 1});
                valid_moves.push(Square{file ,rank : rank - 1});
            }
            if file >= 1{
                valid_moves.push(Square{file: file - 1 , rank: rank + 1});
                valid_moves.push(Square{file: file - 1 ,rank});
            }

            if rank >1 && file >= 1{
                valid_moves.push(Square{file: file - 1 , rank: rank - 1});
            }
        },
        _ => todo!("implement possible moves for {:?}", piece.piece_type),

    }

    valid_moves
}
    fn square_contains_piece_square(b : &Board, rarnk : u8, file : u8) -> bool{
        for piece in b.pieces.as_slice(){
            match piece.piece_type {
                _ => {/* do noething*/}
            }
            if piece.position.rank == rarnk && piece.position.file == file{
                return true;
            }
        }
        false
    }


    fn get_piece_at(b : &Board, rarnk : u8, file : u8) -> Option<Piece>{
        for piece in b.pieces.as_slice(){
            if piece.position.rank == rarnk && piece.position.file == file{
                return Some(piece.clone());
            }
        }
        None
    }

    //Should this rturn bool or Result or something?
fn move_piece(b: &mut Board, piece_id : u8, new_pos : &Square){
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
                println!("Here are the valid moves for {:?}:\n {:?}", 
                         piece, 
                         piece.possible_moves.as_slice());
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
            let piece =  selected_piece.unwrap();

            match piece.piece_type{
                PieceType::Pawn  => {
                    match piece.color{
                        PieceColor::Black => {
                        },
                        PieceColor::White => {
                        },
                    }
                }
                PieceType::Knight  => {
                    match piece.color{
                        PieceColor::Black => {},
                        PieceColor::White => {},
                    }
                }
                PieceType::Bishop  => {
                    match piece.color{
                        PieceColor::Black => {},
                        PieceColor::White => {},
                    }
                }
                PieceType::Rook  => {
                    match piece.color{
                        PieceColor::Black => {},
                        PieceColor::White => {},
                    }
                }
                PieceType::Queen  => {
                    match piece.color{
                        PieceColor::Black => {},
                        PieceColor::White => {},
                    }
                }
                PieceType::King  => {
                    match piece.color{
                        PieceColor::Black => {},
                        PieceColor::White => {},
                    }
                }
                 _ => print!(""),
            }

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
            println!("({}): {}, [{}{}]",piece.id, piece.piece_type.to_str(), SquareFile::from(piece.position.file).to_str(), SquareRank::from(piece.position.rank).to_str());
        }
    }

}

#[derive(Clone,Debug)]
struct Piece{
    id : u8, // For ease of search and Id
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
                rank : 0,
                file : 0,
            },
            possible_moves : vec![],
            threatened : false,
            color : PieceColor::Black,
            piece_type : PieceType::None,
        }
    }

    fn new(rank : u8, file : u8, _color : PieceColor, _type : PieceType) -> Self{
        //CUR_PIECE_NUM.fetch_add;
        Self{
            position : Square::new(SquareRank::from(rank),SquareFile::from(file)),
            color : _color,
            possible_moves : vec![],
            threatened : false,
            id : rank + file, //TODO: Implement global UUID
            piece_type : _type,
        }
    }

    fn all() -> Vec<Self> {
        let mut pieces = vec![];
            for file in 0..8{
                pieces.push(Self::new(1,file,PieceColor::White,PieceType::Pawn));
                pieces.push(Self::new(6,file,PieceColor::Black,PieceType::Pawn));
            }
            pieces.push(Self::new(0,0,PieceColor::White,PieceType::Rook));
            pieces.push(Self::new(0,7,PieceColor::White,PieceType::Rook));
            pieces.push(Self::new(7,0,PieceColor::Black,PieceType::Rook));
            pieces.push(Self::new(7,7,PieceColor::Black,PieceType::Rook));

            pieces.push(Self::new(0,1,PieceColor::White,PieceType::Knight));
            pieces.push(Self::new(0,6,PieceColor::White,PieceType::Knight));
            pieces.push(Self::new(7,1,PieceColor::Black,PieceType::Knight));
            pieces.push(Self::new(7,6,PieceColor::Black,PieceType::Knight));

            pieces.push(Self::new(0,2,PieceColor::White,PieceType::Bishop));
            pieces.push(Self::new(0,5,PieceColor::White,PieceType::Bishop));
            pieces.push(Self::new(7,2,PieceColor::Black,PieceType::Bishop));
            pieces.push(Self::new(7,5,PieceColor::Black,PieceType::Bishop));

            pieces.push(Self::new(0,4,PieceColor::White,PieceType::King));
            pieces.push(Self::new(7,4,PieceColor::Black,PieceType::King));

            pieces.push(Self::new(0,3,PieceColor::White,PieceType::Queen));
            pieces.push(Self::new(7,3,PieceColor::Black,PieceType::Queen));
        pieces
    }
   
    //NOTE: Board is in charge of validating the move, by contract,
    //      we assume that at this point the move is valid
    //      despite Pieces owning their possible valid moves
    fn update_pos(&mut self, new_pos : &Square){
            self.position.file = new_pos.file;
            self.position.rank = new_pos.rank;
    }
}

fn main() {
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
    for (idx_r,rank) in (0..8).rev().enumerate(){ //POV: Playing as white pieces
    //for (idx_r,rank) in (0..8).enumerate(){ //POV: Playing as black pieces
        print!("{}. ", ansi_term::Color::Purple.paint((rank+1).to_string()));
        for (idx_c,file) in (0..8).enumerate(){
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
            if square_contains_piece_square(&chess_board, rank, file){
                
                let cur_piece = get_piece_at(&chess_board,rank, file).expect("square_contains_piece_square to work as intended");
                cur_pos.push((file,rank));
                paint_piece(&cur_piece);
            }else{
                print!("{}",square_color.paint("#"));
            }
        }
            println!("");

        }
            print!("   "); // padding
            for (_,file) in (0..8).enumerate(){
                print!("{}", ansi_term::Color::Purple.paint(SquareFile::from(file).to_str()));
            }
            println!("");
            chess_board.print_pieces();
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
                    if input_buffer.len() < 5{
                        //invalid input. valid input e.g. 'Pe4e5g, 
                    }else{
                        let source_square = input_buffer.get(1..3).expect("input buffer to havee 1..3"); 
                        let dest_square =  input_buffer.get(3..5).expect("input buffer to have 3..5"); 
                        
                        
                        let src_file = source_square.chars().nth(0).expect("source square to have .nth(1)");
                        let src_rank = source_square.chars().nth(1).expect("source square to have .nth(2)");

                        let dst_file = dest_square.chars().nth(0).expect("dest square to have .nth(3)");
                        let dst_rank = dest_square.chars().nth(1).expect("des square to have .nth(4)");

                        if square_contains_piece_square(
                            &chess_board, 
                            SquareRank::from_char(src_rank) as u8 - 1, 
                            SquareFile::from_char(src_file) as u8 //
                            ){

                            //println!("You have selected a valid piece");
                            let piece_id = get_piece_id_at( 
                                &chess_board, 
                                &Square{file : SquareFile::from_char(src_file) as u8 - 1, 
                                       rank: SquareRank::from_char(src_rank) as u8   }
                                );
                                                                   
                            move_piece(&mut chess_board, piece_id, 
                                &Square{file : SquareFile::from_char(dst_file) as u8 - 1, 
                                       rank: SquareRank::from_char(dst_rank) as u8   }
                                       );
                            //
                            //if we successfully move a piece then we tick a turn
                            chess_board.advance_turn();

                        }else{
                            println!("You selected an empty square, plese select a square with a piece in it");
                        }
                    }
            }
    }
}




