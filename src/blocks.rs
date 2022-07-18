
// intermediate representation

pub trait Block {}

pub struct UtteranceBlock {
    pub content: String
}
impl Block for UtteranceBlock {}

pub struct EndCommandBlock {
}
impl Block for EndCommandBlock {}

