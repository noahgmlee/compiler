use crate::ast::*;
use crate::lexer::*;
use crate::environment::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
  environment: Rc<RefCell<Environment>>,
}

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
    let env = Environment::new();
    Self { environment: Rc::new(RefCell::new(env)), }
  }

  pub fn interpret(&mut self, stmts: &Vec<Stmt>) {
    for stmt in stmts {
      let result = self.execute(stmt);
      match result {
        Ok(_) => {},
        Err(err) => {
          err.print();
        }
      }
    }
  }

  pub fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
    match stmt {
      Stmt::Block(stmt) => self.visitBlockStmt(stmt),
      Stmt::Expression(expr) => self.visitExpressionStmt(expr),
      Stmt::Print(expr) => self.visitPrintStmt(expr),
      Stmt::Var(expr) => self.visitVarStmt(expr),
      Stmt::If(expr) => self.visitIfStmt(expr),
      Stmt::While(expr) => self.visitWhileStmt(expr),
      Stmt::For(expr) => self.visitForStmt(expr),
    }
  }

  pub fn execute_block(&mut self, block: &BlockStmt) -> Result<(), InterpreterError> {
    // Take current environment (replacing it with a dummy one temporarily)
    let prev = Rc::clone(&self.environment);
    let enclosed_env = Environment::new_enclosed(Rc::clone(&self.environment));
    self.environment = Rc::new(RefCell::new(enclosed_env));
    for statement in &block.statements {
      let res = self.execute(statement);
      match res {
        Ok(_) => {},
        Err(err) => {
          self.environment = prev;
          return Err(err);
        }
      }
    }
    self.environment = prev;
    Ok(())
  }

  // Okay so visitor pattern doesn't really need to be implemented here...
  pub fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, InterpreterError> {
    match expr {
      Expr::Assign(expr) => self.visitAssignExpression(expr),
      Expr::Binary(expr) => self.visitBinaryExpr(expr),
      Expr::Grouping(expr) => self.visitGroupingExpr(expr),
      Expr::Literal(expr) => self.visitLiteralExpr(expr),
      Expr::Unary(expr) => self.visitUnaryExpr(expr),
      Expr::Variable(expr) => self.visitVariableExpression(expr),
      Expr::Logical(expr) => self.visitLogicalExpression(expr),
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

  fn visitVariableExpression(&mut self, expr: &VariableExpr) -> Result<LoxValue, InterpreterError> {
    let value = self.environment.borrow().get(&expr.name.token);
    match value {
      Ok(v) => Ok(v),
      Err(msg) => Err(InterpreterError::new(
        expr.name.clone(),
        msg,
      )),
    }
  }

  fn visitAssignExpression(&mut self, expr: &AssignExpr) -> Result<LoxValue, InterpreterError> {
    let value = self.evaluate(&expr.value)?;
    let is_ok = self.environment.borrow_mut().assign(expr.name.token.clone(), value.clone());
    if let Err(msg) = is_ok {
      return Err(InterpreterError::new(
        expr.name.clone(),
        msg,
      ));
    }
    Ok(value)
  }

  fn visitLogicalExpression(&mut self, expr: &LogicalExpr) -> Result<LoxValue, InterpreterError> {
    let left = self.evaluate(&expr.left)?;
    if expr.operator.token_type == TokenType::Or {
      if Interpreter::is_truthy(left.clone()) {
        return Ok(left);
      }
    } else {
      if !Interpreter::is_truthy(left.clone()) {
        return Ok(left);
      }
    }
    self.evaluate(&expr.right)
  }
}

impl StmtVisitor<Result<(), InterpreterError>> for Interpreter {
  fn visitForStmt(&mut self, stmt: &ForStmt) -> Result<(), InterpreterError> {
    if let Some(initializer) = &stmt.initializer {
      self.execute(initializer)?;
    }
    while match &stmt.condition {
      Some(condition) => Interpreter::is_truthy(self.evaluate(condition)?),
      None => true,
    } {
      self.execute(&stmt.body)?;
      if let Some(increment) = &stmt.increment {
        self.evaluate(increment)?;
      }
    }
    Ok(())
  }

  fn visitWhileStmt(&mut self, stmt: &WhileStmt) -> Result<(), InterpreterError> {
    while Interpreter::is_truthy(self.evaluate(&stmt.condition)?) {
      self.execute(&stmt.body)?;
    }
    Ok(())
  }

  fn visitIfStmt(&mut self, stmt: &IfStmt) -> Result<(), InterpreterError> {
    let condition = self.evaluate(&stmt.condition)?;
    if Interpreter::is_truthy(condition) {
      self.execute(&stmt.then_branch)?;
    } else if let Some(else_branch) = &stmt.else_branch {
      self.execute(else_branch)?;
    }
    Ok(())
  }

  fn visitBlockStmt(&mut self, stmt: &BlockStmt) -> Result<(), InterpreterError> {
    self.execute_block(stmt)?;
    Ok(())
  }

  fn visitExpressionStmt(&mut self, stmt: &ExprStmt) -> Result<(), InterpreterError> {
    self.evaluate(&stmt.expression)?;
    Ok(())
  }

  fn visitPrintStmt(&mut self, stmt: &PrintStmt) -> Result<(), InterpreterError> {
    let value = self.evaluate(&stmt.expression)?;
    println!("{}", value);
    Ok(())
  }

  fn visitVarStmt(&mut self, stmt: &VarStmt) -> Result<(), InterpreterError> {
    let value = if let Some(initializer) = &stmt.initializer {
      self.evaluate(initializer)?
    } else {
      LoxValue::Nil
    };
    self.environment.borrow_mut().define(stmt.name.token.clone(), value);
    Ok(())
  }
}
