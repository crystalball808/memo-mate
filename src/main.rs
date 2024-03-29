use std::{fs::read_to_string, process::exit};

use clap::{Arg, Command};

use memo_mate::{
    notification::{append_notification, parse_notifications, NOTIFICATIONS_FILE_PATH},
    start_daemon, stop_daemon,
};

fn cli() -> Command {
    Command::new("memo-mate")
        .about("A lightweight notification CLI tool")
        .subcommand_required(true)
        .subcommand(Command::new("start").about("Start sending notifications"))
        .subcommand(Command::new("stop").about("Stop sending notifications"))
        .subcommand(
            Command::new("list")
                .visible_alias("ls")
                .about("List all types of notifications"),
        )
        .subcommand(
            Command::new("create")
                .about("Create a new notification type")
                .args([
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .required(true)
                        .help("The message that would be shown"),
                    Arg::new("interval")
                        .short('i')
                        .long("interval")
                        .required(true)
                        .help("Frequensy of notification in seconds"),
                ]),
        )
        .subcommand(Command::new("delete").about("Delete a notification type"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("start", _)) => {
            let Ok(content) = read_to_string(NOTIFICATIONS_FILE_PATH) else {
                println!("To start, create a first notification");
                exit(0);
            };
            let notifications = match parse_notifications(content) {
                Ok(notifications) => notifications,
                Err(error) => {
                    eprint!("Failed to get notifications: {error}");
                    exit(1);
                }
            };
            start_daemon(notifications);
        }
        Some(("stop", _)) => stop_daemon(),
        Some(("list", _)) => {
            let Ok(content) = read_to_string(NOTIFICATIONS_FILE_PATH) else {
                println!("To start, create a first notification");
                exit(0);
            };
            let notifications = match parse_notifications(content) {
                Ok(notifications) => notifications,
                Err(error) => {
                    eprint!("Failed to get notifications: {error}");
                    exit(1);
                }
            };

            for notification in notifications {
                println!("{}", notification)
            }
        }
        Some(("create", arg_matches)) => {
            let Some(title) = arg_matches.get_one::<String>("title") else {
                eprint!("Please provide a title for new notification");
                exit(1);
            };
            let Some(interval) = arg_matches.get_one::<String>("interval") else {
                eprint!("Please provide an interval for new notification");
                exit(1);
            };

            let mut content = read_to_string(NOTIFICATIONS_FILE_PATH).unwrap_or(String::new());

            append_notification(&mut content, title, interval);

            unimplemented!()
        }
        Some(("delete", _)) => println!("TODO"),
        _ => {}
    }
}
