use crate::lexer::*;

trait ExprVisitor<R> {
  #[allow(non_snake_case)]
  fn visitBinaryExpr(&mut self, expr: &BinaryExpr) -> R;
  #[allow(non_snake_case)]
  fn visitGroupingExpr(&mut self, expr: &GroupingExpr) -> R;
  #[allow(non_snake_case)]
  fn visitLiteralExpr(&mut self, expr: &LiteralExpr) -> R;
  #[allow(non_snake_case)]
  fn visitUnaryExpr(&mut self, expr: &UnaryExpr) -> R;
}

trait AcceptExprVisitor<R> {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R;
}

pub enum Expr {
  Binary(BinaryExpr),
  Grouping(GroupingExpr),
  Literal(LiteralExpr),
  Unary(UnaryExpr),
}

impl<R> AcceptExprVisitor<R> for Expr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    match self {
      Expr::Binary(expr) => expr.accept(visitor),
      Expr::Grouping(expr) => expr.accept(visitor),
      Expr::Literal(expr) => expr.accept(visitor),
      Expr::Unary(expr) => expr.accept(visitor),
    }
  }
}

pub struct BinaryExpr {
  left: Box<Expr>,
  operator: Token,
  right: Box<Expr>,
}

impl<R> AcceptExprVisitor<R> for BinaryExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitBinaryExpr(self)
  }
}

impl BinaryExpr {
  pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
    Self { left, operator, right }
  }
}

pub struct GroupingExpr {
  expression: Box<Expr>,
}

impl<R> AcceptExprVisitor<R> for GroupingExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitGroupingExpr(self)
  }
}

impl GroupingExpr {
  pub fn new(expression: Box<Expr>) -> Self {
    Self { expression }
  }
}

pub struct LiteralExpr {
  token_type: TokenType,
  literal: String,
}

impl<R> AcceptExprVisitor<R> for LiteralExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitLiteralExpr(self)
  }
}

impl LiteralExpr {
  pub fn new(token_type: TokenType, literal: String) -> Self {
    Self { token_type, literal }
  }
}

pub struct UnaryExpr {
  operator: Token,
  right: Box<Expr>,
}

impl<R> AcceptExprVisitor<R> for UnaryExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitUnaryExpr(self)
  }
}

impl UnaryExpr {
  pub fn new(operator: Token, right: Box<Expr>) -> Self {
    Self { operator, right }
  }
}

/////////////// AST generation ///////////////
// following Robby Nystroms visitor pattern he demonstrates in the book...

// The first visitor is simple. It should just print out AST from grouped expressions
pub struct ASTPrinter;

impl ExprVisitor<String> for ASTPrinter {
  fn visitBinaryExpr(&mut self, expr: &BinaryExpr) -> String {
    format!("({} {} {})", expr.operator.token, expr.left.accept(self), expr.right.accept(self))
  }

  fn visitGroupingExpr(&mut self, expr: &GroupingExpr) -> String {
    format!("(group {})", expr.expression.accept(self))
  }

  fn visitLiteralExpr(&mut self, expr: &LiteralExpr) -> String {
    return expr.literal.to_string();
  }

  fn visitUnaryExpr(&mut self, expr: &UnaryExpr) -> String {
    format!("({} {})", expr.operator.token, expr.right.accept(self))
  }
}

impl ASTPrinter {
  pub fn print(&mut self, expr: &Expr) -> String {
    expr.accept(self)
  }
}

