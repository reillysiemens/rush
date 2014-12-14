fn main() {
    loop {
        prompt::display_prompt();
        process_line(prompt::get_input());
        // TODO: Figure out how to exit! (Either by typing `exit [someval?]` or ^D?)
        // In prompt::get_input, the value in .expect() is what rush "panick[s] at"
        // when ^D is sent.
    }
}

fn process_line(s: String) {
    let split_vec: Vec<&str> = s.split_str(" ").filter(|p| !p.is_empty()).collect();
    execute::run(split_vec);
}

mod prompt {
    use std::io;

    pub fn display_prompt() {
        let prompt_string = "$ ";
        print!("{}", prompt_string);
    }

    pub fn get_input() -> String {
        let input = io::stdin()
                        .read_line()
                        .ok()
                        .expect("Failed to read line");

        input.trim().to_string()
    }
}

mod execute {
    use std::io::Command;

    pub fn run(command_line: Vec<&str>) {
        let mut cl = command_line.clone();
        let command = cl[0].to_string();
        cl.remove(0);
        let output = match Command::new(command).args(cl.as_slice()).output() {
            Ok(output) => output,
            Err(e) => panic!("Failed to execute process: {}", e),
        };

        println!("Process exited with {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(output.output.as_slice()));
        println!("stderr: {}", String::from_utf8_lossy(output.error.as_slice()));
    }
}
