// intermediate representation

#[derive(Debug, Clone)]
pub enum JumpTarget {
    Bookmark(String),
    Dialog(String),
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
    EndCommand,
}
