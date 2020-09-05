use clap::{Arg, App};

fn main() {
    let matches = App::new("Hours: Freelance billing app for Lex Office")
        .version("0.1.0")
        .author("Pyry Kovanen")
        .subcommands(vec![
            App::new("for")
                .about("selects project in hours")
                .arg(Arg::with_name("project_name")
                    .takes_value(true)
                    .about("Name of the project")
                    .required(true)
                ),
            App::new("worked-on")
                .about("saves worked hours for current day, or given day if date argument is given")
                .args(vec![
                    Arg::with_name("task")
                        .about("Task name")
                        .takes_value(true)
                        .required(true),
                    Arg::with_name("time")
                        .about("Time spent on task")
                        .takes_value(true)
                        .required(true),
                    Arg::new("date")
                        .short('d')
                        .about("Specify date instead of the default current date")
                        .required(false)
                ]),
            App::new("invoice")
                .about("Creates invoice for customer, takes in all the hours that have not been billed yet until current date (inclusive)")
                .args(vec![
                    Arg::with_name("project_name")
                        .takes_value(true)
                        .about("Name of the project")
                        .required(true)
                ])
        ])
        .get_matches();

    match matches.subcommand() {
        ("for", Some(register_project)) => {
            println!("Project: {}!", register_project.value_of("project_name").unwrap());
        }
        ("worked-on", Some(worked)) => {
            println!("Worked on {} for {} hours today!", worked.value_of("task").unwrap(), worked.value_of("time").unwrap());
        }
        ("invoice", Some(worked)) => {
            println!("Draft invoice created for {} !", worked.value_of("project name").unwrap());
        }
        _ => println!("No input"),
    };
}