/*
Overview:

Our parsing / precedence is based on:
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;

This grammar allows for a recursive descent parser to be implemented.
note that left recursion is intentionally avoided in the grammar.
*/

use crate::lexer::*;
use crate::ast::*;
use crate::logging::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub struct ParserError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();
        let mut had_error = false;
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                  statements.push(stmt);
                }
                Err(_) => {
                  self.synchronize();
                  had_error = true;
                }
            }
        }
        if had_error {
            return Err(ParserError {});
        }
        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_tokens(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.match_tokens(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(VarStmt::new(name, initializer)))
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokens(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt::new(self.block()?)));
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();
        while !self.is_at_end() && !self.check(TokenType::RightBrace) {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                  return Err(ParserError{});
                }
            }
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt{expression : Box::new(value)}))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(ExprStmt{ expression : Box::new(expr)}))
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality()?;
        if self.match_tokens(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assign(AssignExpr::new(var.name, Box::new(value))));
            }
            self.error(equals, "Invalid assignment target.");
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;
        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        while self.match_tokens(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;
        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr::new(operator, Box::new(right))));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.peek();
        if self.match_tokens(vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenType::False, LoxValue::Boolean(false))));
        }
        if self.match_tokens(vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenType::True, LoxValue::Boolean(true))));
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenType::Nil, LoxValue::Nil)));
        }
        if self.match_tokens(vec![TokenType::Number]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenType::Number, token.literal.clone())));
        }
        if self.match_tokens(vec![TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenType::String, token.literal.clone())));
        }
        if self.match_tokens(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr{name : token.clone()}));
        }
        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _noop = self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr::new(Box::new(expr))));
        }
        let token = self.peek();
        Err(self.error(token, "Expect expression."))
    }

    fn match_tokens(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
      self.peek().token_type == TokenType::Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        let token = self.peek();
        Err(self.error(token, message))
    }

    fn error(&mut self, token: Token, message: &str) -> ParserError {
        error_at_token(&token, message);
        ParserError {}
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}
