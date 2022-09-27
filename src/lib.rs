fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {line}] Error{where_}: {message}")
}

pub fn error_line(line: usize, message: &str) {
    report(line, "", message);
}
