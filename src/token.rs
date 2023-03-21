enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}

#[derive(Default)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: i32,
}

impl Token {
       
}

impl<T> ToString for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{token_type} {lexeme} {literal}")
    }
}
