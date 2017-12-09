use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    loop {
        print_prompt();
        let line = read_prompt();
        if line.starts_with(".") { 
            match do_meta_comand(&line) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => println!("Unrecognized command '{}'.", line),
            }
        } else if line == "" {
            continue;
        } else {
            match prepare_statement(&line) {
                None => println!("Unrecognized keyword at start of '{}'.", line),
                Some(statement) => {
                    execute_statement(statement);
                    println!("Executed.");
                }
            }
        }
    }
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    type_: StatementType,
}

fn prepare_statement(statement: &String) -> Option<Statement> {
    if statement.starts_with("insert") {
        return Some(Statement{type_ : StatementType::Insert})
    }
    else if statement.starts_with("select") {
        return Some(Statement{type_ : StatementType::Select})
    } else {
        return None
    }
}

fn execute_statement(statement: Statement) {
    match statement.type_ {
        StatementType::Insert => println!("This is where we would do an insert."),
        StatementType::Select => println!("This is where we would do a select."),
    }

}

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

fn do_meta_comand(command: &String) -> MetaCommandResult {
    if command == ".exit" {
        exit(0);
    } else {
        return MetaCommandResult::UnrecognizedCommand;
    }
}

fn read_prompt() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Could not read input!");
    buffer.pop();
    return buffer;
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().expect("Could not write output!");
}