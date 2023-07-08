use crate::types::*;
use inquire::{Confirm, InquireError, Select};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn study(
    rand_opt: bool,
    mcq_opt: String,
    chapters: Vec<u32>,
) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    let data_string = &std::fs::read_to_string("test.yml")?;
    let data = deserialize(data_string)?;

    let mut selected_data: Vec<Problem> = vec![];

    // filter appropriate problems
    for problem in data {
        if !(chapters.contains(&problem.chapter) && (problem.mcq || problem.tf)) {
            continue;
        };
        match mcq_opt.as_str() {
            "Only MCQ" => {
                if problem.mcq {
                    selected_data.push(problem)
                }
            }
            "Only TF" => {
                if problem.tf {
                    selected_data.push(problem)
                }
            }
            "MCQ and TF" => selected_data.push(problem),
            _ => panic!("unexpected error"),
        }
    }

    if !rand_opt {
        start_study(&selected_data);
        return Ok(selected_data);
    }

    // optional randomization
    // firstly, shuffle in chunk
    let mut rng = thread_rng();
    selected_data.shuffle(&mut rng);

    // secondly, shuffle choice and adjust answers of mcq
    for problem in &mut selected_data {
        if problem.tf {
            continue;
        }
        // get answer string
        let answer_str = String::from(match problem.answer.as_str() {
            "A" => problem.mc.as_ref().unwrap().a.as_str(),
            "B" => problem.mc.as_ref().unwrap().b.as_str(),
            "C" => problem.mc.as_ref().unwrap().c.as_str(),
            "D" => problem.mc.as_ref().unwrap().d.as_str(),
            _ => panic!("unexpected error"),
        });

        if let Some(ref mut s) = problem.mc {
            let mut choices = vec![s.a.clone(), s.b.clone(), s.c.clone(), s.d.clone()];
            choices.shuffle(&mut rng);

            if answer_str == choices[0] {
                problem.answer = "A".to_owned();
            } else if answer_str == choices[1] {
                problem.answer = "B".to_owned();
            } else if answer_str == choices[2] {
                problem.answer = "C".to_owned();
            } else {
                problem.answer = "D".to_owned();
            }

            s.a = String::from(&choices[0]);
            s.b = String::from(&choices[1]);
            s.c = String::from(&choices[2]);
            s.d = String::from(&choices[3]);
        }
    }

    start_study(&selected_data);
    Ok(selected_data)
}

fn start_study(problems: &[Problem]) {
    let length = problems.len();

    for (i, problem) in problems.iter().enumerate() {
        println!("\n\n\n");
        println!("{}", "\u{2500}".repeat(20));
        println!("Progress: {} of {}\n", i + 1, length);
        if problem.tf {
            test_tf(i, problem);
        } else {
            test_mcq(i, problem);
        }
    }

    println!("{}", "\u{2500}".repeat(20));
    println!("Finished!");
}

fn test_tf(i: usize, problem: &Problem) {
    let options = vec!["A) TRUE", "B) FALSE"];
    let ans = Select::new(problem.question.as_str(), options)
        .prompt()
        .unwrap();
    match ans.contains(problem.answer.as_str()) {
        true => println!("You got the answer!"),
        false => {
            println!("\nWrong. \nThe answer is {}.", problem.answer);
            println!("{}", "\u{2500}".repeat(20));
            let ans = Confirm::new("Move on to the next question?")
                .with_default(true)
                .with_help_message("If no, you will quit the program.")
                .prompt()
                .unwrap();
            match ans {
                true => (),
                false => panic!("You chose to quit"),
            }
        }
    }
}

fn test_mcq(i: usize, problem: &Problem) {
    if let Some(s) = &problem.mc {
        let options = vec![
            format!("A) {}", s.a),
            format!("B) {}", s.b),
            format!("C) {}", s.c),
            format!("D) {}", s.d),
        ];
        let correct_ans = match problem.answer.as_str() {
            "A" => &options[0],
            "B" => &options[1],
            "C" => &options[2],
            "D" => &options[3],
            _ => panic!("unexpected error"),
        }
        .clone();
        let mut ans = Select::new(problem.question.as_str(), options)
            .prompt()
            .unwrap();
        ans.truncate(1);

        match ans == problem.answer {
            true => println!("You got the answer!"),
            false => {
                println!("\nWrong.\nThe answer is {}", correct_ans);
                println!("{}", "\u{2500}".repeat(20));
                let ans = Confirm::new("Move on to the next question?")
                    .with_default(true)
                    .with_help_message("If no, you will quit the program.")
                    .prompt()
                    .unwrap();
                match ans {
                    true => (),
                    false => panic!("You chose to quit"),
                }
            }
        }
    }
}

fn deserialize(s: &str) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_str::<Vec<Problem>>(s)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract() {
        let t = study(true, "MCQ and TF".to_string(), vec![1]).unwrap();
        start_study(&t);
    }
}
