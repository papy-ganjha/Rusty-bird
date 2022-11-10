use std::io::{stdin, stdout, Write};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


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
    thread::spawn(move || detect_user_input(sender));

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

fn detect_user_input(sender: Sender<char>){
    let stdin = stdin();
    //detecting keydown events
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('w') => sender.send('w').unwrap(),
            Key::Char('a') => sender.send('a').unwrap(),
            Key::Char('s') => sender.send('s').unwrap(),
            Key::Char('d') => sender.send('d').unwrap(),
            Key::Char('q') => sender.send('q').unwrap(),
            _ => println!("Don't know this key bro!"),
        };
    }
}
