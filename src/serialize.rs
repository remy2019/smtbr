use crate::types::{MultipleChoice, Problem};
use dotext::{Docx, MsDoc};
use regex::Regex;
use std::io::Read;

pub fn serialize_problems() -> Result<(), Box<dyn std::error::Error>> {
    let re_problems = Regex::new(r"(?mU)\d\d?\d?\)(.|\n)+AACSB")?;
    let re_partition = Regex::new(
        r"(?mU)(?<number>\d\d?\d?)\)\s(?<question>.*\n)(?<choice>(.|\n)*)Answer:(?<answer>.*)\n",
    )?;
    let re_mcq =
        Regex::new(r"(?m)A\)\s(?<A>.+)\n*B\)\s(?<B>.+)\n*C\)\s(?<C>.+)\n*D\)\s(?<D>.+)\n*")?;
    let mut problem_chunk = vec![];
    let mut prob_index: u32 = 1;

    for i in 1..=12 {
        let mut problem_set = vec![];

        let exe = std::env::current_exe()?;
        let dir = exe.parent().expect("Executable must be in some directory");
        let dir = dir.join("tb");

        let tb = dir.join(format!("barney_smca6_tif_{:02}.docx", i));
        let mut file = Docx::open(tb)?;
        let mut text = String::new();
        let _ = file.read_to_string(&mut text);
        let mut iter = re_problems.captures_iter(&text);
        while let Some(problem) = iter.next() {
            problem_set.push(String::from(&problem[0]));
        }

        for problem in problem_set {
            problem_chunk.push(extract_problem(
                &problem,
                &re_partition,
                &re_mcq,
                i,
                prob_index,
            )?);
            prob_index += 1;
        }
    }

    to_yaml(problem_chunk)?;
    Ok(())
}

fn extract_problem(
    s: &str,
    re_partition: &Regex,
    re_mcq: &Regex,
    chapter: u32,
    ab: u32,
) -> Result<Problem, Box<dyn std::error::Error>> {
    let chapter = chapter;
    let prob_num_ab = ab;

    let caps = re_partition.captures(s).unwrap();
    let prob_num_rel = caps
        .name("number")
        .unwrap()
        .as_str()
        .trim()
        .parse::<u32>()?;
    let question = caps.name("question").unwrap().as_str().trim().to_owned();
    let answer = caps.name("answer").unwrap().as_str().trim().to_owned();
    let choice = caps.name("choice").unwrap().as_str().trim();
    let mcq: bool;
    let tf: bool;
    let mc: Option<MultipleChoice>;

    (mcq, tf, mc) = if choice.is_empty() {
        if answer.contains("TRUE") || answer.contains("FALSE") {
            (false, true, None)
        } else {
            (false, false, None)
        }
    } else {
        let caps = re_mcq.captures(choice).unwrap();
        let abcd = Some(MultipleChoice::new(
            caps.name("A").unwrap().as_str().trim().to_owned(),
            caps.name("B").unwrap().as_str().trim().to_owned(),
            caps.name("C").unwrap().as_str().trim().to_owned(),
            caps.name("D").unwrap().as_str().trim().to_owned(),
        ));
        (true, false, abcd)
    };

    let extracted_problem = Problem::new(
        chapter,
        prob_num_rel,
        prob_num_ab,
        mcq,
        tf,
        mc,
        question,
        answer,
    );
    Ok(extracted_problem)
}

pub fn to_yaml(problems: Vec<Problem>) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.yml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &problems)?;
    Ok(())
}
