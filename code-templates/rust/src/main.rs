extern crate serde_json;

use serde_json::Value;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("should provide a JSON arg");
        std::process::exit(1);
    }

    let json_str = &args[1];
    let parsed_json: Result<Value, _> = serde_json::from_str(json_str);

    match parsed_json {
        Ok(arr) => {
            let result = solution::sum(arr[0], arr[1]);
            println!("{:?}", result);
        }
        Err(e) => {
            eprintln!("Error al parsear JSON: {}", e);
            std::process::exit(1);
        }
    }
}

pub mod solution;