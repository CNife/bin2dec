//! # Bin2Dec - 二进制数转十进制
//! 
//! 参见 [app-ideas/Bin2Dec](https://github.com/florinpop17/app-ideas/blob/master/Projects/1-Beginner/Bin2Dec-App.md)
//! 
//! ## 用法
//! `cargo run` 编译并运行程序，每行输入一个二进制数字，按 ENTER 键转换数字。
//! 
//! ## 错误
//! 1. 输入除 1 和 0 以外的字符；
//! 2. 输入超过 8 个数字；
//! 3. 没有输入；

use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    println!("bin2dec");

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
        if digits.len() > 8 {
            eprintln!("input too long , 8 digits at most");
            continue;
        }

        match u8::from_str_radix(digits, 2) {
            Ok(result) => println!("{} => {}", digits, result),
            Err(err) => eprintln!("input is not valid\n\t{:?}", err),
        }
    }
}
