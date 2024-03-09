pub struct PrintGlobalState {
    erroneous: bool,
    file_name_printed: bool,
}

impl PrintGlobalState {
    pub fn new() -> Self {
        PrintGlobalState {
            erroneous: false,
            file_name_printed: false,
        }
    }

    pub fn reset(&mut self) {
        self.erroneous = false;
        self.file_name_printed = false;
    }

    pub fn has_encountered_error(&self) -> bool {
        self.erroneous
    }

    pub fn print_file_name(&mut self, file_name: &str) {
        if !self.file_name_printed {
            println!(
                "\x1b[36;1mIn file \x1b[33m\"\x1b[0m{}\x1b[33m\"\x1b[0m:",
                file_name
            );
        }
        self.file_name_printed = true;
    }

    pub fn error(&mut self, string: &str, line: usize, col: usize, file: &str) {
        self.print_file_name(file);

        self.erroneous = true;
        eprintln!("\x1b[32merror:\x1b[0m {}", string);
        self.print_file(line, col, file);
    }

    pub fn print_file(&self, line: usize, col: usize, file: &str) {
        let mut c = col;
        while file.chars().nth(c).unwrap_or('\n') != '\n' {
            c -= 1;
        }
        c += 1;

        let mut string = String::new();
        let mut x = file.chars().nth(c).unwrap_or('\n');

        while x != '\n' {
            string.push(x);
            c += 1;
            x = file.chars().nth(c).unwrap_or('\n');
        }

        println!("{g:>width$}|", g = ' ', width = line.to_string().len() + 1);
        println!("\x1b[34;1m{} | {}\x1b[0m", line, string);
        println!("{g:>width$}|", g = ' ', width = line.to_string().len() + 1);
    }
}