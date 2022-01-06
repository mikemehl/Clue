#![allow(non_camel_case_types)]

use self::TokenType::*;
use std::fmt;

#[derive(Debug)]
pub enum TokenType {
	//symbols
	ROUND_BRACKET_OPEN, ROUND_BRACKET_CLOSED,
	SQUARE_BRACKET_OPEN, SQUARE_BRACKET_CLOSED,
	CURLY_BRACKET_OPEN, CURLY_BRACKET_CLOSED,
	COMMA, DOT, SEMICOLON, NOT, AND, OR, DOLLAR,
	PLUS, MINUS, STAR, SLASH, PERCENTUAL, CARET,
	HASHTAG, METHOD, TWODOTS, TREDOTS,
	
	//definition and comparison
	DEFINE, DEFINEIF, INCREASE, DECREASE, MULTIPLY, DIVIDE, EXPONENTIATE, CONCATENATE,
	EQUAL, NOT_EQUAL, BIGGER, BIGGER_EQUAL, SMALLER, SMALLER_EQUAL, LAMBDA,
	
	//literals
	IDENTIFIER, NUMBER, STRING,
	
	//keywords
	DO, IF, ELSEIF, ELSE, FOR, OF, IN, WITH, WHILE, NEW, META,
	UNTIL, GOTO, LOCAL, FUNCTION, RETURN, THIS, TRUE, FALSE, NIL,
	
	EOF = -1
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub struct Token {
	pub kind: TokenType,
	pub lexeme: String,
	pub line: u32
}

impl Token {
	fn new(kind: TokenType, lexeme: String, line: u32) -> Token {
		Token {
			kind: kind,
			lexeme: String::from(lexeme),
			line: line
		}
	}
}

struct CodeInfo {
	line: u32,
	start: usize,
	current: usize,
	size: usize,
	code: String,
	filename: String,
	tokens: Vec<Token>,
	errored: bool,
}

impl CodeInfo {
	fn new(code: String, filename: String) -> CodeInfo {
		CodeInfo {
			line: 1,
			start: 0,
			current: 0,
			size: code.chars().count(),
			code: code,
			filename: filename,
			tokens: Vec::new(),
			errored: false
		}
	}

	fn ended(&self) -> bool {
		self.current >= self.size
	}

	fn at(&self, pos: usize) -> char {
		if pos >= self.size {return 0 as char}
		self.code.as_bytes()[pos] as char
	}

	fn readNext(&mut self) -> char {
		let prev: char = self.at(self.current);
		self.current += 1;
		prev
	}

	fn compare(&mut self, expected: char) -> bool {
		if self.ended() {return false;}
		if self.at(self.current) != expected {return false;}
		self.current = self.current + 1;
		true
	}

	fn peekFar(&self, pos: usize) -> char {
		let pos: usize = self.current + pos;
		if pos >= self.size {return 0 as char;}
		self.at(pos)
	}

	fn peek(&self) -> char {
		self.peekFar(0)
	}

	//isNumber: c.is_ascii_digit()
	//isChar: c.is_ascii_alphabetic()
	//isCharOrNumber: c.is_ascii_alphanumeric()

	fn substr(&self, start: usize, end: usize) -> String {
		let mut result: String = String::new();
		for i in start..end {
			if i >= self.size {break}
			result.push(self.at(i));
		}
		result
	}

	fn addLiteralToken(&mut self, kind: TokenType, literal: String) {
		self.tokens.push(Token::new(kind, literal, self.line));
	}

	fn addToken(&mut self, kind: TokenType) {
		let lexeme: String = self.substr(self.start, self.current);
		self.tokens.push(Token::new(kind, lexeme, self.line));
	}

	fn compareAndAdd(&mut self, c: char, kt: TokenType, kf: TokenType) {
		let kind: TokenType = match self.compare(c) {
			true => kt,
			false => kf
		};
		self.addToken(kind);
	}

