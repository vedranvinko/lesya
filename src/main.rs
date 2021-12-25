use rustyline::Editor;

use std::io;
use std::io::Write;

fn main() {
    println!("lesya {}", env!("CARGO_PKG_VERSION"));
    println!("Press Ctrl+c to Exit\n");

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("lesya> ");
        io::stdout().flush().unwrap();

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", line);
            }
            Err(_) => break,
        }
    }
}
