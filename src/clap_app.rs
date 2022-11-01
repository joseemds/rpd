use clap::Command;

pub fn create_app() -> Command<'static> {
    Command::new("rpd")
        .author("joseemds")
        .version("0.0.1")
        .about("Register you involuntary thoughts without exiting the terminal")
        .subcommands(vec![
            Command::new("add").subcommands(vec![Command::new("feeling"), Command::new("thought")]),
            Command::new("query"),
        ])
}
