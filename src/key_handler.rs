use std::sync::mpsc::Sender;
use termion::event::Key;
use termion::input::TermRead;
use std::io::stdin;


pub fn detect_user_input(sender: Sender<char>){
    let stdin = stdin();
    //detecting keydown events
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('w') => sender.send('w').unwrap(),
            Key::Char('a') => sender.send('a').unwrap(),
            Key::Char('s') => sender.send('s').unwrap(),
            Key::Char('d') => sender.send('d').unwrap(),
            Key::Char('q') => sender.send('q').unwrap(),
            Key::Esc => sender.send('q').unwrap(),
            Key::Ctrl('c') => sender.send('q').unwrap(),
            _ => println!("Don't know this key bro!"),
        };
    }
}
