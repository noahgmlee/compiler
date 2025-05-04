use crate::lexer::*;
use crate::interpreter::*;

pub trait LoxCallable: std::fmt::Debug {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Box<LoxValue>;
  fn arity(&self) -> usize;
  fn box_clone(&self) -> Box<dyn LoxCallable>;
}

impl Clone for Box<dyn LoxCallable> {
  fn clone(&self) -> Box<dyn LoxCallable> {
    self.box_clone()
  }
}

impl PartialEq for Box<dyn LoxCallable> {
  fn eq(&self, other: &Self) -> bool {
    return true;
  }
}
