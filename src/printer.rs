use std::str::FromStr;

use crate::chatgpt::Role;

enum Colors {
    Blue,
    Red,
    Green,
}

impl Colors {
    fn get_color_prefix(&self) -> &str {
        match self {
            Colors::Blue => "\x1b[34m",
            Colors::Red => "\x1b[31m",
            Colors::Green => "\x1b[32m",
        }
    }
}


pub struct ConsolePrinter {
}

impl ConsolePrinter {
    fn print_message(&self, role: &str, content: &str) {
        let role = Role::from_str(role).expect("could not convert role");
        let role_str = role.to_string();
        let color_prefix = match role {
            Role::User => Colors::Blue.get_color_prefix(),
            Role::System => Colors::Green.get_color_prefix(),
            Role::Assistant => Colors::Red.get_color_prefix(),
        };
        println!("{}{}\x1b[0m: {}", color_prefix, role_str, content);
    }
}

pub struct MockPrinter {
    pub messages: Vec<(String, String)>,
}

#[allow(dead_code)]
impl MockPrinter {
    pub fn new() -> Self {
        MockPrinter { messages: Vec::new() }
    }

    fn print_message(&mut self, role: &str, content: &str) {
        self.messages.push((role.to_string(), content.to_string()));
    }
}


pub enum Printer<'a> {
    Console(ConsolePrinter),
    #[allow(dead_code)]
    Mock(&'a mut MockPrinter),
}
impl <'a> Printer<'a> {
    pub fn print_message(&mut self, role: &str, content: &str) {
        match self {
            Printer::Console(console_printer) => console_printer.print_message(role, content),
            Printer::Mock(mock_printer) => mock_printer.print_message(role, content),
        }
    }
}
