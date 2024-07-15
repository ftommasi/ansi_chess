use std::{ascii::AsciiExt, env, io, ops::Add, ops::Sub};

mod ansi_chess;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    interactive_mode();
    //test_print();
}

fn test_print() {
    let mut chess_board = ansi_chess::Board::new();
    //print!("{}", chess_board.to_string());
    //from viuwa main. How do we abstract this into a function call?
    //let orig = {
    //    image::io::Reader::open(&config.image)?
    //        .with_guessed_format()?
    //        .decode()
    //        .context("Failed to load image, the file extension may be incorrect")?
    //};
}

fn interactive_mode() {
    let mut chess_board = ansi_chess::Board::new();
    //init board
    //TODO: I;m sure there is a better way to do this, doing it the quick and dirty way for now
    let mut chess_board_clone = chess_board.clone();
    for piece in &mut chess_board.pieces {
        let mut moves = ansi_chess::get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
        piece.possible_moves.append(&mut moves);
    }
    let mut move_history = String::new();
    'game: loop {
        //Draw board. Refactor?
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
                } else {
                    if idx_c % 2 == 0 {
                        square_color = ansi_term::Color::Green;
                    } else {
                        square_color = ansi_term::Color::White;
                    }
                }
                if ansi_chess::square_contains_piece_square(
                    &chess_board,
                    ansi_chess::SquareRank::from(rank),
                    ansi_chess::SquareFile::from(file),
                ) {
                    let cur_piece =
                        ansi_chess::get_piece_at(&chess_board, ansi_chess::SquareRank::from(rank), ansi_chess::SquareFile::from(file))
                            .expect("square_contains_piece_square to work as intended");
                    cur_pos.push((file, rank));
                    ansi_chess::paint_piece(&cur_piece);
                } else {
                    print!("{}", square_color.paint("#"));
                }
            }
            println!("");
        }
        print!("   "); // padding
        for (_, file) in (1..9).enumerate() {
            print!(
                "{}",
                ansi_term::Color::Purple.paint(ansi_chess::SquareFile::from(file).to_str())
            );
        }
        println!("");
        //chess_board.print_pieces();

        let mut input_buffer = String::new();
        let result = io::stdin().read_line(&mut input_buffer);
        if !result.is_ok() {
            unreachable!("We have to get something from stdin...");
        }
        if input_buffer.contains("END") {
            break 'game;
        }
        //TODO: SPEED
        if input_buffer.contains("X") {
            let mut output_buffer = String::new();
            print!("Export file_name?: ");
            let result = io::stdin().read_line(&mut output_buffer);
            if output_buffer.ends_with('\n') {
                output_buffer.pop();
                if output_buffer.ends_with('\r') {
                    output_buffer.pop();
                }
            }

            let mut test_path_out = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
            let mut test_path_in = std::env::current_dir().unwrap_or(std::path::PathBuf::new());
            test_path_out.push("src");
            test_path_out.push("test_ins");
            test_path_in.push("src");
            test_path_in.push("test_ins");
            let test_out = output_buffer.clone() + "_out";
            let test_in = output_buffer.clone() + "_in";
            test_path_out.push(test_out.clone());
            test_path_in.push(test_in.clone());
            test_path_out.set_extension("txt");
            test_path_in.set_extension("txt");

            if !result.is_ok() {
                unreachable!("We have to get something from stdin...");
            }
            //println!("expected_in:\n'{}'",move_history);
            //println!("expected_out:\n'{}'",ansi_chess::Board::decode_board_ascii(chess_board.to_string()).to_string());

            if let Err(error) =
                std::fs::write(test_path_out.to_str().unwrap(), chess_board.to_string())
            {
                println!(
                    "we got error {} when trying to write {}",
                    error,
                    test_path_out.to_str().unwrap()
                );
            }

            if let Err(error) = std::fs::write(test_path_in, move_history.clone()) {
                println!(
                    "we got error {} when trying to write {}",
                    error,
                    test_in.clone()
                );
            }
            //println!("expected_out:\n'{}'",chess_board.to_string());
        }

        //if a move is being input
        if input_buffer.starts_with("P")
            || input_buffer.starts_with("N")
            || input_buffer.starts_with("I")
            || input_buffer.starts_with("R")
            || input_buffer.starts_with("Q")
            || input_buffer.starts_with("K")
        {
            if input_buffer.len() >= 5 {
                //invalid input. valid input e.g. 'Pe4e5g,
                let source_square = input_buffer.get(1..3).expect("input buffer to havee 1..3");
                let dest_square = input_buffer.get(3..5).expect("input buffer to have 3..5");

                let src_file = source_square
                    .chars()
                    .nth(0)
                    .expect("source square to have .nth(1)");
                let src_rank = source_square
                    .chars()
                    .nth(1)
                    .expect("source square to have .nth(2)");

                let dst_file = dest_square
                    .chars()
                    .nth(0)
                    .expect("dest square to have .nth(3)");
                let dst_rank = dest_square
                    .chars()
                    .nth(1)
                    .expect("des square to have .nth(4)");

                if ansi_chess::square_contains_piece_square(
                    &chess_board,
                    ansi_chess::SquareRank::from_char(src_rank),
                    ansi_chess::SquareFile::from_char(src_file), //
                ) {
                    //println!("You have selected a valid piece");
                    let piece_id = ansi_chess::get_piece_id_at(
                        &chess_board,
                        &ansi_chess::Square {
                            rank: ansi_chess::SquareRank::from_char(src_rank),
                            file: ansi_chess::SquareFile::from_char(src_file), //
                        },
                    );

                    //before we move piece lets remove
                    if let Some(target_piece) = ansi_chess::get_piece_at(
                        &chess_board,
                        ansi_chess::SquareRank::from_char(dst_rank),
                        ansi_chess::SquareFile::from_char(dst_file),
                    ) {
                        //By contract we know that if a valid move is a capture that the piece colors are opposite. See get_valid_moves_for_piece
                        chess_board.delete_piece(target_piece.id); //TODO: Currently this deletes a piece right after moving it...
                    }
                    if let Err(e) = ansi_chess::move_piece(
                        &mut chess_board,
                        piece_id,
                        &ansi_chess::Square {
                            file: ansi_chess::SquareFile::from_char(dst_file),
                            rank: ansi_chess::SquareRank::from_char(dst_rank),
                        },
                    ){
                        println!("Encountered '{:?}' error while moving piece",e);
                    }
                    //
                    //if we successfully move a piece then we tick a turn
                    chess_board.advance_turn();
                    move_history.push_str(" ");
                    move_history.push_str(input_buffer.as_str());

                    //After we move a piece recalculate possible moves
                    //TODO: Fix this hack with the chess_board_clone
                    chess_board_clone.pieces = chess_board.pieces.clone();
                    for piece in &mut chess_board.pieces {
                        piece.possible_moves.clear();
                        piece.set_targeted(false); //reset targeted status, it will get updated when computing valid moves
                        let mut moves =
                            ansi_chess::get_valid_moves_for_piece(&piece.clone(), &chess_board_clone);
                        piece.possible_moves.append(&mut moves);
                    }
                } else {
                    println!(
                        "You selected an empty square, plese select a square with a piece in it"
                    );
                }
            } else {
                println!("invalid input lneght {}", input_buffer.len());
            }
        }
    }
}
