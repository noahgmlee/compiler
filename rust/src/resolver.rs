use crate::interpreter::*;
use crate::ast::*;
use crate::lexer::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum FunctionType {
  None,
  Function,
  Initializer,
  Method
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ClassType {
  None,
  Class
}

pub struct Resolver {
  pub interpreter: Box<Rc<RefCell<Interpreter>>>,
  pub scopes: Vec<HashMap<String, bool>>,
  current_function: FunctionType,
  current_class: ClassType,
}

impl Resolver {
  pub fn new(interpreter: Box<Rc<RefCell<Interpreter>>>) -> Self {
    let scopes = Vec::new();
    let current_function = FunctionType::None;
    let current_class = ClassType::None;
    Self { interpreter, scopes, current_function, current_class }
  }

  pub fn resolve(&mut self, statements: &[Stmt]) {
    for statement in statements {
      self.resolve_stmt(statement);
    }
  }

  fn resolve_stmt(&mut self, statement: &Stmt) {
    match statement {
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

  fn resolve_expr(&mut self, expr: &Expr) {
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

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: &Token) {
    if let Some(scope) = self.scopes.last_mut() {
      if (scope.contains_key(&name.token)) {
        print!("Error: Variable {} already declared in this scope.", name.token);
        panic!();
      }
      scope.insert(name.token.clone(), false);
      return;
    }
  }

  fn define(&mut self, name: &Token) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.token.clone(), true);
      return;
    }
  }

  fn resolve_local(&mut self, expr: Expr, name: &Token) {
    for (i, scope) in self.scopes.iter().enumerate().rev() {
      if scope.contains_key(&name.token) {
        self.interpreter.borrow_mut().resolve(expr, self.scopes.len() - 1 - i);
        return;
      }
    }
  }

  fn resolve_function(&mut self, function: &FunStmt, function_type: FunctionType) {
    let enclosing_function = self.current_function.clone();
    self.current_function = function_type;
    self.begin_scope();
    for param in &function.params {
      self.declare(&param);
      self.define(&param);
    }
    self.resolve_stmt(&Stmt::Block(BlockStmt {
      statements: function.body.statements.iter().cloned().collect(),
    }));
    self.end_scope();
    self.current_function = enclosing_function;
  }
}

impl ExprVisitor<()> for Resolver {
  fn visitLiteralExpr(&mut self, expr: &LiteralExpr)  {
  }

  fn visitGroupingExpr(&mut self, expr: &GroupingExpr)  {
    self.resolve_expr(&expr.expression);
  }

  fn visitUnaryExpr(&mut self, expr: &UnaryExpr)  {
    self.resolve_expr(&expr.right);
  }

  fn visitBinaryExpr(&mut self, expr: &BinaryExpr)  {
    self.resolve_expr(&expr.left);
    self.resolve_expr(&expr.right);
  }

  fn visitCallExpr(&mut self, expr: &CallExpr)  {
    self.resolve_expr(&expr.callee);
    for arg in &expr.arguments {
      self.resolve_expr(arg);
    }
  }

  fn visitGetExpr(&mut self, expr: &GetExpr)  {
    self.resolve_expr(&expr.object);
  }

  fn visitSetExpr(&mut self, expr: &SetExpr)  {
    self.resolve_expr(&expr.value);
    self.resolve_expr(&expr.object);
  }

  fn visitVariableExpression(&mut self, expr: &VariableExpr)  {
    if let Some(scope) = self.scopes.last() {
      if let Some(defined) = scope.get(&expr.name.token) {
        if !defined {
          // Error: variable used before declaration
          eprintln!("Can't read local variable {} in its own initializer.", expr.name.token);
        }
      }
    }
    let expr_as_expr = Expr::Variable(expr.clone());
    self.resolve_local(expr_as_expr, &expr.name);
  }

  fn visitAssignExpression(&mut self, expr: &AssignExpr)  {
    self.resolve_expr(&expr.value);
    self.resolve_local(Expr::Assign(expr.clone()), &expr.name);
  }

  fn visitLogicalExpression(&mut self, expr: &LogicalExpr)  {
    self.resolve_expr(&expr.left);
    self.resolve_expr(&expr.right);
  }

  fn visitThisExpression(&mut self, expr: &ThisExpr) -> () {
    if self.current_class == ClassType::None {
      eprintln!("Error: Can't use 'this' outside of a class.");
      panic!();
    }
    self.resolve_local(Expr::This(expr.clone()), &expr.keyword);
  }
}

impl StmtVisitor<()> for Resolver {
  fn visitBlockStmt(&mut self, stmt: &BlockStmt) {
    self.begin_scope();
    self.resolve(&stmt.statements);
    self.end_scope();
  }

  fn visitForStmt(&mut self, stmt: &ForStmt) {
  }

  fn visitWhileStmt(&mut self, stmt: &WhileStmt) {
    self.resolve_expr(&stmt.condition);
    self.resolve_stmt(&stmt.body);
  }

  fn visitIfStmt(&mut self, stmt: &IfStmt) {
    self.resolve_expr(&stmt.condition);
    self.resolve_stmt(&stmt.then_branch);
    if let Some(else_branch) = &stmt.else_branch {
      self.resolve_stmt(else_branch);
    }
  }

  fn visitExpressionStmt(&mut self, stmt: &ExprStmt) {
    self.resolve_expr(&stmt.expression);
  }

  fn visitPrintStmt(&mut self, stmt: &PrintStmt) {
    self.resolve_expr(&stmt.expression);
  }

  fn visitReturnStmt(&mut self, stmt: &RetStmt) {
    if self.current_function == FunctionType::None {
      eprintln!("Error: Can't return from top-level code.");
      panic!();
    }
    if let Some(value) = &stmt.value {
      if self.current_function == FunctionType::Initializer {
        eprintln!("Error: Can't return a value from an initializer.");
        panic!();
      }
      self.resolve_expr(value);
    }
  }

  fn visitVarStmt(&mut self, stmt: &VarStmt) {
    self.declare(&stmt.name);
    if let Some(initializer) = &stmt.initializer {
      self.resolve_expr(initializer);
    }
    self.define(&stmt.name);
  }

  fn visitFunStmt(&mut self, stmt: &FunStmt) {
    self.declare(&stmt.name);
    self.define(&stmt.name);
    self.resolve_function(stmt, FunctionType::Function);
  }

  fn visitClassStmt(&mut self, stmt: &ClassStmt) -> () {
    let enclosing_class = self.current_class.clone();
    self.current_class = ClassType::Class;
    self.declare(&stmt.name);
    self.define(&stmt.name);
    self.begin_scope();
    self.scopes.last_mut().unwrap().insert("this".to_string(), true);
    for method in &stmt.methods {
      let mut function_type = FunctionType::Method;
      if (method.name.token == "init") {
        function_type = FunctionType::Initializer;
      }
      self.resolve_function(method, function_type);
    }
    self.end_scope();
    self.current_class = enclosing_class;
  }
}
