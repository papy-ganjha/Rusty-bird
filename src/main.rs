use std::io::{stdout, Write};
use std::thread;
use std::sync::mpsc;
use termion::raw::IntoRawMode;

mod key_handler;


const CURSOR_X: u16 = 0;
const CURSOR_Y: u16 = 1;
fn main() {
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}Welcome to my new game it should be a rusty flappy bird with some useless features just to learn rust""#, termion::cursor::Goto(CURSOR_X, CURSOR_Y), termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();

    // Create Sender and Receiver object to comunicte thgrough threads
    let (sender, receiver) = mpsc::channel();
    // Create seperate thread for detecting input user and gives him the sender object
    thread::spawn(move || key_handler::detect_user_input(sender));

    loop{
        match receiver.try_recv() {
            Ok('w') => println!("w is received!"),
            Ok('a') => println!("a is received!"),
            Ok('s') => println!("s is received!"),
            Ok('d') => println!("d is received!"),
            Ok('q') => break,
            _ => {}
        };
    }

    println!("Game ended!");
    stdout.flush().unwrap();
}
