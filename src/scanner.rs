/// A token in the space lox language
#[derive(Debug, Clone)]
pub struct Token<'a> {
  /// The token kind
  pub kind: TokenKind,

  /// The character array of the source
  pub lexeme: &'a str,

  /// line number this token appears
  pub line: i32,
}

/// Token kinds in the space lox language
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenKind {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  Identifier,
  String,
  Number,

  And,
  Class,
  Else,
  False,
  For,
  Fun,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Error,
  Eof,
}

/// A scanner for the lox language. This struct is
/// responsible for taking a source string and tokenizing it
pub struct Scanner<'a> {

  /// The input source string
  source: &'a str,

  /// The current line number
  line: i32,

  /// The start of the current token
  start: usize,

  /// The current index into the source str
  current: usize,

  /// The start of the current character
  char_start: usize,
}

const STRING_ERROR: &'static str = "Unterminated string";
const UNKNOWN_CHARACTER: &'static str = "Unexpected character";
const END_OF_FILE: &'static str = "";

impl<'a> Scanner<'a> {
  /// Create a new scanner that can be used to tokenize
  /// a lox source string
  /// 
  /// # Examples
  /// ```
  /// use lox_runtime::scanner::{Scanner, TokenKind};
  /// 
  /// let source = "
  /// var x = \"something\";
  /// if not x == \"something\" {
  ///   print(x);
  /// } 
  /// ";
  /// 
  /// let mut scanner = Scanner::new(source);
  /// let token = scanner.scan_token();
  /// assert_eq!(token.line, 1);
  /// assert_eq!(token.kind, TokenKind::Var);
  /// assert_eq!(token.lexeme, "var");
  /// ```
  pub fn new(source: &str) -> Scanner {
    Scanner {
      source,

      start: 0,
      current: next_boundary(source, 0),
      char_start: 0,

      line: 0,
    }
  }

  /// Scan the next token from the space lox source
  /// string provide.
  /// 
  /// # Examples
  /// ```
  /// use lox_runtime::scanner::{Scanner, TokenKind};
  /// 
  /// let source = "
  /// var x = \"something\";
  /// if not x == \"something\" {
  ///   print(x);
  /// } 
  /// ";
  /// 
  /// let mut scanner = Scanner::new(source);
  /// let mut token = scanner.scan_token();
  /// assert_eq!(token.line, 1);
  /// assert_eq!(token.kind, TokenKind::Var);
  /// assert_eq!(token.lexeme, "var");
  /// 
  /// token = scanner.scan_token();
  /// assert_eq!(token.line, 1);
  /// assert_eq!(token.kind, TokenKind::Identifier);
  /// assert_eq!(token.lexeme, "x");
  /// 
  /// token = scanner.scan_token();
  /// assert_eq!(token.line, 1);
  /// assert_eq!(token.kind, TokenKind::Equal);
  /// assert_eq!(token.lexeme, "=");
  /// ```
  pub fn scan_token(&mut self) -> Token<'a> {

    // advance whitespace
    self.skip_white_space();

    // find the previous unicode boundary
    self.start = previous_boundary(self.source, self.current);
    self.char_start = self.start;

    // if at end return oef token
    if self.is_at_end() {
      self.current = self.start;
      return self.make_token(TokenKind::Eof, END_OF_FILE, self.line);
    }

