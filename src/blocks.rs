
// intermediate representation

#[derive(Debug)]
pub enum Block {
    Utterance {
        content: String,
    },
    SetCommand {
        id: String,
        value: String,
    },
    EndCommand,
}

