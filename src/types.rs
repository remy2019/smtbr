use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MultipleChoice {
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Problem {
    pub chapter: u32,
    pub prob_num_rel: u32,
    pub prob_num_ab: u32,
    pub mcq: bool,
    pub tf: bool,
    pub mc: Option<MultipleChoice>,
    pub question: String,
    pub answer: String,
}

impl MultipleChoice {
    pub fn new(a: String, b: String, c: String, d: String) -> Self {
        MultipleChoice { a, b, c, d }
    }
}

impl Problem {
    pub fn new(
        chapter: u32,
        prob_num_rel: u32,
        prob_num_ab: u32,
        mcq: bool,
        tf: bool,
        mc: Option<MultipleChoice>,
        question: String,
        answer: String,
    ) -> Self {
        Problem {
            chapter,
            prob_num_rel,
            prob_num_ab,
            mcq,
            tf,
            mc,
            question,
            answer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_new() {
        let test = Problem {
            chapter: 12,
            prob_num_rel: 4,
            prob_num_ab: 99,
            mcq: true,
            tf: false,
            mc: Some(MultipleChoice {
                a: ("a".to_owned()),
                b: ("b".to_owned()),
                c: ("c".to_owned()),
                d: ("d".to_owned()),
            }),
            question: "as ________ profits.".to_owned(),
            answer: "A".to_owned(),
        };
        let mc_new = Some(MultipleChoice::new(
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
            "d".to_owned(),
        ));

        assert_eq!(format!("{:?}", test.mc), format!("{:?}", mc_new));

        let problem_new = Problem::new(
            12,
            4,
            99,
            true,
            false,
            mc_new,
            "as ________ profits.".to_owned(),
            "A".to_owned(),
        );

        assert_eq!(format!("{:?}", test), format!("{:?}", problem_new));
    }
}
