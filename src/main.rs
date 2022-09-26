mod database;

use clap::{Parser, Subcommand};
// use inquire::{list_option::ListOption, InquireError, MultiSelect, Text};
// use crate::database::{create_connection, create_tables};


#[derive(Subcommand, Debug)]
enum Commands {
    Query,
    #[clap(subcommand)]
    Add(AddArgs)
}

#[derive(Subcommand, Debug)]
enum AddArgs {
    Thought,
    Feeling
}

/// Register your automatic thoughts
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Bin {
    #[clap(subcommand)]
    command: Commands
}

// #[derive(Debug)]
// struct Thought {
//     feeling: String,
//     situation: String,
//     thought: String,
//     intensity: i32
// }



fn main() {

    // let conn = create_connection();
    // create_tables(&conn);

    // let app = App::new("rpd")
    //     .author("joseemds")
    //     .version("0.0.1")
    //     .about("Register you involuntary thoughts without exiting the terminal")
    //     .subcommand(SubCommand::with_name("register").about("add a feeling"))
    //     .get_matches();

    //     match app.subcommand() {
    //     Some(("register", _)) | None => {
    //        let mut feelings_query = conn.prepare("SELECT id, name FROM feelings").unwrap();

    //        let feelings_stmt = feelings_query.query_map([], |row| {
    //             Ok(ListOption::new(row.get(0).unwrap(), row.get(1).unwrap()))
    //        }).unwrap();
           
    //        let mut feelings: Vec<ListOption<String>> = vec![];

    //        for feeling in feelings_stmt.into_iter() {
    //         feelings.push(feeling.unwrap())
    //        }

    //         let feeling = MultiSelect::new("What are you feeling now", feelings)
    //             .prompt()
    //             .map_err(|err| match err {
    //                 InquireError::OperationInterrupted | InquireError::OperationCanceled => {
    //                     std::process::exit(1)
    //                 }
    //                 err => err,
    //             })
    //             .unwrap();

    //         let situation = Text::new("What happened that made you feel like this?")
    //             .prompt()
    //             .map_err(|err| match err {
    //                 InquireError::OperationInterrupted | InquireError::OperationCanceled => {
    //                     std::process::exit(1)
    //                 }
    //                 err => err,
    //             })
    //             .unwrap();
    //         let thought = Text::new("What do you think about this?")
    //             .prompt()
    //             .map_err(|err| match err {
    //                 InquireError::OperationInterrupted | InquireError::OperationCanceled => {
    //                     std::process::exit(1)
    //                 }
    //                 err => err,
    //             })
    //             .unwrap();
    //         let intensity = Text::new("How intensive is that feeling in a scale of [1-10]")
    //             .prompt()
    //             .map_err(|err| match err {
    //                 InquireError::OperationInterrupted | InquireError::OperationCanceled => {
    //                     std::process::exit(1)
    //                 }
    //                 err => err,
    //             })
    //             .unwrap();
    //     }
    //     _ => {
    //         panic!("Unexpected Command")
    //     }
    // }

    let app = Bin::parse();

    match app.command {
        Commands::Query => panic!("todo"),
        Commands::Add(_) => panic!("todo")
    }


}