	fn warning(&mut self, message: &str) {
		println!("Error in file \"{}\" at line [{}]!\nError: \"{}\"", self.filename, self.line, message);
		self.errored = true;
	}
}

pub fn ScanFile(code: String, filename: String) -> Result<Vec<Token>, String> {
	let mut i: CodeInfo = CodeInfo::new(code, filename);
	while !i.ended() {
		i.start = i.current;
		let c: char = i.readNext();
		match c {
			'(' => i.addToken(ROUND_BRACKET_OPEN),
			')' => i.addToken(ROUND_BRACKET_CLOSED),
			'[' => i.addToken(SQUARE_BRACKET_OPEN),
			']' => i.addToken(SQUARE_BRACKET_CLOSED),
			'{' => i.addToken(CURLY_BRACKET_OPEN),
			'}' => i.addToken(CURLY_BRACKET_CLOSED),
			',' => i.addToken(COMMA),
			'.' => {
				if i.peek() == '.' {
					i.current += 1;
					let f: char = i.peekFar(1);
					if f == '.' {
						i.current += 1;
						i.addToken(TREDOTS);
					} else if f == '=' {
						i.current += 1;
						i.addToken(CONCATENATE);
					} else {
						i.addToken(TWODOTS);
					}
				} else {
					i.addToken(DOT);
				}
			},
			';' => i.addToken(SEMICOLON),
			'+' => i.compareAndAdd('=', INCREASE, PLUS),
			'-' => i.compareAndAdd('=', DECREASE, MINUS),
			'*' => i.compareAndAdd('=', MULTIPLY, STAR),
			'^' => i.compareAndAdd('=', EXPONENTIATE, CARET),
			'#' => i.addToken(HASHTAG),
			'/' => {
				match i.peek() {
					'/' => while i.peek() != '\n' && !i.ended() {i.current += 1},
					'*' => {
						while !i.ended() && !(i.peek() == '*' && i.peekFar(1) == '/') {
							if i.peek() == '\n' {i.line += 1}
							i.current += 1;
						}
						if i.ended() {
							i.warning("Unterminated comment.");
						} else {
							i.current += 2;
						}
					},
					'=' => i.addToken(DIVIDE),
					_ => i.addToken(SLASH)
				}
			},
			'%' => i.addToken(PERCENTUAL),
			'!' => i.compareAndAdd('=', NOT_EQUAL, NOT),
			'=' => {
				match i.peek() {
					'=' => {i.current += 1; i.addToken(EQUAL)},
					'>' => {i.current += 1; i.addToken(LAMBDA)},
					_ => i.addToken(DEFINE)
				}
			},
			'<' => i.compareAndAdd('=', SMALLER_EQUAL, SMALLER),
			'>' => i.compareAndAdd('=', BIGGER_EQUAL, BIGGER),
			'?' => i.compareAndAdd('=', DEFINEIF, AND),
			'&' => i.addToken(AND),
			':' => i.compareAndAdd(':', METHOD, OR),
			'|' => i.addToken(OR),
			'$' => i.addToken(DOLLAR),
			' ' | '\r' | '\t' => {},
			'\n' => i.line += 1,
			'"' => {
				while !i.ended() && i.peek() != '"' {
					if i.peek() == '\n' {i.line += 1};
					i.current += 1;
				}
				if i.ended() {
					i.warning("Unterminated string");
				} else {
					i.current += 1;
					let literal: String = i.substr(i.start + 1, i.current - 1);
					i.addLiteralToken(STRING, literal);
				}
			}
			_ => {
				if c.is_ascii_digit() {
					while i.peek().is_ascii_digit() {i.current += 1}
					if i.peek() == '.' && i.peekFar(1).is_ascii_digit() {
						i.current += 1;
						while i.peek().is_ascii_digit() {i.current += 1}
					}
					i.addLiteralToken(NUMBER, i.substr(i.start, i.current));
				} else if c.is_ascii_alphabetic() {
					while i.peek().is_ascii_alphanumeric() {i.current += 1}
					let string: String = i.substr(i.start, i.current);
					let kind: TokenType = match string.as_str() {
						"do" => DO,
						"if" => IF,
						"elseif" => ELSEIF,
						"else" => ELSE,
						"for" => FOR,
						"of" => OF,
						"in" => IN,
						"with" => WITH,
						"while" => WHILE,
						"new" => NEW,
						"meta" => META,
						"until" => UNTIL,
						"goto" => GOTO,
						"local" => LOCAL,
						"function" => FUNCTION,
						"return" => RETURN,
						"this" => THIS,
						"true" => TRUE,
						"false" => FALSE,
						"nil" => NIL,
						_ => IDENTIFIER
					};
					i.addToken(kind);
				} else {
					i.warning(format!("Unexpected character '{}'.", c).as_str());
				}
			}
		}
	}
	if i.errored {
		return Err(String::from("Cannot continue until the above errors are fixed."));
	}
	i.addLiteralToken(EOF, String::new());
	Ok(i.tokens)
}