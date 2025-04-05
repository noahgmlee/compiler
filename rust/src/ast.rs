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
  #[allow(non_snake_case)]
  fn visitLogicalExpression(&mut self, expr: &LogicalExpr) -> R;
}

pub enum Expr {
  Assign(AssignExpr),
  Binary(BinaryExpr),
  Grouping(GroupingExpr),
  Literal(LiteralExpr),
  Unary(UnaryExpr),
  Variable(VariableExpr),
  Logical(LogicalExpr),
}
pub struct AssignExpr {
  pub name: Token,
  pub value: Box<Expr>,
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

impl BinaryExpr {
  pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
    Self { left, operator, right }
  }
}

pub struct GroupingExpr {
  pub expression: Box<Expr>,
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

impl LiteralExpr {
  pub fn new(token_type: TokenType, literal: LoxValue) -> Self {
    Self { token_type, literal }
  }
}

pub struct UnaryExpr {
  pub operator: Token,
  pub right: Box<Expr>,
}

impl UnaryExpr {
  pub fn new(operator: Token, right: Box<Expr>) -> Self {
    Self { operator, right }
  }
}

pub struct VariableExpr {
  pub name: Token,
}

pub struct LogicalExpr {
  pub left: Box<Expr>,
  pub operator: Token,
  pub right: Box<Expr>,
}

impl LogicalExpr {
  pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
    Self { left, operator, right }
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
  #[allow(non_snake_case)]
  fn visitIfStmt(&mut self, stmt: &IfStmt) -> R;
  #[allow(non_snake_case)]
  fn visitWhileStmt(&mut self, stmt: &WhileStmt) -> R;
  #[allow(non_snake_case)]
  fn visitForStmt(&mut self, stmt: &ForStmt) -> R;
}

pub enum Stmt {
  Block(BlockStmt),
  Expression(ExprStmt),
  Print(PrintStmt),
  Var(VarStmt),
  If(IfStmt),
  While(WhileStmt),
  For(ForStmt),
}

pub struct ExprStmt {
  pub expression: Box<Expr>,
}

pub struct PrintStmt {
  pub expression: Box<Expr>,
}

pub struct VarStmt {
  pub name: Token,
  pub initializer: Option<Expr>,
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

pub struct IfStmt {
  pub condition: Box<Expr>,
  pub then_branch: Box<Stmt>,
  pub else_branch: Option<Box<Stmt>>,
}

impl IfStmt{
  pub fn new(condition: Box<Expr>, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Self {
    Self { condition, then_branch, else_branch }
  }
}

pub struct WhileStmt {
  pub condition: Box<Expr>,
  pub body: Box<Stmt>,
}

impl WhileStmt {
  pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
    Self { condition, body }
  }
}

pub struct ForStmt {
  pub initializer: Option<Box<Stmt>>,
  pub condition: Option<Box<Expr>>,
  pub increment: Option<Box<Expr>>,
  pub body: Box<Stmt>,
}

impl ForStmt {
  pub fn new(initializer: Option<Box<Stmt>>, condition: Option<Box<Expr>>, increment: Option<Box<Expr>>, body: Box<Stmt>) -> Self {
    Self { initializer, condition, increment, body }
  }
}
