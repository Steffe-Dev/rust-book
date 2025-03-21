use std::io::{Write, stdin, stdout};

use ch8_ex_3_department_index::{CommandResult, Company};

fn main() {
    let mut company = Company::new();
    loop {
        let mut buf = String::new();
        print!(">> ");
        stdout().flush().expect("Should be able to flush stdout");
        if let Err(_) = stdin().read_line(&mut buf) {
            eprintln!("Something went wrong while reading input");
            continue;
        };

        if let CommandResult::Break = ch8_ex_3_department_index::execute(buf.trim(), &mut company) {
            break;
        }
    }
}
