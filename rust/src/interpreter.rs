use crate::ast::*;
use crate::lexer::*;
use crate::environment::*;
use crate::oop::*;
use crate::stl::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

pub struct Interpreter {
  pub globals: Rc<RefCell<Environment>>,
  environment: Rc<RefCell<Environment>>,
  locals: HashMap<Expr, usize>,
}

// This is the error type for "RunTime" errors in our itnerpreter
#[derive(Debug)]
pub enum InterpreterErrorType {
  FatalError,
  ReturnValue(Box<LoxValue>),
}

#[derive(Debug)]
pub struct InterpreterError {
  pub final_token: Token,
  pub message: String,
  pub error_type: InterpreterErrorType,
}

impl InterpreterError {
  pub fn new(final_token: Token, message: String) -> Self {
    Self { final_token, message, error_type: InterpreterErrorType::FatalError }
  }

  pub fn new_with_type(final_token: Token, message: String, error_type: InterpreterErrorType) -> Self {
    Self { final_token, message, error_type }
  }

  pub fn print(&self) {
    eprintln!("Error at token: {}. INFO: {} ", &self.final_token, &self.message);
  }
}

impl Interpreter {
  pub fn new() -> Self {
    let mut globals = Environment::new();
    globals.define(
      "clock".to_string(),
      LoxValue::Callable(Rc::new(RefCell::new(Box::new(ClockCallable::new())))),
    );
    let globals_ref = Rc::new(RefCell::new(globals));
    Self { globals: globals_ref.clone(), environment: globals_ref.clone(), locals: HashMap::new() }
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

  pub fn resolve(&mut self, expr: Expr, depth: usize) {
    self.locals.insert(expr, depth);
  }

  pub fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
    match stmt {
      Stmt::Block(stmt) => self.visitBlockStmt(stmt),
      Stmt::Expression(expr) => self.visitExpressionStmt(expr),
      Stmt::Print(expr) => self.visitPrintStmt(expr),
      Stmt::Return(expr) => self.visitReturnStmt(expr),
      Stmt::Var(expr) => self.visitVarStmt(expr),
      Stmt::Fun(expr) => self.visitFunStmt(expr),
      Stmt::If(expr) => self.visitIfStmt(expr),
      Stmt::While(expr) => self.visitWhileStmt(expr),
      Stmt::For(expr) => self.visitForStmt(expr),
      Stmt::Class(expr) => self.visitClassStmt(expr),
    }
  }