    // move scanner index and get current unicode character
    let c = self.advance();
    match c {
      "(" => self.make_token_source(TokenKind::LeftParen),
      ")" => self.make_token_source(TokenKind::RightParen),
      "{" => self.make_token_source(TokenKind::LeftBrace),
      "}" => self.make_token_source(TokenKind::RightBrace),
      ";" => self.make_token_source(TokenKind::Semicolon),
      "," => self.make_token_source(TokenKind::Comma),
      "." => self.make_token_source(TokenKind::Dot),
      "-" => self.make_token_source(TokenKind::Minus),
      "+" => self.make_token_source(TokenKind::Plus),
      "/" => self.make_token_source(TokenKind::Slash),
      "*" => self.make_token_source(TokenKind::Star),
      "=" => {
        if self.match_token("=") {
          self.make_token_source(TokenKind::EqualEqual)
        } else {
          self.make_token_source(TokenKind::Equal)
        }
      }
      "<" => {
        if self.match_token("=") {
          self.make_token_source(TokenKind::LessEqual)
        } else {
          self.make_token_source(TokenKind::Less)
        }
      }
      ">" => {
        if self.match_token("=") {
          self.make_token_source(TokenKind::GreaterEqual)
        } else {
          self.make_token_source(TokenKind::Greater)
        }
      }
      "!" => {
        if self.match_token("=") {
          self.make_token_source(TokenKind::BangEqual)
        } else {
          self.make_token_source(TokenKind::Bang)
        }
      }
      "\"" => self.string(),
      _ => {
        if is_digit(c) {
          return self.number();
        }

        if is_alpha(c) {
          return self.identifier();
        }

        self.error_token(UNKNOWN_CHARACTER)
      }
    }
  }

  /// Generate an identifier token
  fn identifier(&mut self) -> Token<'a> {

    // advance until we hit whitespace or a special char
    while !self.is_at_end() && (is_alpha(self.peek()) || is_digit(self.peek())) {
      self.advance();
    }

    // identifier if we are actually a keyword
    self.make_token_source(self.identifier_type())
  }

  /// Generate a number token
  fn number(&mut self) -> Token<'a> {

    // advance consecutive digits
    while !self.is_at_end() && is_digit(self.peek()) {
      self.advance();
    }

    // check if floating point format
    if !self.is_at_end() && self.peek() == "." {
      if let Some(next) = self.peek_next() {
        if is_digit(next) {
          self.advance();
          while !self.is_at_end() && is_digit(self.peek()) {
            self.advance();
          }
        }
      }
    }

    self.make_token_source(TokenKind::Number)
  }

  /// Generate a string token
  fn string(&mut self) -> Token<'a> {
    while !self.is_at_end() && self.peek() != "\"" {
      if self.peek() == "\n" {
        self.line += 1;
      }
      self.advance();
    }

    if self.is_at_end() {
      return self.error_token(&STRING_ERROR);
    }

    self.advance();
    self.make_token_source(TokenKind::String)
  }

  /// Advance through whitespace effectively throwing it away
  fn skip_white_space(&mut self) {
    while !self.is_at_end() {
      let c = self.peek();
      match c {
        " " | "\r" | "\t" => {
          self.advance();
        }
        "\n" => {
          self.line += 1;
          self.advance();
        }
        "/" => match self.peek_next() {
          Some(next) => {
            if next == "/" {
              while self.peek() != "\n" && self.is_at_end() {
                self.advance();
              }
            } else {
              return;
            }
          }
          None => {
            return;
          }
        },
        _ => {
            return;
          }
      }
    }
  }

  /// Identify if the current slice is a keyword. 
  /// This uses a short of hardcoded trie
  fn identifier_type(&self) -> TokenKind {
    match self.nth_char_from(self.start, 0) {
      Some(c1) => match c1 {
        "a" => self.check_keyword(1, "nd", TokenKind::And),
        "c" => self.check_keyword(1, "lass", TokenKind::Class),
        "e" => self.check_keyword(1, "lse", TokenKind::Else),
        "f" => match self.nth_char_from(self.start, 1) {
          Some(c2) => match c2 {
            "a" => self.check_keyword(2, "lse", TokenKind::False),
            "o" => self.check_keyword(2, "r", TokenKind::For),
            "u" => self.check_keyword(2, "n", TokenKind::Fun),
            _ => TokenKind::Identifier,
          },
          None => TokenKind::Identifier
        },
        "i" => self.check_keyword(1, "f", TokenKind::If),
        "n" => self.check_keyword(1, "il", TokenKind::Nil),
        "o" => self.check_keyword(1, "r", TokenKind::Or),
        "p" => self.check_keyword(1, "rint", TokenKind::Print),
        "r" => self.check_keyword(1, "eturn", TokenKind::Return),
        "s" => self.check_keyword(1, "uper", TokenKind::Super),
        "t" => match self.nth_char_from(self.start, 1) {
          Some(c2) => match c2 {
            "h" => self.check_keyword(2, "is", TokenKind::This),
            "r" => self.check_keyword(2, "ue", TokenKind::True),
            _ => TokenKind::Identifier,
          },
          None => TokenKind::Identifier
        }
        "v" => self.check_keyword(1, "ar", TokenKind::Var),
        "w" => self.check_keyword(1, "hile", TokenKind::While),
        _ => TokenKind::Identifier,
      }
      None => panic!("")
    }
  }

  /// Check if the remainder of the current slice matches the rest
  /// of the keyword
  fn check_keyword(&self, start: usize, rest: &str, kind: TokenKind) -> TokenKind {
    let start_index = self.nth_next_boundary(self.start, start);
    let len = self.current - start_index - 1;

    if len == rest.len() && rest == &self.source[start_index..self.current - 1] {
      return kind;
    }

    TokenKind::Identifier
  }

  /// Make a token from the current state of the scanner
  fn make_token_source(&self, kind: TokenKind) -> Token<'a> {
    self.make_token(kind, self.current_slice(), self.line)
  }

  /// Make a new error token
  fn error_token(&self, message: &'static str) -> Token<'a> {
    self.make_token(TokenKind::Error, message, self.line)
  }

  /// Make a new token
  fn make_token(&self, kind: TokenKind, lexeme: &'a str, line: i32) -> Token<'a> {
    Token { kind, lexeme, line }
  }

  /// Peek the next token
  fn peek_next(&self) -> Option<&str> {
    let start = self.current;
    let end = next_boundary(self.source, self.current);

    if self.char_index_at_end(end) {
      return None;
    }

    Some(&self.source[start..end])
  }

  /// Peek the current token
  fn peek(&self) -> &str {
    &self.source[self.char_start..self.current]
  }

  /// Find the nth char from the current index
  fn nth_char_from(&self, start: usize, n: u8) -> Option<&str> {
    let mut current_index = next_boundary(self.source, start);
    let mut start_index = start;

    for _ in 0..n {
      start_index = current_index;
      current_index = next_boundary(self.source, current_index);
    }

    match self.char_index_at_end(current_index) {
      true => None,
      false => Some(&self.source[start_index..current_index])
    }
  }

  /// Get the current str slice
  fn current_slice(&self) -> &'a str {
    &self.source[self.start..self.current - 1]
  }

  /// Advance the char index forward returning the current
  pub fn advance(&mut self) -> &str {
    let current = &self.source[self.char_start..self.current];
    self.advance_indices();
    current
  }

  /// Advance the housekeeping indices
  fn advance_indices(&mut self) {
    self.char_start = self.current;
    self.current = next_boundary(self.source, self.current);
  }

  /// Find the nth next char boundary
  fn nth_next_boundary(&self, start: usize, n: usize) -> usize {
    let mut current = start;
    for _ in 0..n {
      current = next_boundary(self.source, current);
    }

    current
  }

  /// match the current token against an expected
  fn match_token(&mut self, expected: &str) -> bool {
    if self.is_at_end() {
      return false;
    }

    if self.peek() != expected {
      return false;
    }

    self.advance_indices();
    true
  }

  /// Is the scanner at eof
  fn is_at_end(&self) -> bool {
    self.current > self.source.len()
  }

  /// Is the index at eof
  fn char_index_at_end(&self, index: usize) -> bool {
    index > self.source.len()
  }
}

