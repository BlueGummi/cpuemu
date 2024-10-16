use crate::*;
use std::process;

// these are helper functions, not missiong critical, they help.

pub fn neg_num_err(instruction: &str) {
    eprintln!(
        "{}{}{}",
        "ERROR, ".color(Colors::RedFg),
        instruction.color(Colors::YellowFg),
        " WILL RESULT IN NEGATIVE NUMBER.\nTERMINATING.".color(Colors::RedFg)
    );
    process::exit(0);
}

pub fn err_print(error: String) {
    eprintln!(
        "{}{}",
        "ERROR, ".color(Colors::RedFg),
        error.color(Colors::RedFg)
    );
    process::exit(0);
}

pub fn letter_to_integer(letter: char) -> Option<u8> {
    if letter.is_ascii_lowercase() {
        Some(letter as u8 - b'a')
    } else if letter.is_ascii_uppercase() {
        Some(letter as u8 - b'A')
    } else {
        None
    }
}

pub fn integer_to_letter(n: usize) -> char {
    if n < 26 {
        (n as u8 + b'a') as char
    } else {
        err_print("value passed to integer_to_letter was too large.".to_string());
        process::exit(0);
    }
}

pub fn has_b_with_num(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut found_b = false;
    for &byte in bytes {
        if found_b {
            if byte.is_ascii_digit() {
                return true;
            } else {
                found_b = false; // reset if not a digit
            }
        }
        if byte == b'b' || byte == b'B' {
            found_b = true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn debug_print(instruc: &str, src: &String, dest: &String, f_contents: &str) {
    println!(
        "\nRemaining line:\n{}",
        f_contents.trim().color(Colors::YellowFg)
    );
    println!("{}", "FOUND INSTRUCTION".color(Colors::BlueFg));
    print!("{}", "INSTRUCTION:".color(Colors::RedFg));
    println!(
        "{}\n",
        instruc.to_uppercase().color(Colors::BrightMagentaFg)
    );
    print!("{}", "SRC:".color(Colors::RedFg));
    println!("{}\n", dest.color(Colors::BrightMagentaFg));
    print!("{}", "DEST:".color(Colors::RedFg));
    print!("{}\n\n", src.color(Colors::BrightMagentaFg));
}

// this is here for debug, please ignore :)
#[allow(dead_code)]
pub fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
