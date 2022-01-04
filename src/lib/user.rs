use crate::lib::basic::Level;

pub struct User {
    pub correct: u8,
    pub wrong: u8,
    pub ranking: Ranking,
    pub score: u32,
}

/// Represents the end user - the person playing the regex game. Keeps track of all relevant data during
/// their journey.
///
#[allow(unused)]
impl User {
    /// Creates a new user from scratch
    pub fn new() -> User {
        User {
            correct: 0,
            wrong: 0,
            ranking: Ranking::Noob,
            score: 0,
        }
    }

    /// Called when a user answers correctly
    /// qta = (Q)uestion (T)ype (A)nswered - Can be Easy, Medium, or Hard
    /// Depending on the level of difficulty of their question, this will determine how much to
    /// increment their score by.
    pub fn increment(&mut self, qta: Level) {
        //TODO: Current implementation is basic and not correct, see above documentation
        match qta{
            Level::Easy => self.score += 25,
            Level::Medium => self.score += 50,
            Level::Hard => self.score += 75,
        }
    }

    pub fn pct(&self) -> f32 {
        let total = (self.correct + self.wrong) as f32;
        if total == 0.0 {
            // If total is 0, compiler will see it as NaN
            0.0
        } else {
            self.correct as f32 / total
        }
    }

    pub fn get_user_ranking(&self) -> &str {
        match self.ranking {
            Ranking::Noob => "Noob",
            Ranking::Beginner => "Beginner",
            Ranking::Padawan => "Padawan",
            Ranking::Knight => "Knight",
            Ranking::Sage => "Sage",
            Ranking::Master => "Master",
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let total: i8 = (self.correct as i8) - (self.wrong as i8);
        if total < 0 {
            0u32
        } else {
            (total * 100) as u32
        }
    }
}

//TODO: This will need to be persisted
#[allow(unused)]
struct Answer {
    correct: bool,
    string_value: &'static str,
}

pub enum Ranking {
    Noob,
    Beginner,
    Padawan,
    Knight,
    Sage,
    Master,
    // Pro, Master, Hacker
}

#[cfg(test)]
mod tests {
    use crate::user::{Ranking, User};

    #[test]
    fn test_pct() {
        let user1 = User {
            correct: 15,
            wrong: 15,
            ranking: Ranking::Noob,
            score: 0,
        };
        assert_eq!(user1.pct(), 0.50)
    }
}
