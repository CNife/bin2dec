use std::io;
use std::io::Write;

const MAX_DIGITS: usize = 8;

fn main() {
    println!("Convert binary number to decimal.");

    let mut buffer = String::new();
    loop {
        print!("Input: ");
        io::stdout().flush().expect("Error when flushing stdout");

        buffer.clear();
        match io::stdin().read_line(&mut buffer) {
            Ok(len) => {
                if len - 1 > MAX_DIGITS {
                    eprintln!("Too Long, 8 digits at most");
                    continue;
                }

                match u8::from_str_radix(&buffer.trim_end(), 2) {
                    Ok(result) => println!("{}", result),
                    Err(parse_err) => eprintln!("Error when parsing: {:?}", parse_err),
                }
            }
            Err(err) => {
                eprintln!("Error when reading stdin: {:?}", err)
            }
        }
    }
}
