use clap::{Arg, Command};
use std::{fs, io::Write, process::exit};

use memo_mate::{
    notification::{append_notification, parse_notifications, NOTIFICATIONS_FILE_PATH},
    start,
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
            let Ok(content) = fs::read_to_string(NOTIFICATIONS_FILE_PATH) else {
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
            start(notifications);
        }
        Some(("list", _)) => {
            let Ok(content) = fs::read_to_string(NOTIFICATIONS_FILE_PATH) else {
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

            let mut content = fs::read_to_string(NOTIFICATIONS_FILE_PATH).unwrap_or(String::new());

            append_notification(&mut content, title, interval);
            let mut file_handle = match fs::File::create(NOTIFICATIONS_FILE_PATH) {
                Ok(file_handle) => file_handle,
                Err(error) => {
                    eprint!("Failed to open the memo file to save: {error}");
                    exit(1);
                }
            };
            if let Err(error) = file_handle.write(content.as_bytes()) {
                eprint!("Failed to write the changes to the memo file: {error}");
                exit(1);
            };
        }
        Some(("delete", _)) => println!("TODO"),
        _ => {}
    }
}
