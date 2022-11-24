#![feature(more_qualified_paths)]
use std::io::{stdout, Write};
use std::thread;
use std::sync::mpsc;
use termion::raw::IntoRawMode;

mod key_handler;

mod frame_generator;
use frame_generator::{ FrameGenerator, Generator };

const CURSOR_X: u16 = 1;
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

    let mut frame_gen: FrameGenerator = FrameGenerator::new(50, 50);
    frame_gen.init_generator();

    loop{
        match receiver.try_recv() {
            Ok(' ') => handle_jump(),
            Ok('q') => break,
            _ => { 
                write!(stdout, r#"{}"#, termion::cursor::Goto(CURSOR_X, CURSOR_Y)).unwrap();
                let mut i = 0;
                let mut current_y = CURSOR_Y;
                for line in frame_gen.frame.iter() {
                    i = i + 1;
                    write!(stdout, r#"{}{}"#, termion::cursor::Goto(CURSOR_X, current_y), line).unwrap();
                    current_y = current_y + 1;
                }
            }
        };
        stdout.flush().unwrap()
    }
    println!("Game ended!");
    stdout.flush().unwrap();
}

fn handle_jump(){

}
