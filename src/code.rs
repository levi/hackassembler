use token::Token;

pub trait Code {
    fn binary() -> String;
}

/// Label
pub struct Label {
    token: Token,
}

impl Label {
    pub fn new(token: Token) -> Label {
        Label{
            token: token,
        }
    }
}

impl Code for Label {
    fn binary() -> String {
        String::new()
    }
}

/// A Instruction
pub struct AInstruction {
    token: Token,
}

impl AInstruction {
    fn new(token: Token) -> Code {
        AInstruction{
            token: token,
        }
    }
}

impl Code for AInstruction {
    fn binary() -> String {
        String::new()
    }
}

/// C Instruction
pub struct CInstruction {
   dest: Vec<Token>,
   comp: Comp,
   jump: Option<Token>, 
}

impl CInstruction {
    pub fn new(dest: Vec<Token>, comp: Comp, jump: Option<Token>) -> CInstruction {
        CInstruction{
            dest: dest,
            comp: comp,
            jump: jump,
        }
    }
}

impl Code for CInstruction {
    fn binary() -> String {
        String::new()
    }
}

/// Comp
pub struct Comp {
    pub left: Option<Token>,
    pub operator: Option<Token>,
    pub right: Token,
}

impl Comp {
    pub fn new(left: Option<Token>, operator: Option<Token>, right: Token) -> Comp {
        Comp{
            left: left,
            operator: operator,
            right: right,
        }
    }
}