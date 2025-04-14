use crate::callable::*;
use crate::environment::Environment;
use core::fmt;
use std::any::Any;
use crate::lexer::*;
use crate::interpreter::*;
use crate::ast::*;
use std::rc::Rc;
use std::cell::RefCell;

///////////// The Lox STL ///////////////
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct ClockCallable {}

impl ClockCallable {
  pub fn new() -> Self {
    Self {}
  }
}
impl LoxCallable for ClockCallable {
  fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> Box<dyn Any> {
    let time = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("Time went backwards")
      .as_secs_f64();
    Box::new(LoxValue::Number(time))
  }

  fn arity(&self) -> usize {
    0
  }

  fn box_clone(&self) -> Box<dyn LoxCallable> {
    Box::new(ClockCallable::new())
  }
}

#[derive(Clone, Debug)]
pub struct InterpretedFunction {
  pub declaration: FunStmt,
}

impl InterpretedFunction {
  pub fn new(declaration: FunStmt) -> Self {
    Self { declaration }
  }
}

impl LoxCallable for InterpretedFunction {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Box<dyn Any> {
    let mut environment = Environment::new_enclosed(Rc::clone(&interpreter.globals));
    for (i, param) in self.declaration.params.iter().enumerate() {
      print!("defining {} as {}", param.token.clone(), arguments[i]);
      environment.define(param.token.clone(), arguments[i].clone());
    }
    let result = interpreter.execute_block(&self.declaration.body, Rc::new(RefCell::new(environment)));
    if let Err(e) = result {
      print!("Error in function call: {:?}", e);
      return Box::new(LoxValue::Nil);
    }
    Box::new(LoxValue::Nil)
  }

  fn arity(&self) -> usize {
    self.declaration.params.len()
  }

  fn box_clone(&self) -> Box<dyn LoxCallable> {
    Box::new(InterpretedFunction::new(self.declaration.clone()))
  }
}
