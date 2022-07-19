
// intermediate representation

#[derive(Debug)]
pub enum Block {
    Utterance {
        content: String,
    },
    EndCommand,
}

