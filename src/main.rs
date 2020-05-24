use std::io;
use std::io::Write;

const MAX_DIGITS: usize = 8;

fn main() -> io::Result<()> {
    println!("Convert binary number to decimal.");

    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        buffer.clear();
        let _ = io::stdin().read_line(&mut buffer)?;

        let digits = buffer.trim_end();
        if digits.len() > MAX_DIGITS {
            eprintln!("input too long , {} digits at most", MAX_DIGITS);
            continue;
        }

        match u8::from_str_radix(digits, 2) {
            Ok(result) => println!("{} => {}", digits, result),
            Err(err) => eprintln!("input is not valid\n\t{:?}", err),
        }
    }
}
