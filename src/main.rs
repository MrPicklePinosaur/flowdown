use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Options};

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
# hello world!

hello
world
!

my list
- item 1
- item 2
- item 3
"#;

fn main() {
    // parse(INPUT);

    let options = Options::empty();
    let parser = Parser::new_ext(INPUT, options);

    parser.for_each(|event| println!("event {:?}", event));
}
