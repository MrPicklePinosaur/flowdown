
// intermediate representation

#[derive(Debug)]
pub enum Block {
    Start,
    Utterance {
        content: String,
    },
    EndCommand,
}

