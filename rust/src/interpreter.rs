use std::backtrace;

use crate::ast::*;
use crate::lexer::*;

pub struct Interpreter {}

// This is the error type for "RunTime" errors in our itnerpreter
pub struct InterpreterError {
  pub final_token: Token,
  pub message: String,
}

impl InterpreterError {
  pub fn new(final_token: Token, message: String) -> Self {
    Self { final_token, message }
  }

  pub fn print(&self) {
    eprintln!("Error at token: {}. INFO: {} ", &self.final_token, &self.message);
  }
}

impl Interpreter {
  pub fn new() -> Self {
    Self {}
  }

  pub fn interpret(&mut self, expr: &Expr) {
    let result = self.evaluate(expr);
    match result {
      Ok(value) => {
        println!("{:?}", value);
      }
      Err(err) => {
        err.print();
      }
    }
  }

  // Okay so visitor pattern doesn't really need to be implemented here...
  pub fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, InterpreterError> {
    match expr {
      Expr::Binary(expr) => self.visitBinaryExpr(expr),
      Expr::Grouping(expr) => self.visitGroupingExpr(expr),
      Expr::Literal(expr) => self.visitLiteralExpr(expr),
      Expr::Unary(expr) => self.visitUnaryExpr(expr),
    }
  }

  pub fn is_truthy(value: LoxValue) -> bool {
    match value {
      LoxValue::Nil => false,
      LoxValue::Boolean(b) => b,
      _ => true,
    }
  }

  pub fn is_equal(left: &LoxValue, right: &LoxValue) -> bool {
    match (left, right) {
      (LoxValue::Nil, LoxValue::Nil) => true,
      (LoxValue::Number(l), LoxValue::Number(r)) => l == r,
      (LoxValue::String(l), LoxValue::String(r)) => l == r,
      (LoxValue::Boolean(l), LoxValue::Boolean(r)) => l == r,
      _ => false,
    }
  }

  pub fn not_a_number_error(expr: &Token, val: &LoxValue) -> InterpreterError {
    InterpreterError::new(
      expr.clone(),
      format!("{} {:?} must be a number.",
        expr.token,
        val),  
    )
  }

  pub fn not_numbers_error(expr: &Token, left: &LoxValue, right: &LoxValue) -> InterpreterError {
    InterpreterError::new(
      expr.clone(),
      format!("{} {:?} {:?} must be numbers.",
        expr.token,
        left,
        right),  
    )
  }

  pub fn not_numbers_or_strings_error(expr: &Token, left: &LoxValue, right: &LoxValue) -> InterpreterError {
    InterpreterError::new(
      expr.clone(),
      format!("{} {:?} {:?} must be numbers or strings.",
        expr.token,
        left,
        right),  
    )
  }
}

impl ExprVisitor<Result<LoxValue, InterpreterError>> for Interpreter {
  fn visitLiteralExpr(&mut self, expr: &LiteralExpr) -> Result<LoxValue, InterpreterError> {
    match &expr.literal {
      LoxValue::Nil => Ok(LoxValue::Nil),
      LoxValue::Number(n) => Ok(LoxValue::Number(n.clone())),
      LoxValue::String(s) => Ok(LoxValue::String(s.clone())),
      LoxValue::Boolean(b) => Ok(LoxValue::Boolean(b.clone())),
    }
  }

  fn visitGroupingExpr(&mut self, expr: &GroupingExpr) -> Result<LoxValue, InterpreterError> {
    return self.evaluate(&expr.expression);
  }

  fn visitUnaryExpr(&mut self, expr: &UnaryExpr) -> Result<LoxValue, InterpreterError> {
    let right = self.evaluate(&expr.right)?;
    match expr.operator.token_type {
      TokenType::Minus => {
        if let LoxValue::Number(n) = right {
          return Ok(LoxValue::Number(-n));
        } else {
          return Err(Interpreter::not_a_number_error(&expr.operator, &right));
        }
      }
      TokenType::Bang => {
        return Ok(LoxValue::Boolean(!Interpreter::is_truthy(right)));
      }
      _ => {}
    }
    Ok(LoxValue::Nil)
  }

  fn visitBinaryExpr(&mut self, expr: &BinaryExpr) -> Result<LoxValue, InterpreterError> {
    let left = self.evaluate(&expr.left)?;
    let right = self.evaluate(&expr.right)?;

    match expr.operator.token_type {
      TokenType::Plus => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Number(l + r));
        }
        if let (LoxValue::String(l), LoxValue::String(r)) = (&left, &right) {
          return Ok(LoxValue::String(format!("{}{}", l, r)));
        }
        return Err(Interpreter::not_numbers_or_strings_error(
          &expr.operator,
          &left,
          &right,
        ));
      }
      TokenType::Minus => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Number(l - r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      TokenType::Star => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Number(l * r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      TokenType::Slash => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Number(l / r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      TokenType::BangEqual => {
        return Ok(LoxValue::Boolean(!Interpreter::is_equal(&left, &right)));
      }
      TokenType::EqualEqual => {
        return Ok(LoxValue::Boolean(Interpreter::is_equal(&left, &right)));
      }
      TokenType::Greater => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Boolean(l > r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      TokenType::GreaterEqual => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Boolean(l >= r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      TokenType::Less => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Boolean(l < r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      TokenType::LessEqual => {
        if let (LoxValue::Number(l), LoxValue::Number(r)) = (&left, &right) {
          return Ok(LoxValue::Boolean(l <= r));
        } else {
          return Err(Interpreter::not_numbers_error(
            &expr.operator,
            &left,
            &right,
          ));
        }
      }
      _ => {}
    }

    Ok(LoxValue::Nil)
  }
}
