use clap::{App, Parser, SubCommand};
use inquire::{list_option::ListOption, InquireError, MultiSelect, Text};

/// Register your automatic thoughts
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

// #[derive(Debug)]
// struct Thought {
//     feeling: String,
//     situation: String,
//     thought: String,
//     intensity: i32
// }

fn main() {
    let app = App::new("rpd")
        .author("joseemds")
        .version("0.0.1")
        .about("Register you involuntary thoughts without exiting the terminal")
        .subcommand(SubCommand::with_name("register").about("add a feeling"))
        .get_matches();

    println!("App: {:?}", app);
    // let args = Args::parse();
    // let _ = Thought {feeling: String::from("bad"), situation: String::from("really bad"), thought: String::from("im a piece of shit"), intensity: 10};
    match app.subcommand() {
        Some(("register", _)) | None => {
            let options = vec![
                ListOption::new(0, &"sad"),
                ListOption::new(1, &"angry"),
                ListOption::new(2, &"frustated"),
            ];

            let feeling = MultiSelect::new("What are you feeling now", options)
                .prompt()
                .map_err(|err| match err {
                    InquireError::OperationInterrupted | InquireError::OperationCanceled => {
                        std::process::exit(1)
                    }
                    err => err,
                })
                .unwrap();

            let situation = Text::new("What happened that made you feel like this?")
                .prompt()
                .map_err(|err| match err {
                    InquireError::OperationInterrupted | InquireError::OperationCanceled => {
                        std::process::exit(1)
                    }
                    err => err,
                })
                .unwrap();
            let thought = Text::new("What do you think about this?")
                .prompt()
                .map_err(|err| match err {
                    InquireError::OperationInterrupted | InquireError::OperationCanceled => {
                        std::process::exit(1)
                    }
                    err => err,
                })
                .unwrap();
            let intensity = Text::new("How intensive is that feeling in a scale of [1-10]")
                .prompt()
                .map_err(|err| match err {
                    InquireError::OperationInterrupted | InquireError::OperationCanceled => {
                        std::process::exit(1)
                    }
                    err => err,
                })
                .unwrap();
        }
        _ => {
            panic!("Unexpected Command")
        }
    }
}
