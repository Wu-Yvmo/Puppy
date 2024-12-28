use std::{env, fs};

use Puppy::{interpreter, parser};

fn main() {
    let launch_configurations = analyze_launch_configurations();
    let code = fs::read_to_string(launch_configurations.input_file).unwrap();
    let ast = parser::parse(&code);
    let mut interpreter = interpreter::Interpreter::create(&ast);
    interpreter.interpret(&ast);
}

struct LaunchConfigurations {
    input_file: String,
}

fn analyze_launch_configurations() -> LaunchConfigurations {
    let raw_launch_configurations: Vec<_> = env::args().collect();
    if raw_launch_configurations.len() != 2 {
        panic!("launch configurations error");
    }
    LaunchConfigurations{
        input_file: raw_launch_configurations[1].clone(),
    }
}