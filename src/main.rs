use std::io::{BufRead, Write};

use jesschess::board::Board;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("jesschess version: {}", jesschess::version());

    let mut stdin = std::io::stdin().lock();
    let mut cmd_buf = String::new();
    let mut board = None;

    loop {
        print!("jesschess> ");
        std::io::stdout().flush()?;

        cmd_buf.clear();
        
        stdin.read_line(&mut cmd_buf)?;
        let cmd = cmd_buf
            .split_whitespace()
            .collect::<Vec<_>>();

        match cmd.as_slice() {
            ["quit" | "q"] => {
                println!("bye!");
                break
            },
            ["version"] => println!("{}", jesschess::version()),
            ["help" | "h"] => println!("Commands: quit/q, version, help/h, position/pos, print/p"),
            ["position" | "pos", "startpos"] => {
                board = Some(Board::startpos());
            },
            ["position" | "pos", fen @ ..] => {
                let fen = fen.join(" ");
                match Board::from_fen(fen.as_str()) {
                    Err(e) => println!("Error: {}", e),
                    Ok(b) => board = Some(b),
                }
            },
            ["print" | "p"] => {
                match &board {
                    Some(b) => println!("{b}"),
                    None => println!("No board loaded"),
                }
            }
            _ => println!("Unknown command: {}", cmd.join(" ")),
        }
    }
    
    Ok(())
}