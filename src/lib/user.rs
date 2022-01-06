use crate::lib::basic::Level;

#[allow(unused)]
struct Answer {
    correct: bool,
    string_value: &'static str,
}

#[derive(Debug)]
pub enum Ranking {
    Noob,
    Beginner,
    Padawan,
    Knight,
    Sage,
    Master,
    // Pro, Master, Hacker
}

#[derive(Debug)]
pub struct User {
    pub correct: u8,
    pub wrong: u8,
    pub ranking: Ranking,
    pub points: i16,
}

/// Represents the end user - the person playing the regex game. Keeps track of all relevant data during
/// their journey.
///
#[allow(unused)]
impl User {
    pub fn default() -> Self {
        Self {
            correct: 0,
            wrong: 0,
            ranking: Ranking::Noob,
            points: 0,
        }
    }

    /// Called when a user answers correctly
    /// qta = (Q)uestion (T)ype (A)nswered - Can be Easy, Medium, or Hard
    /// Depending on the level of difficulty of their question, this will determine how much to
    /// increment their score by.
    pub fn increment(&mut self, qta: &Level, incr: bool) {
        self.points += match qta {
            Level::Easy => {
                if incr {
                    5
                } else {
                    -5
                }
            }
            Level::Medium => {
                if incr {
                    10
                } else {
                    -10
                }
            }
            Level::Hard => {
                if incr {
                    15
                } else {
                    -15
                }
            }
        }
    }
    /**
     * Get percentage total win of correct scores
     * @param self: User
     * @returns final pct: f32
     */
    pub fn pct(&self) -> f32 {
        let total = (self.correct + self.wrong) as f32;
        if total == 0.0 {
            // If total is 0, compiler will see it as NaN
            return 0.0;
        } else {
            (self.correct as f32 / total) * 100.00
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

#[cfg(test)]
mod tests {
    use crate::user::{Ranking, User};

    #[test]
    fn test_pct() {
        let user1 = User {
            correct: 15,
            wrong: 15,
            ranking: Ranking::Noob,
            points: 0,
        };
        assert_eq!(user1.pct(), 50.0);

        let user2 = User {
            correct: 66,
            wrong: 6,
            ranking: Ranking::Noob,
            points: 12,
        };
        assert_eq!(user2.pct(), 91.66667);
    }
}
