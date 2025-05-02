use crate::lexer::*;
use std::hash::Hash;
use std::rc::Rc;

/////////////// Expressions ///////////////
/// 
pub trait ExprVisitor<R> {
  #[allow(non_snake_case)]
  fn visitAssignExpression(&mut self, expr: &AssignExpr) -> R;
  #[allow(non_snake_case)]
  fn visitBinaryExpr(&mut self, expr: &BinaryExpr) -> R;
  #[allow(non_snake_case)]
  fn visitCallExpr(&mut self, expr: &CallExpr) -> R;
  #[allow(non_snake_case)]
  fn visitGetExpr(&mut self, expr: &GetExpr) -> R;
  #[allow(non_snake_case)]
  fn visitSetExpr(&mut self, expr: &SetExpr) -> R;
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
  #[allow(non_snake_case)]
  fn visitSuperExpression(&mut self, expr: &SuperExpr) -> R;
  #[allow(non_snake_case)]
  fn visitThisExpression(&mut self, expr: &ThisExpr) -> R;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Expr {
  Assign(AssignExpr),
  Binary(BinaryExpr),
  Call(CallExpr),
  Get(GetExpr),
  Set(SetExpr),
  Grouping(GroupingExpr),
  Literal(LiteralExpr),
  Unary(UnaryExpr),
  Variable(VariableExpr),
  Logical(LogicalExpr),
  Super(SuperExpr),
  This(ThisExpr),
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AssignExpr {
  pub name: Token,
  pub value: Box<Expr>,
}

impl AssignExpr {
  pub fn new(name: Token, value: Box<Expr>) -> Self {
    Self { name, value }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CallExpr {
  pub callee: Box<Expr>,
  pub paren: Token,
  pub arguments: Vec<Expr>,
}

impl CallExpr {
  pub fn new(callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Self {
    Self { callee, paren, arguments }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GetExpr {
  pub object: Box<Expr>,
  pub name: Token,
}

impl GetExpr {
  pub fn new(object: Box<Expr>, name: Token) -> Self {
    Self { object, name }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SetExpr {
  pub object: Box<Expr>,
  pub name: Token,
  pub value: Box<Expr>,
}

impl SetExpr {
  pub fn new(object: Box<Expr>, name: Token, value: Box<Expr>) -> Self {
    Self { object, name, value }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GroupingExpr {
  pub expression: Box<Expr>,
}

impl GroupingExpr {
  pub fn new(expression: Box<Expr>) -> Self {
    Self { expression }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralExpr {
  pub token_type: TokenType,
  pub literal: LoxValue,
}

// This is not good enough, but we never resolve literals, so it doesn't matter
impl Hash for LiteralExpr {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.token_type.hash(state);
  }
}

impl Eq for LiteralExpr {}

impl LiteralExpr {
  pub fn new(token_type: TokenType, literal: LoxValue) -> Self {
    Self { token_type, literal }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UnaryExpr {
  pub operator: Token,
  pub right: Box<Expr>,
}

impl UnaryExpr {
  pub fn new(operator: Token, right: Box<Expr>) -> Self {
    Self { operator, right }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VariableExpr {
  pub name: Token,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SuperExpr {
  pub keyword: Token,
  pub method: Token,
}

impl SuperExpr {
  pub fn new(keyword: Token, method: Token) -> Self {
    Self { keyword, method }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ThisExpr {
  pub keyword: Token,
}

impl ThisExpr {
  pub fn new(keyword: Token) -> Self {
    Self { keyword }
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
  fn visitReturnStmt(&mut self, stmt: &RetStmt) -> R;
  #[allow(non_snake_case)]
  fn visitFunStmt(&mut self, stmt: &FunStmt) -> R;
  #[allow(non_snake_case)]
  fn visitIfStmt(&mut self, stmt: &IfStmt) -> R;
  #[allow(non_snake_case)]
  fn visitWhileStmt(&mut self, stmt: &WhileStmt) -> R;
  #[allow(non_snake_case)]
  fn visitForStmt(&mut self, stmt: &ForStmt) -> R;
  #[allow(non_snake_case)]
  fn visitClassStmt(&mut self, stmt: &ClassStmt) -> R;
}

#[derive(Clone, Debug)]
pub enum Stmt {
  Block(BlockStmt),
  Expression(ExprStmt),
  Print(PrintStmt),
  Return(RetStmt),
  Var(VarStmt),
  Fun(FunStmt),
  If(IfStmt),
  While(WhileStmt),
  For(ForStmt),
  Class(ClassStmt),
}

#[derive(Clone, Debug)]
pub struct ExprStmt {
  pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct PrintStmt {
  pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct VarStmt {
  pub name: Token,
  pub initializer: Option<Expr>,
}

impl VarStmt {
  pub fn new(name: Token, initializer: Option<Expr>) -> Self {
    Self { name, initializer }
  }
}

#[derive(Clone, Debug)]
pub struct RetStmt {
  pub keyword: Token,
  pub value: Option<Box<Expr>>,
}

impl RetStmt {
  pub fn new(keyword: Token, value: Option<Box<Expr>>) -> Self {
    Self { keyword, value }
  }
}

#[derive(Clone, Debug)]
pub struct FunStmt {
  pub name: Token,
  pub params: Vec<Token>,
  pub body: Rc<BlockStmt>,
}

impl FunStmt {
  pub fn new(name: Token, params: Vec<Token>, body: Rc<BlockStmt>) -> Self {
    Self { name, params, body }
  }
}

#[derive(Clone, Debug)]
pub struct BlockStmt {
  pub statements: Vec<Stmt>,
}

impl BlockStmt {
  pub fn new(statements: Vec<Stmt>) -> Self {
    Self { statements }
  }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct WhileStmt {
  pub condition: Box<Expr>,
  pub body: Box<Stmt>,
}

impl WhileStmt {
  pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
    Self { condition, body }
  }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ClassStmt {
  pub name: Token,
  pub superclass: Option<Box<Expr>>,
  pub methods: Vec<FunStmt>,
}

impl ClassStmt {
  pub fn new(name: Token, superclass: Option<Box<Expr>>,  methods: Vec<FunStmt>) -> Self {
    Self { name, superclass, methods }
  }
}
