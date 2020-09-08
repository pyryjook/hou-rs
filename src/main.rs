mod services;

use clap::{Arg, App};

const FOR_COMMAND: &str = "for";
const WORKED_ON_COMMAND: &str = "worked-on";
const INVOICE_COMMAND: &str = "invoice";

const PROJECT_NAME_ARG: &str = "project_name";
const TASK_ARG: &str = "task";
const TIME_ARG: &str = "time";
const DATE_ARG: &str = "date";

fn main() {
    let matches = App::new("Hours: Freelance billing app for Lex Office")
        .version("0.1.0")
        .author("Pyry Kovanen")
        .subcommands(vec![
            App::new(FOR_COMMAND)
                .about("selects project in hours")
                .arg(Arg::with_name(PROJECT_NAME_ARG)
                    .takes_value(true)
                    .about("Name of the project")
                    .required(true)
                ),
            App::new(WORKED_ON_COMMAND)
                .about("saves worked hours for current day, or given day if date argument is given")
                .args(vec![
                    Arg::with_name(TASK_ARG)
                        .about("Task name")
                        .takes_value(true)
                        .required(true),
                    Arg::with_name(TIME_ARG)
                        .about("Time spent on task")
                        .takes_value(true)
                        .required(true),
                    Arg::new(DATE_ARG)
                        .short('d')
                        .about("Specify date instead of the default current date")
                        .required(false)
                ]),
            App::new(INVOICE_COMMAND)
                .about("Creates invoice for customer, takes in all the hours that have not been billed yet until current date (inclusive)")
                .args(vec![
                    Arg::with_name(PROJECT_NAME_ARG)
                        .takes_value(true)
                        .about("Name of the project")
                        .required(true)
                ])
        ])
        .get_matches();

    let result: String = match matches.subcommand() {
        (FOR_COMMAND, Some(register_project)) =>
            handle_for(
                register_project.value_of(PROJECT_NAME_ARG)
            ),
        (WORKED_ON_COMMAND, Some(worked)) =>
            handle_worked_on(
                worked.value_of(TASK_ARG),
                worked.value_of(TIME_ARG)
            ),
        (INVOICE_COMMAND, Some(worked)) =>
            handle_invoice(
                worked.value_of(PROJECT_NAME_ARG)
            ),
        _ =>
            handle_unknown()
    };


    println!("{}", result)
}


fn handle_for(project_name: Option<&str>) -> String {
    if let Some(name) = project_name {
        return format!("{}", name)
    }

    return String::new()
}

fn handle_worked_on(task: Option<&str>, time: Option<&str>) -> String {
    if let (Some(tsk), Some(tme)) = (task, time) {
        return format!("Marked {} to {}", tme, tsk)
    }

    return String::new()
}

fn handle_invoice(project_name: Option<&str>) -> String {
    if let Some(name) = project_name {
        return format!("{}", name)
    }

    return String::new()
}

fn handle_unknown() -> String {
    return String::new()
}
