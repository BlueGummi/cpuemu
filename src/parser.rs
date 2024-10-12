use crate::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Reads the contents of a file or creates it with default content.
pub fn read_file(f_name: &String) -> String {
    if Path::new(&f_name).exists() {
        fs::read_to_string(&f_name).unwrap_or_else(|_| {
            println!("Error reading file '{}'. Exiting.", f_name);
            std::process::exit(1);
        })
    } else {
        println!("Could not find file; creating it.");
        let default_content = "MOV 1, 5\nMOV 2, 3\nADD 0, 1\nSUB 1, 2\nMUL 1, 2";
        fs::write(&f_name, default_content).unwrap_or_else(|_| {
            println!("Could not write to file '{}'. Exiting.", f_name);
            std::process::exit(1);
        });
        default_content.to_string()
    }
}

/// Lexer to tokenize the assembly code.
fn lex(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            let clean_line = line.split(';').next().unwrap_or(line);
            clean_line
                .split_whitespace()
                .filter(|token| !token.is_empty()) // Ignore empty tokens
                .map(|token| token.to_string())
                .collect()
        })
        .collect()
}

/// Parses the tokenized lines into instructions, handling functions internally.
pub fn parse_file(f_contents: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut functions = HashMap::new();
    let config = declare_config();
    let tokens = lex(&f_contents);
    let mut current_function: Option<String> = None;
    let mut current_function_instructions = Vec::new();

    if config.verbose_debug {
        println!("Tokenized instructions:\n{:?}", tokens);
    }

    for (line_number, tokens) in tokens.iter().enumerate() {
        if tokens.is_empty() {
            continue; // Skip empty lines
        }

        if tokens[0].starts_with('.') {
            if tokens[0] == ".end" {
                if let Some(func_name) = current_function.take() {
                    functions.insert(func_name, current_function_instructions);
                    current_function_instructions = Vec::new(); // Reset for next function
                } else {
                    println!(
                        "Error: .end without a corresponding function on line {}.",
                        line_number
                    );
                    std::process::exit(0);
                }
            } else {
                // Start a new function
                if current_function.is_none() {
                    current_function = Some(tokens[0].clone());
                } else {
                    println!(
                        "Error: Nested function definitions are not allowed on line {}.",
                        line_number
                    );
                    std::process::exit(0);
                }
            }
        } else if let Some(ref _func_name) = current_function {
            // Collect instructions for the current function
            if let Some(instruction) = parse_instruction(tokens, line_number as i32) {
                current_function_instructions.push(instruction);
            }
        } else if let Some(instruction) = parse_instruction(tokens, line_number as i32) {
            instructions.push(instruction);
        }
    }

    if config.verbose_debug {
        println!("Global instructions: {:?}", instructions);
        println!("Functions: {:?}", functions);
    }

    // Ensure HALT at the end of global instructions
    instructions.push(Instruction::HALT);

    instructions
}

/// Parses a single instruction from tokens.
fn parse_instruction(tokens: &[String], line_number: i32) -> Option<Instruction> {
    if tokens.is_empty() {
        return None; // No instruction found
    }
    let instruc = &tokens[0];
    let (dest, src): (u16, u16) = parse_operands(tokens);
    match instruc.to_uppercase().as_str() {
        "ADD" => Some(Instruction::ADD(dest, src)),
        "SUB" => Some(Instruction::SUB(dest, src)),
        "MUL" => Some(Instruction::MUL(dest, src)),
        "MOV" => {
            // Function to create a MOV instruction based on the destination and source
            fn create_mov_instruction(dest: u16, src: Option<&str>) -> Instruction {
                match src {
                    Some(value) => {
                        // Try to parse the source value as u16
                        if let Ok(parsed_value) = value.parse::<u16>() {
                            // If parsed successfully, check if it's a register or immediate value
                            Instruction::MOV(dest, parsed_value) // Move immediate value
                        } else {
                            // If parsing fails, treat the value as a register
                            let reg_index =
                                letter_to_integer(value.chars().next().unwrap_or(' ')).unwrap_or(0);
                            Instruction::MOVR(dest, reg_index.into()) // Move from register
                        }
                    }
                    None => Instruction::MOV(dest, 0), // Default to moving 0 if src is None
                }
            }

            // Convert dest and src to u16 and call create_mov_instruction
            let instruction = create_mov_instruction(dest.try_into().unwrap(), Some(&tokens[2]));
            Some(instruction)
        }
        "SWAP" => Some(Instruction::SWAP(dest, src)),
        "DIV" => Some(Instruction::DIV(dest, src)),
        "CLR" => Some(Instruction::CLR(dest)),
        "DEC" => Some(Instruction::DEC(dest)),
        "INC" => Some(Instruction::INC(dest)),
        "CMP" => Some(Instruction::CMP(dest, src)),
        "HALT" => Some(Instruction::HALT),
        "PRINT" => Some(Instruction::PRINT(dest)),
        "POW" => Some(Instruction::POW(dest, src.try_into().ok()?)),
        "MOVR" => Some(Instruction::MOVR(dest, src)),
        "JMP" => Some(Instruction::JMP(dest)),
        _ => {
            println!(
                "Error: Unknown instruction: \"{}\" on line {}.",
                instruc, line_number
            );
            std::process::exit(0);
        }
    }
}

/// Parses the operands from the tokenized line.
fn parse_operands(tokens: &[String]) -> (u16, u16) {
    let dest = parse_value(&tokens.get(1).unwrap_or(&"0".to_string()));
    let src = parse_value(&tokens.get(2).unwrap_or(&"0".to_string()));
    (dest, src)
}

/// Converts a token into a u16 value, handling both numeric and register inputs.
fn parse_value(token: &String) -> u16 {
    if token.starts_with("b") && has_b_with_num(token) {
        i32::from_str_radix(&token[2..], 2).unwrap_or_else(|_| {
            println!("Error: Not a valid binary number: {}", token);
            std::process::exit(0);
        }) as u16 // Cast to u16 instead of usize
    } else if let Ok(value) = token.parse::<u16>() {
        // Change to parse::<u16>()
        value
    } else {
        letter_to_integer(token.chars().next().unwrap_or(' '))
            .unwrap_or(0)
            .into() // Ensure this returns a u16
    }
}
