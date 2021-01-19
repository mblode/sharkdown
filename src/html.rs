use std::{fs::File, io::Write, path::Path};

#[derive(Default)]
pub struct Html {
    file_name: String,
}

impl Html {
    pub fn output(file_name: &str, tokens: Vec<String>) -> Result<(), std::io::Error> {
        let output_file_name = Path::new(file_name);

        let mut output_file =
            File::create(output_file_name).expect("[ ERROR ] Could not create output file!");

        for line in &tokens {
            output_file
                .write_all(line.as_bytes())
                .expect("[ ERROR ] Could not write to output file!");
        }

        println!("[ INFO ] Parsing complete!");

        Ok(())
    }
}
