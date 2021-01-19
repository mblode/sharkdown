use clap::{load_yaml, App};

mod html;
mod parser;

pub use html::Html;
pub use parser::Parser;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    let parser = Parser::default(input_file).parse();
    Html::output(output_file, parser).unwrap();
}
