package main

import "fmt"

const (
	// Single-character tokens.
	LEFT_PAREN int = iota
	RIGHT_PAREN
	LEFT_BRACE
	RIGHT_BRACE
	COMMA
	DOT
	MINUS
	PLUS
	SEMICOLON
	SLASH
	STAR

	// One or two character tokens.
	BANG
	BANG_EQUAL
	EQUAL
	EQUAL_EQUAL
	GREATER
	GREATER_EQUAL
	LESS
	LESS_EQUAL

	// Literals.
	IDENTIFIER
	STRING
	NUMBER

	// Keywords.
	AND
	CLASS
	ELSE
	FALSE
	FUN
	FOR
	IF
	NIL
	OR
	PRINT
	RETURN
	SUPER
	THIS
	TRUE
	VAR
	WHILE

	EOF
)

var token_names = map[int]string{
	LEFT_PAREN:    "LEFT_PAREN",
	RIGHT_PAREN:   "RIGHT_PAREN",
	LEFT_BRACE:    "LEFT_BRACE",
	RIGHT_BRACE:   "RIGHT_BRACE",
	COMMA:         "COMMA",
	DOT:           "DOT",
	MINUS:         "MINUS",
	PLUS:          "PLUS",
	SEMICOLON:     "SEMICOLON",
	SLASH:         "SLASH",
	STAR:          "STAR",
	BANG:          "BANG",
	BANG_EQUAL:    "BANG_EQUAL",
	EQUAL:         "EQUAL",
	EQUAL_EQUAL:   "EQUAL_EQUAL",
	GREATER:       "GREATER",
	GREATER_EQUAL: "GREATER_EQUAL",
	LESS:          "LESS",
	LESS_EQUAL:    "LESS_EQUAL",
	IDENTIFIER:    "IDENTIFIER",
	STRING:        "STRING",
	NUMBER:        "NUMBER",
	AND:           "AND",
	CLASS:         "CLASS",
	ELSE:          "ELSE",
	FALSE:         "FALSE",
	FUN:           "FUN",
	FOR:           "FOR",
	IF:            "IF",
	NIL:           "NIL",
	OR:            "OR",
	PRINT:         "PRINT",
	RETURN:        "RETURN",
	SUPER:         "SUPER",
	THIS:          "THIS",
	TRUE:          "TRUE",
	VAR:           "VAR",
	WHILE:         "WHILE",
	EOF:           "EOF",
}

var keywords = map[string]int{
	"and":    AND,
	"class":  CLASS,
	"else":   ELSE,
	"false":  FALSE,
	"for":    FOR,
	"fun":    FUN,
	"if":     IF,
	"nil":    NIL,
	"or":     OR,
	"print":  PRINT,
	"return": RETURN,
	"super":  SUPER,
	"this":   THIS,
	"true":   TRUE,
	"var":    VAR,
	"while":  WHILE,
}

type Token struct {
	token_type_ int
	lexeme  string
	literal any
	line int
}

func (t Token) ToString() string {
	return fmt.Sprintf("TOKEN_TYPE: %s, TOKEN: %s, LITERAL: %v", token_names[t.token_type_], t.lexeme, t.literal)
}

type Lexer struct {
	source string
	tokens []Token
	start int
	current int
	line int
}

func NewLexer(source string) *Lexer {
	return &Lexer{source: source, tokens: nil, start: 0, current: 0, line: 1}
}

func (l *Lexer) ScanTokens() []Token {
	for !l.isAtEnd() {
		l.start = l.current
		l.ScanToken()
	}
	l.tokens = append(l.tokens, Token{EOF, "", nil, l.line})
	return l.tokens
}

func (l *Lexer) ScanToken() {
	c := l.advance()
	switch c {
		case '(': 
			l.addToken(LEFT_PAREN)
		case ')': 
			l.addToken(RIGHT_PAREN)
		case '{': 
			l.addToken(LEFT_BRACE)
		case '}': 
			l.addToken(RIGHT_BRACE)
		case ',': 
			l.addToken(COMMA)
		case '.': 
			l.addToken(DOT)
		case '-': 
			l.addToken(MINUS)
		case '+': 
			l.addToken(PLUS)
		case ';': 
			l.addToken(SEMICOLON)
		case '*': 
			l.addToken(STAR) 
		case '!':
			if l.match('=') {
				l.addToken(BANG_EQUAL)
			} else {
				l.addToken(BANG)
			}
		case '=':
			if l.match('=') {
				l.addToken(EQUAL_EQUAL)
			} else {
				l.addToken(EQUAL)
			}
		case '<':
			if l.match('=') {
				l.addToken(LESS_EQUAL)
			} else {
				l.addToken(LESS)
			}
		case '>':
			if l.match('=') {
				l.addToken(GREATER_EQUAL)
			} else {
				l.addToken(GREATER)
			}
		case '/':
			if (l.match('/')) {
				for l.peek() != '\n' && !l.isAtEnd() {
					l.advance()
				}
			} else {
				l.addToken(SLASH)
			}
		case ' ':
		case '\r':
		case '\t':
		case '\n':
			l.line++
		case '"': 
			l.string()
		default:
			if (isDigit(c)) {
				l.number()
			} else if (isAlpha(c)) {
				for isAlphaNumeric(l.peek()) {
					l.advance()
				}
				text := l.source[l.start:l.current]
				token_type, is_keyword := keywords[text]
				if is_keyword {
					l.addToken(token_type)
				} else {
					l.addTokenLiteral(IDENTIFIER, text)
				}
			} else {
				error(l.line, "Unexpected character.")
			}
	}
}

func isDigit(c byte) bool {
	return c >= '0' && c <= '9'
}

func isAlpha(c byte) bool {
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

func isAlphaNumeric(c byte) bool {
	return isAlpha(c) || isDigit(c)
}

func (l *Lexer) addToken(token_type_ int) {
	text := l.source[l.start:l.current]
	l.tokens = append(l.tokens, Token{token_type_, text, nil, l.line})
}

func (l *Lexer) addTokenLiteral(token_type_ int, literal any) {
	text := l.source[l.start:l.current]
	l.tokens = append(l.tokens, Token{token_type_, text, literal, l.line})
}

func (l *Lexer) advance() byte {
	c := l.source[l.current]
	l.current++
	return c
}

func (l *Lexer) match(expected byte) bool {
	if l.isAtEnd() {
		return false
	}
	if l.source[l.current] != expected {
		return false
	}
	l.current++
	return true
}

func (l *Lexer) peek() byte {
	if l.isAtEnd() {
		return '\000'
	}
	return l.source[l.current]
}

func (l *Lexer) peekNext() byte {
	if l.current + 1 >= len(l.source) {
		return '\000'
	}
	return l.source[l.current + 1]
}

func (l *Lexer) string() {
	for l.peek() != '"' && !l.isAtEnd() {
		if l.peek() == '\n' {
			l.line++
		}
		l.advance()
	}
	if l.isAtEnd() {
		error(l.line, "Unterminated string.")
		return
	}
	l.advance()
	value := l.source[l.start + 1:l.current - 1]
	l.addTokenLiteral(STRING, value)
}

func (l *Lexer) number() {
	for isDigit(l.peek()) {
		l.advance()
	}
	if l.peek() == '.' && isDigit(l.peekNext()) {
		l.advance()
		for isDigit(l.peek()) {
			l.advance()
		}
	}
	literal := l.source[l.start:l.current]
	l.addTokenLiteral(NUMBER, literal)
}

func (l *Lexer) isAtEnd() bool {
	return l.current >= len(l.source)
}
