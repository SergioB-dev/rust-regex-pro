pub mod preset_questions {
    use crate::basic::Level;
    use crate::questions::{FillerOrder, Question};
    use crate::user::Ranking;

    pub const NOOB_QUESTIONS: [Question;3] = [
        Question {
            explanation: "",
            search_string: "2004-10-10",
            filler_string: None,
            filler_order: FillerOrder::Void,
            points: 0,
            ranking: Ranking::Noob
        },
        Question {
            explanation: "",
            search_string: "(202)389-1200",
            filler_string: Some("My number is"),
            filler_order: FillerOrder::Before,
            points: 0,
            ranking: Ranking::Noob
        },
        Question {
            explanation: "",
            search_string: "J-502",
            filler_string: Some("He lives in apartment"),
            filler_order: FillerOrder::Before,
            points: 0,
            ranking: Ranking::Noob
        }
     ];
}