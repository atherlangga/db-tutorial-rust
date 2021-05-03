#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate serde_derive;

extern crate rustyline;

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    id: u32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    rows: std::vec::Vec<Row>,
}

enum Statement {
    Insert(Row),
    Select,
}

fn main() {
    let mut table = Table { rows: Vec::new() };

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline("db > ");
        match readline {
            Ok(raw_line) => {
                let line = raw_line.trim();
                if line.starts_with(".") {
                    let meta_command_result = do_meta_command(line);
                    match meta_command_result {
                        Err(err) => println!("Error: {:?}", err),
                        _ => (),
                    }
                } else {
                    let parse_statement_result = parse_statement(line);
                    match parse_statement_result {
                        Err(err) => println!("Error: {:?}", err),
                        Ok(statement) => {
                            let execution_result = execute_statement(&mut table, statement);
                            match execution_result {
                                Err(err) => println!("Error: {:?}", err),
                                Ok(()) => println!("Executed."),
                            }
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn do_meta_command(command: &str) -> Result<(), String> {
    if command == ".exit" {
        std::process::exit(0);
    } else {
        return Err(format!("Unrecognized command: {}", command));
    }
}

fn parse_statement(command: &str) -> Result<Statement, String> {
    if command.to_uppercase().starts_with("INSERT") {
        let (id, username, email) = scan_fmt!(
            &command.to_lowercase(),
            "insert {} {} {}",
            u32,
            String,
            String
        );

        if let None = id {
            return Err(format!("ID must be positive"))
        }

        if let (Some(id), Some(username), Some(email)) = (id, username, email) {
            return Ok(Statement::Insert(Row {
                id,
                username,
                email,
            }));
        }

        return Err(format!("Could not parse {}", command));
    } else if command.to_uppercase().starts_with("SELECT") {
        return Ok(Statement::Select);
    }

    return Err("Statement is neither INSERT nor SELECT".to_string());
}

fn execute_statement(table: &mut Table, statement: Statement) -> Result<(), String> {
    match statement {
        Statement::Insert(row) => return execute_insert(table, row),
        Statement::Select => return execute_select(table),
    }
}

fn execute_insert(table: &mut Table, row: Row) -> Result<(), String> {
    table.rows.push(row);
    return Ok(());
}

fn execute_select(table: &Table) -> Result<(), String> {
    for row in &table.rows {
        println!("({}, {}, {})", row.id, row.username, row.email);
    }
    return Ok(());
}
