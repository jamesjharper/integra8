use std::io::Write;
use std::mem;

pub struct Prefix {
    line: Option<String>,
    next_line: Option<String>,
}

impl Prefix {
    pub fn next_with(line: impl Into<String>) -> Self {
        Self {
            line: None,
            next_line: Some(line.into()),
        }
    }

    pub fn with(line: impl Into<String>) -> Self {
        Self {
            line: Some(line.into()),
            next_line: None,
        }
    }

    pub fn then_next(self, line: impl Into<String>) -> Self {
        Self {
            line: self.line,
            next_line: Some(line.into()),
        }
    }
}

pub struct PrefixedTextWriter<W: Write> {
    current_prefix: Vec<String>,
    next_prefix: Option<Vec<String>>,
    writer: W,
}

impl<W: Write> PrefixedTextWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            current_prefix: vec![],
            next_prefix: None,
            writer: writer,
        }
    }

    pub fn push(&mut self, p: Prefix) {
        if let Some(next) = p.next_line {
            self.push_next_ln(next);
        }
        match p.line {
            Some(cur) => self.push_cur_ln(cur),
            // if no current line was given, pad with empty string as
            // this will ensure pop still work correctly
            None => self.push_cur_ln("".to_string()),
        }
    }

    pub fn pop(&mut self) {
        self.current_prefix.pop();
        if let Some(ref mut n) = &mut self.next_prefix {
            n.pop();
        }
    }

    pub fn writeln<D: std::fmt::Display>(&mut self, display: D) -> std::io::Result<()> {
        writeln!(self.writer, "{}{}", self.current_prefix.concat(), display)?;
        self.move_next_line();
        Ok(())
    }

    pub fn write_newline(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "{}", &self.current_prefix.concat())?;
        self.move_next_line();
        Ok(())
    }

    fn move_next_line(&mut self) {
        if let Some(next) = mem::take(&mut self.next_prefix) {
            self.current_prefix = next;
        }
    }

    fn push_cur_ln(&mut self, current: String) {
        self.current_prefix.push(current);
    }

    fn push_next_ln(&mut self, next: String) {
        if let Some(ref mut n) = &mut self.next_prefix {
            n.push(next);
        } else {
            let mut n = self.current_prefix.clone();
            n.push(next);
            self.next_prefix = Some(n);
        }
    }
}
