use crate::basic::Level;

#[allow(unused)]
struct User {
    score: u16,
    correct: u8,
    wrong: u8,

    //TODO: Implement a ranking system
    //ranking: ??

    //answers
}

/// Represents the end user - the person playing the regex game. Keeps track of all relevant data during
/// their journey.
///
#[allow(unused)]
impl User {

    /// Called when a user answers correctly
    /// qta = (Q)uestion (T)ype (A)nswered - Can be Easy, Medium, or Hard
    /// Depending on the level of difficulty of their question, this will determine how much to
    /// increment their score by.
    fn increment(&mut self, qta: Level) {
        //TODO: Current implementation is basic and not correct, see above documentation
        self.score += 1;
    }

    fn pct(&self) -> f32 {
        //TODO: Returns a pct % based on correct / wrong ratio.
        unimplemented!()
    }
}

//TODO: This will need to be persisted
#[allow(unused)]
struct Answer {
    correct: bool,
    string_value: &'static str,
}