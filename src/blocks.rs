
// intermediate representation

#[derive(Debug)]
pub enum Block {
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

