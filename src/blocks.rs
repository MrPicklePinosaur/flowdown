
// intermediate representation

#[derive(Debug)]
pub enum Block {
    Utterance {
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

