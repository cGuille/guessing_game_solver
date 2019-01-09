use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

const SOCKET_PATH: &str = "/tmp/guessing_game_server.sock";

fn main() {
    let stream = UnixStream::connect(SOCKET_PATH)
        .expect("Could not connect to socket; please check if the server is started");

    let mut stream = BufReader::new(stream);

    let mut min = 1;
    let mut max = 100;

    loop {
        let mut prompt = String::new();
        stream.read_line(&mut prompt).unwrap();

        if prompt.trim() != "Please input your guess." {
            panic!("Unexpected welcome message: {}", prompt);
        }

        let guess = min + ((max - min) / 2);

        println!("Guessing {}", guess);
        let message = format!("{}\n", guess);
        stream.get_ref().write(message.as_bytes()).unwrap();

        let mut result = String::new();
        stream.read_line(&mut result).unwrap();
        let result = result.trim();

        println!("{}", result);

        match result {
            "Too big!" => max = guess,
            "Too small!" => min = guess,
            "You win!" => break,
            _ => panic!("Unexpected result: {}", result),
        };
    }
}
