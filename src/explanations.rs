/// This module defines the `Explanation` instance. An `Explanation` is stored information
/// about a given regex topic that the user can read about on demand.

#[allow(unused)]
struct Explanation {
    topic: RegexTopic,
    description: &'static str,

}

#[allow(unused)]
enum RegexTopic {
    CharacterClass(&'static str),

}

#[allow(unused)]
const DIGITS: Explanation = Explanation { topic: RegexTopic::CharacterClass(r"\d"), description: "test"};