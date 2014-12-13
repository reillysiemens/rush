fn main() {
    loop {
        prompt::display_prompt();
        you_typed(prompt::get_input());
    }
}

fn you_typed(s: String) {
    println!("You typed \"{}\"", s);
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
