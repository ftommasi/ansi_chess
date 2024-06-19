

#[derive(Clone)]
enum PieceType{
    None(Piece),
    Pawn  (Piece),
    Knight(Piece),
    Bishop(Piece),
    Rook  (Piece),
    Queen (Piece),
    King  (Piece),
}

fn paint_piece(piece : &PieceType){
    match piece{
                PieceType::Pawn(x)  => {
                    match x.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("P")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("P")),
                    }
                }
                PieceType::Knight(x)  => {
                    match x.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("N")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("N")),
                    }
                }
                PieceType::Bishop(x)  => {
                    match x.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("H")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("H")),
                    }
                }
                PieceType::Rook(x)  => {
                    match x.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("R")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("R")),
                    }
                }
                PieceType::Queen(x)  => {
                    match x.color{
                        PieceColor::Black => print!("{}", ansi_term::Color::Green.paint("Q")),
                        PieceColor::White => print!("{}", ansi_term::Color::White.paint("S")),
                    }
                }
                PieceType::King(x)  => {
                    match x.color{
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
    pieces : Vec<PieceType>,
    squares : Vec<Vec<Square>>,
}

impl Board{
  
    fn new() -> Board{
        Board{
            pieces : Piece::all(),//vec![Piece::new();16],
            squares : Square::all(),
        }
    }

    fn recalculate_valid_moves(&self, piece : &mut Piece){
        piece.possible_moves = vec![Square{row : (piece.position.row + 1)%8,col : (piece.position.col + 1)%8 }];
        todo!("recalculate_valid_moves is not implemented yet");
    }

    //Should this return a bool for success or something?
    fn move_piece(&mut self, piece_id : u8, new_pos : &Square){
        for &mut piece in &mut self.pieces{
            //We found the piece we want to move
            if piece.id == piece_id{
                for valid_move in &mut piece.possible_moves{
                    //if we have a valid move. move the piece
                    if valid_move.eq(new_pos){
                        piece.update_pos(new_pos);
                        self.recalculate_valid_moves(&mut piece);
                        return; // break for performance
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Piece{
    id : u8, // For ease of search and Id
    position : Square,
    threatened : bool,
    possible_moves : Vec<Square>,
    color : PieceColor
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
        }
    }

    fn new(rank : u8, file : u8, _color : PieceColor) -> Piece{
        //CUR_PIECE_NUM.fetch_add;
        Piece{
            position : Square::new(SquareRank::from(rank),file),
            color : _color,
            possible_moves : vec![],
            threatened : false,
            id : rank + file, //TODO: Implement global UUID
        }
    }

    fn all() -> Vec<PieceType> {
        let mut pieces = vec![];
            for file in 0..8{
                pieces.push(PieceType::Pawn(Piece::new(1,file,PieceColor::White)));
                pieces.push(PieceType::Pawn(Piece::new(6,file,PieceColor::Black)));
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
    //let mut chess_board : Board = vec![vec![::new();8];8];
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

            print!("{}",square_color.paint("#"));
        }
        println!("");
    }
}
