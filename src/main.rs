use std::io;
use std::io::Write;

use num::Num;
use num_bigint::BigUint;

fn main() -> io::Result<()> {
    println!("bin2dec - 二进制转十进制");

    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        buffer.clear();
        let _ = io::stdin().read_line(&mut buffer)?;

        // Stdin::read_line 会在末尾添加换行符，这里必须去掉，同时去掉首尾的空格
        // 需要注意的是，即使不用去掉首尾空格，也不能简单地用 &buffer[..buffer.len() - 1]，
        // 因为 Windows 下换行符是 CRLF，是两个字符
        let digits = buffer.trim();

        if digits.is_empty() {
            continue;
        }

        // 检查输入中是否包含非法字符
        if let Err(index) = check_input(digits) {
            print_invalid_char_message(digits, index);
            continue;
        }

        match BigUint::from_str_radix(digits, 2) {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("转换失败：{:?}", err),
        }
    }
}

fn check_input(input: &str) -> Result<(), usize> {
    for (i, ch) in input.chars().enumerate() {
        if ch != '0' && ch != '1' {
            return Err(i);
        }
    }
    Ok(())
}

fn print_invalid_char_message(input: &str, index: usize) {
    eprintln!("输入了非法字符：");
    eprintln!("{}", input);
    for _ in 0..index {
        eprint!(" ");
    }
    eprintln!("^");
}
