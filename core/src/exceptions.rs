use crate::lexer::Symbol;
use backtrace::{Backtrace, Frame};
use colored::Colorize;

pub struct Exception {
    pub idx: usize,
    pub errtype: String,
    pub message: String,
    pub coreframes: Vec<Frame>,
}
impl Exception {
    pub fn new(idx: usize, errtype: &str, message: &str) -> Exception {
        let mut frames = vec![];
        backtrace::trace(|frame| {
            frames.push(frame.clone());
            true // keep going to the next frame
        });
        Exception {
            idx,
            errtype: errtype.into(),
            message: message.into(),
            coreframes: frames,
        }
    }
    pub fn unexpected_symbol(idx: usize, symbol: Symbol, allowed: Vec<Symbol>) -> Exception {
        Exception::new(
            idx,
            "UnexpectedSymbolException",
            &format!(
                "Expected {}: {}, but got the symbol {:?}",
                if allowed.len() > 1 {
                    "one of the following symbols"
                } else {
                    "the symbol"
                },
                allowed
                    .iter()
                    .fold(String::new(), |acc, x| acc + ", " + &x.display_name()),
                symbol
            ),
        )
    }
    pub fn unexpected_name(idx: usize, symbol: Symbol) -> Exception {
        Exception::new(
            idx,
            "UnexpectedNameException",
            &format!("Name {:?} is undefined ", symbol),
        )
    }
    pub fn fmt(&self, input: &String) -> String {
        let mut i = 0;
        let mut lines = 0;
        let mut offset = 0;
        while i < self.idx {
            if input.chars().nth(i).unwrap() == '\n' {
                lines += 1;
                offset = 0;
            }
            offset += 1;
            i += 1;
        }
        let allines: Vec<&str> = input.lines().collect();

        let line1 = format!(
            "      \"{}\"     {}",
            allines[lines].truecolor(255, 255, 255),
            format!(
                "at line {}, col {}",
                lines.to_string().truecolor(255, 255, 255),
                offset.to_string().truecolor(255, 255, 255)
            )
        );
        let line2 = format!(
            "      {}{}      {}",
            " ".repeat(offset - 1),
            "^".bright_red(),
            self.errtype.purple().bold(),
        );
        let line3 = format!(
            "{} {}",
            "ERROR:".red().bold(),
            self.message.bright_purple().bold()
        );

        let dasheslen = line3.len() / 2;
        // dbg!(dasheslen);

        format!(
            "{}\n{}\n{}\n{}\n{}",
            "-".repeat(dasheslen).red(),
            line1,
            line2,
            line3,
            "-".repeat(dasheslen).red()
        )
    }
    pub fn trace(&self) -> String {
        let mut out: String = String::new();

        for frame in &self.coreframes {
            // Resolve this instruction pointer to a symbol name
            backtrace::resolve_frame(frame, |symbol| {
                if let Some(filename) = symbol.filename() {
                    // REMOVE BEFORE PUSHING
                    if filename.starts_with("/home/ce/Documents/GitHub") {
                        if let Some(num) = symbol.lineno() {
                            out.push_str(&format!(
                                "{}:{}\n",
                                filename.to_str().unwrap(),
                                &num.to_string()
                            ));
                        }
                    }
                }
            });
        }
        out
    }
}
