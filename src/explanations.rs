/// This module defines the `Explanation` instance. An `Explanation` is stored information
/// about a given regex topic that the user can read about on demand.

struct Explanation {
    topic: RegexTopic,
    description: &'static str,

}

enum RegexTopic {
    CharacterClass(&'static str),

}

const DIGITS: Explanation = Explanation { topic: RegexTopic::CharacterClass(r"\d"), description: "test"};