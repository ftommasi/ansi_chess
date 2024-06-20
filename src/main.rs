

#[derive(Clone)]
enum PieceType{
    None,
    Pawn,
    Knight,
    Bishop,
    Rook  ,
    Queen ,
    King  ,
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
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("H")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("H")),
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
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("S")),
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

#[derive(Clone)]
enum PieceColor{
    Black,
    White,
}

enum SquareRank{
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl SquareRank{
    fn from(val : u8) -> SquareRank{
        match val{
            0 => SquareRank::A,
            1 => SquareRank::B,
            2 => SquareRank::C,
            3 => SquareRank::D,
            4 => SquareRank::E,
            5 => SquareRank::F,
            6 => SquareRank::G,
            7 => SquareRank::H,
            _ => unreachable!("{} exceeds Rank value!",val),
        }
    }
}


#[derive(Clone)]
struct Square{
    row : u8,
    col : u8,
}

impl Square {
    fn new(rank : SquareRank, file : u8) -> Square{
        Square{row : rank as u8, col : file}
    }

    fn eq(&self,other : &Square) -> bool{
        return self.col == other.col && self.row == other.col;
    }

    fn all() -> Vec<Vec<Square>> {
        let mut ranks = vec![];
        for rank in 0..8{
            let mut squares = vec![];
            for file in 0..8{
                squares.push(Square::new(SquareRank::from(rank),file));
            }
            ranks.push(squares);
        }
        ranks
    }
}

struct Board {
    pieces : Vec<Piece>,
    squares : Vec<Vec<Square>>,
}

impl Board{
  
    fn new() -> Board{
        Board{
            pieces : Piece::all(),//vec![Piece::new();16],
            squares : Square::all(),
        }
    }

    fn square_contains_piece_square(&self, rarnk : u8, file : u8) -> bool{
        for piece in self.pieces.as_slice(){
            if piece.position.row == rarnk && piece.position.col == file{
                return true;
            }
        }
        false
    }


    fn get_piece_at(&self, rarnk : u8, file : u8) -> Option<Piece>{
        for piece in self.pieces.as_slice(){
            if piece.position.row == rarnk && piece.position.col == file{
                return Some(piece.clone());
            }
        }
        None
    }

    //Should this return a bool for success or something?
    fn move_piece(&mut self, piece_id : u8, new_pos : &Square){
    let mut selected_piece : Option<&mut Piece> = None;
    for piece in &mut self.pieces{
        if piece.id == piece_id{
            selected_piece = Some(piece);
        }
    }
    
    match selected_piece {
        Some(piece) => {
            for valid_move in piece.possible_moves.as_slice(){
                if valid_move.eq(new_pos) {
                    //piece.update_pos(new_pos); //<- why is this a problem??
                    piece.position.row = new_pos.row;
                    piece.position.col = new_pos.col;

                }
            }
            // reclaculate moves here. Calling another self, mut function is not working and this
            // is probably the only place where it is needed
            piece.possible_moves = vec![Square{row : (piece.position.row + 1)%8,col : (piece.position.col + 1)%8 }];
            todo!("recalculate_valid_moves is not implemented yet");
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
}

#[derive(Clone)]
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
    fn default() -> Piece{
        return Piece{
            id : 0, //TODO: should we track this better, probably..
            position : Square {
                row : 0,
                col : 0,
            },
            possible_moves : vec![],
            threatened : false,
            color : PieceColor::Black,
            piece_type : PieceType::None,
        }
    }

    fn new(rank : u8, file : u8, _color : PieceColor, _type : PieceType) -> Piece{
        //CUR_PIECE_NUM.fetch_add;
        Piece{
            position : Square::new(SquareRank::from(rank),file),
            color : _color,
            possible_moves : vec![],
            threatened : false,
            id : rank + file, //TODO: Implement global UUID
            piece_type : _type,
        }
    }

    fn all() -> Vec<Piece> {
        let mut pieces = vec![];
            for file in 0..8{
                pieces.push(Piece::new(1,file,PieceColor::White,PieceType::Pawn));
                pieces.push(Piece::new(6,file,PieceColor::Black,PieceType::Pawn));
            }
        pieces
    }
    
    //NOTE: Board is in charge of validating the move, by contract,
    //      we assume that at this point the move is valid
    //      despite Pieces owning their possible valid moves
    fn update_pos(&mut self, new_pos : &Square){
            self.position.col = new_pos.col;
            self.position.row = new_pos.row;
    }
}

fn main() {
    let chess_board = Board::new();
    //init board
    
    for (idx_r,row) in (0..8).enumerate(){
        for (idx_c,col) in (0..8).enumerate(){
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
            if chess_board.square_contains_piece_square(row, col){
                let cur_piece = chess_board.get_piece_at(row, col).expect("square_contains_piece_square to work as intended");
                match cur_piece.piece_type{
                    PieceType::Pawn => print!("{}",square_color.paint("P")),
                    _ => print!("{}",square_color.paint("X")),
                }
            }else{
                print!("{}",square_color.paint("#"));
            }
        }
        println!("");
    }
}
