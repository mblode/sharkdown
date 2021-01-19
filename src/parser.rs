use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Default)]
pub struct Parser {
    pub file_name: String,
    pub tokens: Vec<String>,
}

impl Parser {
    pub fn parse(&mut self) -> Vec<String> {
        let input_file_name = Path::new(&self.file_name);
        let input_file = File::open(&input_file_name).expect("[ ERROR ] Failed to open file!");
        let tokens = self.read_file(input_file).unwrap();

        tokens
    }

    pub fn default(file_name: &str) -> Self {
        let tokens: Vec<String> = Vec::new();

        Self {
            file_name: file_name.to_string(),
            tokens,
        }
    }

    fn read_file(&mut self, input_file: File) -> Result<Vec<String>, std::io::Error> {
        let mut _ptag: bool = false;
        let mut _htag: bool = false;

        let mut tokens: Vec<String> = Vec::new();

        let reader = BufReader::new(input_file);

        for line in reader.lines() {
            let line_contents = line.unwrap();

            let output_line = parse_row(&line_contents);
            println!("{}", &output_line);
            tokens.push(output_line.to_string());
        }

        Ok(tokens)
    }
}

fn parse_row(input: &str) -> String {
    let mut new_input = input.to_string();

    let rules: Vec<(&str, &str)> = vec![
        (r"^###### (?P<h>.*)", "<h6>$h</h6>"),              // Heading 6
        (r"^##### (?P<h>.*)", "<h5>$h</h5>"),               // Heading 5
        (r"^#### (?P<h>.*)", "<h4>$h</h4>"),                // Heading 4
        (r"^### (?P<h>.*)", "<h3>$h</h3>"),                 // Heading 3
        (r"^## (?P<h>.*)", "<h2>$h</h2>"),                  // Heading 2
        (r"^# (?P<h>.*)", "<h1>$h</h1>"),                   // Heading 1
        (r"^\*> (?P<h>.*)", "<blockquote>$h</blockquote>"), // Blockquote
        (r"\*\*(?P<h>.*)\*\*", "<b>$h</b>"),                // Bold
        (r"\*(?P<h>.*)\*", "<i>$h</i>"),                    // Italic
        (
            r"^!\[(?P<a>.)\]\((?P<s>.)\)",
            "<img alt=\"$a\" src=\"$s\" />",
        ), // Image
        (r"\[(?P<a>.)\]\((?P<s>.)\)", "<a href=\"$a\">$</a>"), // Link
        (r"\n(?P<h>.*)\n", "<p>$h</p>"),                    // Paragraph
        (r"\n", "<br />"),                                  // Break
    ];

    for rule in rules.iter() {
        let re = Regex::new(rule.0).unwrap();

        let result = re.replace(&new_input, rule.1);
        new_input = result.to_string();
    }

    new_input
}
