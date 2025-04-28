use std::fmt;
use crate::logging::*;
use crate::callable::*;
use crate::oop::*;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
  Number(f64),
  String(String),
  Boolean(bool),
  Callable(Rc<RefCell<Box<dyn LoxCallable>>>),
  Class(LoxClass),
  Instance(Rc<RefCell<LoxInstance>>),
  Nil,
}

impl LoxValue {
  pub fn as_callable(&mut self) -> Option<Rc<RefCell<Box<dyn LoxCallable>>>> {
    match self {
      LoxValue::Callable(c) => Some(c.clone()),
      LoxValue::Class(c) => Some(Rc::new(RefCell::new(Box::new(c.clone())))),
      _ => None,
    }
  }
}

// Implementing Display for custom formatting
impl fmt::Display for LoxValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          LoxValue::Number(n) => write!(f, "{}", n),
          LoxValue::String(s) => write!(f, "{}", s),
          LoxValue::Boolean(b) => write!(f, "{}", b),
          LoxValue::Callable(c) => write!(f, "{:?}", c),
          LoxValue::Class(c) => write!(f, "{}", c),
          LoxValue::Instance(c) => write!(f, "{}", c.borrow_mut()),
          LoxValue::Nil => write!(f, "nil"),
      }
  }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum TokenType {
  // Single-character tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // One or two character tokens
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  Identifier,
  String,
  Number, // The number literal is always in text format from the lexer
                  // Some string -> int or string -> float conversion will take place eventually

  // Keywords
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub token: String,
  pub literal: LoxValue,
  pub line: usize,
}

impl Token {
  pub fn new(token_type: TokenType, token: String, literal: LoxValue, line: usize) -> Self {
    Self { token_type, token, literal, line }
  }
}

impl Hash for Token {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.token.hash(state);
    self.token_type.hash(state);
    self.line.hash(state);
  }
}

impl Eq for Token {}  // Eq is implemented since PartialEq is valid

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TOKEN_TYPE: {:?}, TOKEN: {}, LINE: {}", self.token_type, self.token, self.line)
  }
}

pub struct Lexer {
  source: String,
  tokens: Vec<Token>,
  start: u32,
  current: u32,
  line: usize,
}

impl Lexer {
  pub fn new(source: String) -> Self {
      Self {
          source,
          tokens: Vec::new(),
          start: 0,
          current: 0,
          line: 1,
      }
  }

  // may need to make the tokens mutable...
  pub fn scan_tokens(&mut self) -> &Vec<Token> {
      while !self.is_at_end() {
          self.start = self.current;
          self.scan_token();
      }

      self.tokens.push(Token::new(TokenType::Eof, String::from(""), LoxValue::Nil, self.line as usize));
      &self.tokens
  }

  fn scan_token(&mut self) {
      let c = self.advance();

      match c {
          '(' => self.add_token(TokenType::LeftParen),
          ')' => self.add_token(TokenType::RightParen),
          '{' => self.add_token(TokenType::LeftBrace),
          '}' => self.add_token(TokenType::RightBrace),
          ',' => self.add_token(TokenType::Comma),
          '.' => self.add_token(TokenType::Dot),
          '-' => self.add_token(TokenType::Minus),
          '+' => self.add_token(TokenType::Plus),
          ';' => self.add_token(TokenType::Semicolon),
          '*' => self.add_token(TokenType::Star),
          '!' => 
            if self.match_char('=') {
              self.add_token(TokenType::BangEqual);
            } else {
              self.add_token(TokenType::Bang);
            },
          '=' => 
            if self.match_char('=') {
              self.add_token(TokenType::EqualEqual);
            } else {
              self.add_token(TokenType::Equal);
            },
          '<' => 
            if self.match_char('=') {
              self.add_token(TokenType::LessEqual);
            } else {
              self.add_token(TokenType::Less);
            },
          '>' => 
            if self.match_char('=') {
              self.add_token(TokenType::GreaterEqual);
            } else {
              self.add_token(TokenType::Greater);
            },
          '/' => {
              if self.match_char('/') {
                  while self.peek() != '\n' && !self.is_at_end() {
                      self.advance();
                  }
              } else {
                  self.add_token(TokenType::Slash);
              }
          },
          ' ' | '\r' | '\t' => (),
          '\n' => self.line += 1,
          '"' => self.string(),
          _ => {
              if c.is_digit(10) {
                  self.number();
              } else if c.is_alphabetic() {
                  self.identifier();
              } else {
                error_at_line(self.line, "Unexpected character.");
              }
          }
      }
  }

  fn add_token(&mut self, token_type: TokenType) {
      let text = self.source[self.start as usize..self.current as usize].to_string();
      self.tokens.push(Token::new(token_type, text, LoxValue::Nil, self.line as usize));
  }

  fn add_token_literal(&mut self, token_type: TokenType, literal: LoxValue) {
    let text = self.source[self.start as usize..self.current as usize].to_string();
    self.tokens.push(Token::new(token_type, text, literal, self.line as usize));
  }

  fn match_char(&mut self, expected: char) -> bool {
      if self.is_at_end() {
          return false;
      }

      if self.source.chars().nth(self.current as usize).unwrap() != expected {
          return false;
      }

      self.current += 1;
      true
  }

  fn is_at_end(&self) -> bool {
      self.current >= self.source.len() as u32
  }

  fn peek(&self) -> char {
      if self.is_at_end() {
          return '\0';
      }

      self.source.chars().nth(self.current as usize).unwrap()
  }

  fn string(&mut self) {
      while self.peek() != '"' && !self.is_at_end() {
          if self.peek() == '\n' {
              self.line += 1;
          }
          self.advance();
      }

      if self.is_at_end() {
          eprintln!("Unterminated string.");
          return;
      }

      self.advance();

      let value = self.source[self.start as usize + 1..self.current as usize - 1].to_string();
      self.add_token_literal(TokenType::String, LoxValue::String(value));
  }

  fn number(&mut self) {
      while self.peek().is_digit(10) {
          self.advance();
      }

      if self.peek() == '.' && self.peek_next().is_digit(10) {
          self.advance();

          while self.peek().is_digit(10) {
              self.advance();
          }
      }

      let value = self.source[self.start as usize..self.current as usize].parse::<f64>().expect("Failed to parse number");
      self.add_token_literal(TokenType::Number, LoxValue::Number(value));
  }

  fn peek_next(&self) -> char {
      if self.current + 1 >= self.source.len() as u32 {
          return '\0';
      }

      self.source.chars().nth((self.current + 1) as usize).unwrap()
  }

  fn identifier(&mut self) {
      while self.peek().is_alphanumeric() {
          self.advance();
      }

      let text = self.source[self.start as usize..self.current as usize].to_string();
      let token_type = match text.as_str() {
          "and" => TokenType::And,
          "class" => TokenType::Class,
          "else" => TokenType::Else,
          "false" => TokenType::False,
          "for" => TokenType::For,
          "fun" => TokenType::Fun,
          "if" => TokenType::If,
          "nil" => TokenType::Nil,
          "or" => TokenType::Or,
          "print" => TokenType::Print,
          "return" => TokenType::Return,
          "super" => TokenType::Super,
          "this" => TokenType::This,
          "true" => TokenType::True,
          "var" => TokenType::Var,
          "while" => TokenType::While,
          _ => TokenType::Identifier,
      };

      self.add_token(token_type);
  }

  fn advance(&mut self) -> char {
    let to_return: char = self.source.chars().nth(self.current as usize).unwrap();
    self.current += 1;
    to_return
  }
}
