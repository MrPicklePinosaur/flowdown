use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
enum BlockType {
    Speak,
}

#[derive(Serialize, Debug)]
struct Block {
    #[serde(rename(serialize = "type"))]
    block_type: BlockType,
    random_speak: Vec<String>,
}

fn parse(input: &str) {
    let mut output: Vec<Block> = vec![];
    let mut utterance = String::new();

    for line in input.lines() {

        // utterence ends when newline
        if line.is_empty() && !utterance.is_empty() {
            output.push(Block {
                block_type: BlockType::Speak,
                random_speak: vec![utterance.drain(..).collect()],
            });
        }
        let delim = if utterance.is_empty() { "" } else { " " };
        utterance += &format!("{}{}", delim, line);
    }
    println!("{:?}", output);
}

const INPUT: &str = r#"
hello world!

hello
world
!

"#;

fn main() {
    parse(INPUT);
}
