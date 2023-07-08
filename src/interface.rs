// use crate::deserialize::get_problems;
use crate::{deserialize::study, serialize::serialize_problems};
use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, Confirm,
    InquireError, MultiSelect, Select,
};

pub fn newcomer() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("clear").status().unwrap();
    println!("{}", "\u{2500}".repeat(20));
    println!("Welcome to SMTBR, a Strategy Management Test Bank study helper with problem Randomization.");
    println!();
    println!("Since it's the first time you're using SMTBR, data transformation works are needed.");
    println!();
    let ans = Confirm::new("Are your test bank docx files ready?")
        .with_default(false)
        .with_help_message("This procedure would take about a second.")
        .prompt();

    match ans {
        Ok(true) => {
            println!("That's awesome! Please wait a second...");
            match serialize_problems() {
                Ok(_) => {
                    println!("Done!");
                    home()?
                }
                Err(_) => {
                    _ = std::fs::remove_file("test.yml");
                    println!(
                        "Error occurred while processing! Make sure that you followed the manual: "
                    )
                }
            }
        }
        Ok(false) => println!("You can read the manual from: www.naver.com"),
        Err(_) => println!("Error with questionnaire, try again later"),
    }
    Ok(())
}

pub fn home() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("{}", "\u{2500}".repeat(20));

    let options: Vec<&str> = vec![
        "Study -> Midterm exam",
        "Study -> Final exam",
        "Help",
        "Factory reset",
        "Contact developer",
        "Quit",
    ];

    println!("Welcome to SMTBR!\n");
    let ans: Result<&str, InquireError> =
        Select::new("What are you going to do?", options).prompt();

    match ans {
        Ok("Help") => {
            println!("You can read the manual from: www.naver.com");
            home()?;
        }
        Ok("Quit") => {
            println!("Bye!");
        }
        Ok("Contact developer") => {
            println!("Email me: remy2019@gmx.us");
            home()?;
        }
        Ok("Factory reset") => {
            _ = std::fs::remove_file("test.yml");
            println!("Reset done! Restarting...");
            newcomer()?;
        }
        Ok("Study -> Midterm exam") => {
            println!("Let's study midterm exam!");
            let rand_opt = ask_rand().unwrap();
            let mcq_opt = ask_mcq().unwrap();

            let chapters: Vec<String> = (1..=6).map(|x| x.to_string()).collect();
            let sel_chapters = chap_sel(chapters)?;
            study(rand_opt, mcq_opt, sel_chapters)?;
        }
        Ok("Study -> Final exam") => {
            println!("Let's study final exam!");
            let rand_opt = ask_rand().unwrap();
            let mcq_opt = ask_mcq().unwrap();

            let chapters: Vec<String> = (7..=12).map(|x| x.to_string()).collect();
            let sel_chapters = chap_sel(chapters)?;
            study(rand_opt, mcq_opt, sel_chapters)?;
        }
        _ => println!("There was an error, please try again"),
    };
    Ok(())
}

fn ask_rand() -> Result<bool, Box<dyn std::error::Error>> {
    println!("{}", "\u{2500}".repeat(20));
    let options: Vec<&str> = vec!["randomize!", "do not randomize."];
    let ans: Result<&str, InquireError> =
        Select::new("Do you want problems order get randomized?", options).prompt();
    match ans {
        Ok("randomize!") => Ok(true),
        Ok("do not randomize.") => Ok(false),
        _ => panic!("unknown error"),
    }
}

fn ask_mcq() -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", "\u{2500}".repeat(20));
    let options: Vec<&str> = vec!["MCQ and TF", "Only MCQ", "Only TF"];
    let ans: Result<&str, InquireError> =
        Select::new("Which type of problems do you want to study?", options).prompt();
    match ans {
        Ok(opt) => Ok(opt.to_owned()),
        _ => panic!("unknown error"),
    }
}

fn chap_sel(s: Vec<String>) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    println!("{}", "\u{2500}".repeat(20));
    let options = s.iter().map(|x| x.as_str()).collect();

    let formatter: MultiOptionFormatter<'_, &str> = &|a| {
        format!(
            "{:?} chapters selected.",
            a.iter()
                .map(|x| (*(x.value)).to_owned())
                .collect::<Vec<String>>()
        )
    };

    let ans = MultiSelect::new("Select chapters to study:", options)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(x) => {
            if x.is_empty() {
                println!("You should select at least one chapter.");
                home()?;
            }
            return Ok(x.iter().map(|&x| x.parse::<u32>().unwrap()).collect());
        }
        Err(_) => println!("Chapter list could not be processed"),
    }
    panic!("error");
}