/// What is the previous char boundary
fn previous_boundary(source: &str, start: usize) -> usize {
  let mut current = start - 1;
  while !source.is_char_boundary(current) && current > 0 {
    current -= 1;
  }

  current
}

/// What is next char boundary
fn next_boundary(source: &str, start: usize) -> usize {
  let mut current = start + 1;
  while !source.is_char_boundary(current) && current < source.len() {
    current += 1;
  }

  current
}

/// Is the str slice a digit. Assumes single char
fn is_digit(c: &str) -> bool {
  c >= "0" && c <= "9"
}

/// Is the str slice a alphabetic. Assumes single char
fn is_alpha(c: &str) -> bool {
  (c >= "a" && c <= "z") || (c >= "A" && c < "Z") || c == "_"
}

#[cfg(test)]
mod test {
  use std::collections::{HashMap};
  use super::*;

  enum TokenGen {
    Symbol(Box<dyn Fn() -> String>),
    Comparator(Box<dyn Fn() -> String>),
    ALpha(Box<dyn Fn() -> String>)
  }

  fn token_gen() -> HashMap<TokenKind, TokenGen> {
    let mut map = HashMap::new();

    map.insert(TokenKind::LeftParen, TokenGen::Symbol(Box::new(|| String::from("("))));
    map.insert(TokenKind::RightParen, TokenGen::Symbol(Box::new(|| String::from(")"))));
    map.insert(TokenKind::LeftBrace, TokenGen::Symbol(Box::new(|| String::from("{"))));
    map.insert(TokenKind::RightBrace, TokenGen::Symbol(Box::new(|| String::from("}"))));
    map.insert(TokenKind::Comma, TokenGen::Symbol(Box::new(|| String::from(","))));
    map.insert(TokenKind::Dot, TokenGen::Symbol(Box::new(|| String::from("."))));
    map.insert(TokenKind::Minus, TokenGen::Symbol(Box::new(|| String::from("-"))));
    map.insert(TokenKind::Plus, TokenGen::Symbol(Box::new(|| String::from("+"))));
    map.insert(TokenKind::Semicolon, TokenGen::Symbol(Box::new(|| String::from(";"))));
    map.insert(TokenKind::Slash, TokenGen::Symbol(Box::new(|| String::from("/"))));
    map.insert(TokenKind::Star, TokenGen::Symbol(Box::new(|| String::from("*"))));

    map.insert(TokenKind::Bang, TokenGen::Comparator(Box::new(|| String::from("!"))));
    map.insert(TokenKind::BangEqual, TokenGen::Comparator(Box::new(|| String::from("!="))));
    map.insert(TokenKind::Equal, TokenGen::Comparator(Box::new(|| String::from("="))));
    map.insert(TokenKind::EqualEqual, TokenGen::Comparator(Box::new(|| String::from("=="))));
    map.insert(TokenKind::Greater, TokenGen::Comparator(Box::new(|| String::from(">"))));
    map.insert(TokenKind::GreaterEqual, TokenGen::Comparator(Box::new(|| String::from(">="))));
    map.insert(TokenKind::Less, TokenGen::Comparator(Box::new(|| String::from("<"))));
    map.insert(TokenKind::LessEqual, TokenGen::Comparator(Box::new(|| String::from("<="))));

    map.insert(TokenKind::Identifier, TokenGen::ALpha(Box::new(|| String::from("example"))));
    map.insert(TokenKind::String, TokenGen::Symbol(Box::new(|| String::from("\"example\""))));
    map.insert(TokenKind::Number, TokenGen::ALpha(Box::new(|| String::from("12345"))));

    map.insert(TokenKind::And, TokenGen::ALpha(Box::new(|| String::from("and"))));
    map.insert(TokenKind::Class, TokenGen::ALpha(Box::new(|| String::from("class"))));
    map.insert(TokenKind::Else, TokenGen::ALpha(Box::new(|| String::from("else"))));
    map.insert(TokenKind::False, TokenGen::ALpha(Box::new(|| String::from("false"))));
    map.insert(TokenKind::For, TokenGen::ALpha(Box::new(|| String::from("for"))));
    map.insert(TokenKind::Fun, TokenGen::ALpha(Box::new(|| String::from("fun"))));
    map.insert(TokenKind::If, TokenGen::ALpha(Box::new(|| String::from("if"))));
    map.insert(TokenKind::Nil, TokenGen::ALpha(Box::new(|| String::from("nil"))));
    map.insert(TokenKind::Or, TokenGen::ALpha(Box::new(|| String::from("or"))));
    map.insert(TokenKind::Print, TokenGen::ALpha(Box::new(|| String::from("print"))));
    map.insert(TokenKind::Return, TokenGen::ALpha(Box::new(|| String::from("return"))));
    map.insert(TokenKind::Super, TokenGen::ALpha(Box::new(|| String::from("super"))));
    map.insert(TokenKind::This, TokenGen::ALpha(Box::new(|| String::from("this"))));
    map.insert(TokenKind::True, TokenGen::ALpha(Box::new(|| String::from("true"))));
    map.insert(TokenKind::Var, TokenGen::ALpha(Box::new(|| String::from("var"))));
    map.insert(TokenKind::While, TokenGen::ALpha(Box::new(|| String::from("while"))));
    map.insert(TokenKind::Error, TokenGen::ALpha(Box::new(|| String::from("$$"))));
    map.insert(TokenKind::Eof, TokenGen::ALpha(Box::new(|| String::from(""))));

    map
  }

  #[test]
  fn test_single_token() {
    for (token_kind, gen) in token_gen() {
      let example = match gen {
        TokenGen::Symbol(x) => x(),
        TokenGen::ALpha(x) => x(),
        TokenGen::Comparator(x) => x(),
      };

      let mut scanner = Scanner::new(&example);
      let scanned_token = scanner.scan_token().clone();
      assert_eq!(scanned_token.kind, token_kind.clone());
    }
  }

  #[test]
  fn test_multiple_tokens() {
    let basic = "10 + 3";
    let mut scanner = Scanner::new(basic);

    let token_ten = scanner.scan_token().clone();
    assert_eq!(token_ten.kind, TokenKind::Number);
    assert_eq!(token_ten.lexeme, "10");

    let token_plus = scanner.scan_token().clone();
    assert_eq!(token_plus.kind, TokenKind::Plus);
    assert_eq!(token_plus.lexeme, "+");

    let token_three = scanner.scan_token().clone();
    assert_eq!(token_three.kind, TokenKind::Number);
    assert_eq!(token_three.lexeme, "3");

    let token_eof = scanner.scan_token().clone();
    assert_eq!(token_eof.kind, TokenKind::Eof);
    assert_eq!(token_eof.lexeme, "");
  }
}