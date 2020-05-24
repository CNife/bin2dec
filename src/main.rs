//! # Commandline binary number to decimal convertor.
//! input a binary number (in `0` and `1`) and press ENTER,
//! the decimal form of it will be printed out.
//!
//! You can input at most **8** digits one line, for a `u8`.

use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    println!("Convert binary number to decimal.");

    let mut buffer = String::new();
    loop {
        print!("> ");
        // flush stdout to make prompt visible
        io::stdout().flush()?;

        buffer.clear();
        let _ = io::stdin().read_line(&mut buffer)?;

        // must trim buffer to remove trailing line seperator
        // shouldn't use `&buffer[..buffer.len() - 1]` because on Windows, 
        // the seperator is `\r\n`, which occupies two `u8`s
        let digits = buffer.trim_end();
        if digits.len() > 8 {
            eprintln!("input too long , 8 digits at most");
            continue;
        }

        // can't use `str.parse` because it can only used to parse decimals.
        match u8::from_str_radix(digits, 2) {
            Ok(result) => println!("{} => {}", digits, result),
            Err(err) => eprintln!("input is not valid\n\t{:?}", err),
        }
    }
}
