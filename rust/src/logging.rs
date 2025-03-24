use crate::lexer::*;

pub fn error_at_line(line: usize, message: &str) {
    report(line, "", message);
}

pub fn error_at_token(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.token), message);
    }
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}
