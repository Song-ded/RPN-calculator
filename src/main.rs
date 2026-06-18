use num_traits;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fmt::Debug;
use std::str::FromStr;
use colored::Colorize;

fn evalute<T>(expression: &str) -> Result<T, String>
where
    T: num_traits::Num + std::str::FromStr + std::fmt::Display + Copy + std::fmt::Debug,
    <T as FromStr>::Err: Debug,
{
    let mut stack = Vec::new();

    for word in expression.split_whitespace() {
        match word {
            "+" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are required before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is required before '{word}'"))?;
                let result = operand1 + operand2;
                println!("{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "-" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are required before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is required before '{word}'"))?;
                let result = operand1 - operand2;
                println!("{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "*" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are required before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is required before '{word}'"))?;
                let result = operand1 * operand2;
                println!("{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "/" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are required before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is required before '{word}'"))?;
                let result = operand1 / operand2;
                println!("{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            "%" => {
                let operand2 = stack
                    .pop()
                    .ok_or(format!("2 operands are required before '{word}'"))?;
                let operand1 = stack
                    .pop()
                    .ok_or(format!("1 operand is required before '{word}'"))?;
                let result = operand1 % operand2;
                println!("{operand1} {word} {operand2} = {result}");
                stack.push(result);
            },
            _ => {
                stack.push(word.parse::<T>().map_err(|_| format!("Can't parse {word} to {word}"))?);
            },
        }
    }

    if stack.len() > 1 {
        let s = format!("Skipped the operand(s) {:?}", &stack[..stack.len() - 1]);
        println!("{}", s.yellow());
    }
    Ok(stack.pop().unwrap())
}

fn main() -> rustyline::Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        //println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let (typ, expression) = line.split_once(':').unwrap_or(("float", &line));
                match typ {
                    "float" => {
                        let result = evalute::<f64>(expression);
                        match result {
                            Ok(i) => println!("{}", i),
                            Err(s) => println!("{}", s),
                        }
                    }
                    "int" => {
                        let result = evalute::<i64>(expression);
                        match result {
                            Ok(i) => println!("{}", i),
                            Err(s) => println!("{}", s.red()),
                        }
                    }
                    _ => {
                        println!("Unknown type {} =(", typ.red());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                //println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                //println!("CTRL-D");
                break;
            }
            Err(err) => {
                //println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt");
    Ok(())
}
