/// This module defines the `Explanation` instance. An `Explanation` is stored information
/// about a given regex topic that the user can read about on demand.

struct Explanation {
    topic: RegexTopic,
    description: &str,

}

enum RegexTopic {
    CharacterClass(&str),

}

const DIGITS: Explanation = Explanation { topic: RegexTopic::CharacterClass(r"\d"), description: "test"};