use kingslayer::CmdTokens;

pub struct Parser;

impl Parser {
    pub fn parse(words: CmdTokens) -> String {
        if let Some(verb) = words.verb() {
            match verb {
                _ => String::from("That doesn't make any sense."),
            }
        } else {
            String::from("I don't understand.")
        }
    }
}
