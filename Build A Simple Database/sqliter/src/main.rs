use std::io::{self, Write};

enum Command {
    Exit,
    NonCommand,
    Unrecognized
}

enum StatementType {
    Insert,
    Select,
    Unrecognized
}

struct Statement {
    statement_type: StatementType,
    unprepared: String
}

impl Command {
    fn from_str(s: &str) -> Command {
        match s {
            command if !command.starts_with(".") => Command::NonCommand,
            ".exit" => Command::Exit,
            _ => Command::Unrecognized,
        }
    }
}

impl Statement {
    fn prepare(s: &str) -> Self {
        match s {
            statement if statement.starts_with("insert") => Self { unprepared: statement.to_string(), statement_type: StatementType::Insert },
            statement if statement.starts_with("select") => Self { unprepared: statement.to_string(), statement_type: StatementType::Select },
            _ => Self { unprepared: s.to_string(), statement_type: StatementType::Unrecognized },
        }
    }

    fn execute(&self) {
        match self.statement_type {
            StatementType::Insert => print!("This is where we would do an insert.\n"),
            StatementType::Select => print!("This is where we would do a select.\n"),
            StatementType::Unrecognized => print!("Unrecognized keyword at start of \"{}\"\n", self.unprepared.trim()),
        }
    }
}

fn prepare_and_execute_statement(input: &str) {
    let statement = Statement::prepare(&input);

    statement.execute()
}

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn main() {
    println!("SQLiter version 0.1.0");

    loop {
        let mut input = String::new();

        print_prompt();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match Command::from_str(&input.trim().to_lowercase()) {
            Command::Exit => break,
            Command::Unrecognized => print!("Unrecognized command: {}", input),
            Command::NonCommand => prepare_and_execute_statement(&input),
        }
    }
}
