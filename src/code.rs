use token::Token;

#[derive(Debug)]
pub enum Code {
    Label(Token),
    AInstruction(Token),
    CInstruction { dest: Vec<Token>, comp: Expression, jump: Option<Token> }
}

#[derive(Debug)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    Literal(Token),
}