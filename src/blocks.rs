// intermediate representation

#[derive(Debug, Clone)]
pub enum JumpTarget {
    Bookmark(String),
    Dialog(String),
}

#[derive(Debug)]
pub enum Operator {
    Equals,
    NotEquals,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equals => write!(f, "equals"),
            Operator::NotEquals => write!(f, "not_equal"),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Variable(String),
    Literal(String),
}

#[derive(Debug)]
pub enum Block {
    Jump {
        target: JumpTarget,
    },
    Utterance {
        voice: Option<String>,
        content: String,
    },
    SetCommand {
        variable: String,
        value: String,
    },
    CaptureCommand {
        variable: String,
    },
    CodeCommand {
        body: String,
    },
    IfCommand {
        operator: Operator,
        op1: Operand,
        op2: Operand,
    },
    EndCommand,
}
