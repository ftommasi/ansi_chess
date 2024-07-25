
use tokio::{net::TcpStream};

pub struct GameState {
    board: String,
    seconds_left: i32,
}

impl GameState {
    //TODO: implement
    fn new() -> Self {
        Self {
            board: String::new(),
            seconds_left: 180,
        }
    }
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //let game_state_ref = Arc::new(Mutex::new(GameState::new()));
    //let server_listener = TcpListener::bind(&format!("127.0.0.1:{}", 8080)).await?;

    //let black_game_state_Ref = Arc::clone(&game_state_ref);
    //let white_game_state_Ref = Arc::clone(&game_state_ref);

    //let white_listener = TcpListener::bind(&format!("127.0.0.1:{}", 8080)).await?;
    //let black_listener = TcpListener::bind(&format!("127.0.0.1:{}", 8081)).await?;

    Ok(())
}

pub async fn process_client_input(move_in: TcpStream) {
    //check to see if we got a move, or a command (chat, resign, draw, )
    //check to see if it its your turn else yield
    let mut buf: [u8; 1024] = [0; 1024]; //Picked arbitrary size. TODO: estimate/concretize how much this makes sense
    move_in.try_read(&mut buf);
    todo!("finish process_client impl");
}
pub async fn bcast_to_clients() {}