  pub fn execute_block(&mut self, block: &BlockStmt, env: Rc<RefCell<Environment>>) -> Result<(), InterpreterError> {
    // Take current environment (replacing it with a dummy one temporarily)
    let prev = self.environment.clone();
    let enclosed_env = Environment::new_enclosed(Rc::clone(&env));
    self.environment = Rc::new(RefCell::new(enclosed_env));
    for statement in &*block.statements {
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
      Expr::Call(expr) => self.visitCallExpr(expr),
      Expr::Get(expr) => self.visitGetExpr(expr),
      Expr::Set(expr) => self.visitSetExpr(expr),
      Expr::Grouping(expr) => self.visitGroupingExpr(expr),
      Expr::Literal(expr) => self.visitLiteralExpr(expr),
      Expr::Unary(expr) => self.visitUnaryExpr(expr),
      Expr::Variable(expr) => self.visitVariableExpression(expr),
      Expr::Logical(expr) => self.visitLogicalExpression(expr),
      Expr::This(expr) => self.visitThisExpression(expr),
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

  pub fn look_up_variable(&self, name: &Token, expr: &Expr) -> Result<LoxValue, InterpreterError> {
    if let Some(distance) = self.locals.get(expr) {
      let value = self.environment.borrow().get_at(*distance, &name.token);
      match value {
        Ok(v) => {
          return Ok(v);
        }
        Err(msg) => return Err(InterpreterError::new(
          name.clone(),
          msg,
        )),
      }
    }
    let value = self.globals.borrow().get(&name.token.clone());
    match value {
      Ok(v) => Ok(v),
      Err(msg) => Err(InterpreterError::new(
        name.clone(),
        msg,
      )),
    }
  }
}

impl ExprVisitor<Result<LoxValue, InterpreterError>> for Interpreter {
  fn visitLiteralExpr(&mut self, expr: &LiteralExpr) -> Result<LoxValue, InterpreterError> {
    match &expr.literal {
      LoxValue::Nil => Ok(LoxValue::Nil),
      LoxValue::Number(n) => Ok(LoxValue::Number(n.clone())),
      LoxValue::String(s) => Ok(LoxValue::String(s.clone())),
      LoxValue::Callable(c) => Ok(LoxValue::Callable(c.clone())),
      LoxValue::Boolean(b) => Ok(LoxValue::Boolean(b.clone())),
      LoxValue::Class(c) => Ok(LoxValue::Class(c.clone())),
      LoxValue::Instance(c) => Ok(LoxValue::Instance(c.clone())),
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

  fn visitCallExpr(&mut self, expr: &CallExpr) -> Result<LoxValue, InterpreterError> {
    let mut callee = self.evaluate(&expr.callee)?;
    let mut arguments = Vec::new();
    for arg in &expr.arguments {
      arguments.push(self.evaluate(arg)?);
    }
    let callable = callee.as_callable();
    match callable {
      Some(callable) => {
        // println!("Callable: {:?}", callable);
        if arguments.len() != callable.borrow().arity() {
          return Err(InterpreterError::new(
            expr.paren.clone(),
            format!(
              "Expected {} arguments but got {}.",
              callable.borrow().arity(),
              arguments.len()
            ),
          ));
        }
        let res = callable.borrow_mut().call(self, arguments.clone());
        Ok(*res)
      }
      _ => Err(InterpreterError::new(
        expr.paren.clone(),
        format!("{} is not callable.", callee),
      )),
    }
  }

  fn visitGetExpr(&mut self, expr: &GetExpr) -> Result<LoxValue, InterpreterError> {
    let object = self.evaluate(&expr.object)?;
    if let LoxValue::Instance(instance) = object {
      match LoxInstance::get(instance, &expr.name.token) {
        Ok(value) => return Ok(value),
        Err(msg) => return Err(InterpreterError::new(
          expr.name.clone(),
          msg,
        )),
      }
    }
    Err(InterpreterError::new(
      expr.name.clone(),
      format!("{} is not an instance.", object),
    ))
  }

  fn visitSetExpr(&mut self, expr: &SetExpr) -> Result<LoxValue, InterpreterError> {
    let object = self.evaluate(&expr.object)?;
    if let LoxValue::Instance(mut instance) = object {
      let value = self.evaluate(&expr.value)?;
      instance.borrow_mut().set(expr.name.token.clone(), value.clone());
      return Ok(value);
    }
    Err(InterpreterError::new(
      expr.name.clone(),
      format!("{} is not an instance.", object),
    ))
  }

  fn visitVariableExpression(&mut self, expr: &VariableExpr) -> Result<LoxValue, InterpreterError> {
    self.look_up_variable(&expr.name, &Expr::Variable(expr.clone()))
  }

  fn visitAssignExpression(&mut self, expr: &AssignExpr) -> Result<LoxValue, InterpreterError> {
    let value = self.evaluate(&expr.value)?;
    let distance = self.locals.get(&Expr::Assign(expr.clone()));
    if let Some(distance) = distance {
      let res = self.environment.borrow_mut().assign_at(*distance, expr.name.token.clone(), value.clone());
      if let Err(msg) = res {
        return Err(InterpreterError::new(
          expr.name.clone(),
          msg,
        ));
      }
    } else {
      let res = self.globals.borrow_mut().assign(expr.name.token.clone(), value.clone());
      if let Err(msg) = res {
        return Err(InterpreterError::new(
          expr.name.clone(),
          msg,
        ));
      }
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

  fn visitThisExpression(&mut self, expr: &ThisExpr) -> Result<LoxValue, InterpreterError> {
    let value = self.look_up_variable(&expr.keyword, &Expr::This(expr.clone()))?;
    Ok(value)
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

    self.execute_block(stmt, self.environment.clone())?;
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

  fn visitReturnStmt(&mut self, stmt: &RetStmt) -> Result<(), InterpreterError> {
    let mut value = LoxValue::Nil;
    if let Some(val) = &stmt.value {
      value = self.evaluate(val)?;
    }
    // Weird but following Robby nystrom for now...
    // We will catch 
    return Err(InterpreterError::new_with_type(stmt.keyword.clone(), 
                                               format!("Return value: {:?}", value), 
                                               InterpreterErrorType::ReturnValue(Box::new(value))));
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

  fn visitFunStmt(&mut self, stmt: &FunStmt) -> Result<(), InterpreterError> {
    let function = LoxFunction::new(Rc::new(stmt.clone()), self.environment.clone(), false);
    self.environment.borrow_mut().define(stmt.name.token.clone(), LoxValue::Callable(Rc::new(RefCell::new(Box::new(function)))));
    Ok(())
  }

  fn visitClassStmt(&mut self, stmt: &ClassStmt) -> Result<(), InterpreterError> {
    self.environment.borrow_mut().define(stmt.name.token.clone(), LoxValue::Nil);
    let mut methods = HashMap::new();
    for method in &stmt.methods {
      let function = LoxFunction::new(Rc::new(method.clone()), self.environment.clone(), false);
      methods.insert(method.name.token.clone(), function);
    }
    let klass = LoxValue::Class(LoxClass::new(stmt.name.token.clone(), methods));
    match self.environment.borrow_mut().assign(stmt.name.token.clone(), klass) {
      Ok(_) => {}
      Err(msg) => return Err(InterpreterError::new(
        stmt.name.clone(),
        msg,
      )),
    }
    Ok(())
  }
}
