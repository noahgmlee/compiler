use crate::lexer::*;

/////////////// Expressions ///////////////
/// 
pub trait ExprVisitor<R> {
  #[allow(non_snake_case)]
  fn visitAssignExpression(&mut self, expr: &AssignExpr) -> R;
  #[allow(non_snake_case)]
  fn visitBinaryExpr(&mut self, expr: &BinaryExpr) -> R;
  #[allow(non_snake_case)]
  fn visitGroupingExpr(&mut self, expr: &GroupingExpr) -> R;
  #[allow(non_snake_case)]
  fn visitLiteralExpr(&mut self, expr: &LiteralExpr) -> R;
  #[allow(non_snake_case)]
  fn visitUnaryExpr(&mut self, expr: &UnaryExpr) -> R;
  #[allow(non_snake_case)]
  fn visitVariableExpression(&mut self, expr: &VariableExpr) -> R;
}

trait AcceptExprVisitor<R> {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R;
}

pub enum Expr {
  Assign(AssignExpr),
  Binary(BinaryExpr),
  Grouping(GroupingExpr),
  Literal(LiteralExpr),
  Unary(UnaryExpr),
  Variable(VariableExpr),
}

impl<R> AcceptExprVisitor<R> for Expr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    match self {
      Expr::Assign(expr) => expr.accept(visitor),
      Expr::Binary(expr) => expr.accept(visitor),
      Expr::Grouping(expr) => expr.accept(visitor),
      Expr::Literal(expr) => expr.accept(visitor),
      Expr::Unary(expr) => expr.accept(visitor),
      Expr::Variable(expr) => expr.accept(visitor),
    }
  }
}

pub struct AssignExpr {
  pub name: Token,
  pub value: Box<Expr>,
}
impl<R> AcceptExprVisitor<R> for AssignExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitAssignExpression(&self)
  }
}

impl AssignExpr {
  pub fn new(name: Token, value: Box<Expr>) -> Self {
    Self { name, value }
  }
}

pub struct BinaryExpr {
  pub left: Box<Expr>,
  pub operator: Token,
  pub right: Box<Expr>,
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
  pub expression: Box<Expr>,
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
  pub token_type: TokenType,
  pub literal: LoxValue,
}

impl<R> AcceptExprVisitor<R> for LiteralExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitLiteralExpr(self)
  }
}

impl LiteralExpr {
  pub fn new(token_type: TokenType, literal: LoxValue) -> Self {
    Self { token_type, literal }
  }
}

pub struct UnaryExpr {
  pub operator: Token,
  pub right: Box<Expr>,
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

pub struct VariableExpr {
  pub name: Token,
}

impl<R> AcceptExprVisitor<R> for VariableExpr {
  fn accept(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
    visitor.visitVariableExpression(&self)
  }
}

/////////////// Statements ///////////////
/// 
pub trait StmtVisitor<R> {
  #[allow(non_snake_case)]
  fn visitBlockStmt(&mut self, stmt: &BlockStmt) -> R;
  #[allow(non_snake_case)]
  fn visitExpressionStmt(&mut self, stmt: &ExprStmt) -> R;
  #[allow(non_snake_case)]
  fn visitPrintStmt(&mut self, stmt: &PrintStmt) -> R;
  #[allow(non_snake_case)]
  fn visitVarStmt(&mut self, stmt: &VarStmt) -> R;
}

pub trait AcceptStmtVisitor<R> {
  fn accept(&self, visitor: &mut dyn StmtVisitor<R>) -> R;
}

pub enum Stmt {
  Block(BlockStmt),
  Expression(ExprStmt),
  Print(PrintStmt),
  Var(VarStmt),
}

impl<R> AcceptStmtVisitor<R> for Stmt {
  fn accept(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
    match self {
      Stmt::Block(block) => block.accept(visitor),
      Stmt::Expression(expr) => expr.accept(visitor),
      Stmt::Print(expr) => expr.accept(visitor),
      Stmt::Var(expr) => expr.accept(visitor),
    }
  }
}

pub struct ExprStmt {
  pub expression: Box<Expr>,
}

impl <R> AcceptStmtVisitor<R> for ExprStmt {
  fn accept(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
    visitor.visitExpressionStmt(self)
  }
}

pub struct PrintStmt {
  pub expression: Box<Expr>,
}

impl <R> AcceptStmtVisitor<R> for PrintStmt {
  fn accept(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
    visitor.visitPrintStmt(self)
  }
}

pub struct VarStmt {
  pub name: Token,
  pub initializer: Option<Expr>,
}

impl <R> AcceptStmtVisitor<R> for VarStmt {
  fn accept(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
    visitor.visitVarStmt(self)
  }
}

impl VarStmt {
  pub fn new(name: Token, initializer: Option<Expr>) -> Self {
    Self { name, initializer }
  }
}

pub struct BlockStmt {
  pub statements: Vec<Stmt>,
}

impl BlockStmt {
  pub fn new(statements: Vec<Stmt>) -> Self {
    Self { statements }
  }
}

impl <R> AcceptStmtVisitor<R> for BlockStmt {
  fn accept(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
    visitor.visitBlockStmt(self)
  }
}
