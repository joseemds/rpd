use clap::{App, Arg, Parser, SubCommand};
use inquire::{list_option::ListOption, MultiSelect, Text};
fn main() {
    let app = App::new("rpd")
        .author("joseemds")
        .version("0.0.1")
        .about("Register you involuntary thoughts without exiting the terminal")
        .subcommand(SubCommand::with_name("register").about("add a feeling"))
        .get_matches();

    println!("App: {:?}", app);
    match app.subcommand(){
        Some(("register", _))
        | None => {
            let options = vec![
                ListOption::new(0, &"sad"),
                ListOption::new(1, &"angry"),
                ListOption::new(2, &"frustated"),
            ];
            let feeling = MultiSelect::new("What are you feeling now", options).prompt();
            let situation = Text::new("What happened that made you feel like this?").prompt();
            let thought = Text::new("What do you think about this?").prompt();
            let intensity =
                Text::new("How intensive is that feeling in a scale of [1-10]").prompt();
        }
        _ => {
            panic!("Unexpected Command")
        }
    }
}
