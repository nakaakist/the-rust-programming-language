use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum InputKind {
    NewRecord,
    ShowList,
}

#[derive(Debug)]
struct Record {
    name: String,
    department: String,
}

#[derive(Debug)]
struct InputData {
    kind: InputKind,
    record: Option<Record>,
}

fn parse_input(input: String) -> Result<InputData, String> {
    let mut words = input.split_whitespace();
    match words.next() {
        Some("Show") => {
            return Ok(InputData {
                kind: InputKind::ShowList,
                record: None,
            })
        }
        Some("Add") => {}
        _ => return Err("First word must be 'Add' or 'Show'".to_string()),
    }

    let name = words.next();

    match words.next() {
        Some("to") => {}
        _ => return Err("Third word must be 'to'".to_string()),
    }

    let department = words.next();

    match [name, department] {
        [Some(name), Some(department)] => Ok(InputData {
            kind: InputKind::NewRecord,
            record: Some(Record {
                name: name.to_string(),
                department: department.to_string(),
            }),
        }),
        _ => Err("Failed to parse name or department".to_string()),
    }
}

fn main() {
    let mut records = HashMap::<String, Vec<String>>::new();

    loop {
        println!(
            "Please input 'Add xxx (person name) to yyy (department name)', or 'Show' to see list"
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input_data = parse_input(input);

        match input_data {
            Ok(InputData {
                kind: InputKind::NewRecord,
                record: Some(r),
            }) => {
                let department = (&r.department).to_string();
                let name = (&r.name).to_string();
                let names = records.entry(department).or_insert(Vec::new());
                names.push(name);
            }
            Ok(InputData {
                kind: InputKind::ShowList,
                record: _,
            }) => println!("{:#?}", records),
            Err(msg) => println!("Error: {}", msg),
            _ => panic!("Unknown error"),
        }
    }
}
