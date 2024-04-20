use std::{fs, io::Write, process::exit};

use clap::{Arg, Command};

use memo_mate::{
    notification::{
        append_notification, delete_notification, parse_notifications, stringify_notifications,
        NOTIFICATIONS_FILE_PATH,
    },
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
        .subcommand(
            Command::new("delete")
                .about("Delete a notification type")
                .arg(
                    Arg::new("id")
                        .required(true)
                        .help("The id of notification to delete"),
                ),
        )
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
                println!(
                    "{} ---
{}, interval: {} seconds",
                    notification.index, notification.title, notification.interval_secs
                )
            }
        }
        Some(("create", arg_matches)) => {
            let Some(title) = arg_matches.get_one::<String>("title") else {
                eprintln!("Please provide a title for new notification");
                exit(1);
            };
            let Some(interval) = arg_matches.get_one::<String>("interval") else {
                eprintln!("Please provide an interval for new notification");
                exit(1);
            };

            let mut content = fs::read_to_string(NOTIFICATIONS_FILE_PATH).unwrap_or(String::new());

            dbg!(&content);
            append_notification(&mut content, title, interval);
            dbg!(&content);

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
        Some(("delete", arg_matches)) => {
            let Some(index) = arg_matches.get_one::<String>("id") else {
                eprintln!("Please provide the id of the notification to delete");
                exit(1);
            };
            let index: usize = index.parse().expect("Id should be a number");

            let content = fs::read_to_string(NOTIFICATIONS_FILE_PATH).unwrap_or(String::new());
            let notifications = match parse_notifications(content) {
                Ok(notifications) => notifications,
                Err(error) => {
                    eprint!("Failed to get notifications: {error}");
                    exit(1);
                }
            };

            let notifications = delete_notification(notifications, index);

            let new_content = stringify_notifications(notifications);
            dbg!(&new_content);

            let mut file_handle = match fs::File::create(NOTIFICATIONS_FILE_PATH) {
                Ok(file_handle) => file_handle,
                Err(error) => {
                    eprint!("Failed to open the memo file to save: {error}");
                    exit(1);
                }
            };
            if let Err(error) = file_handle.write(new_content.as_bytes()) {
                eprint!("Failed to write the changes to the memo file: {error}");
                exit(1);
            } else {
                println!("Deleted the notification successfully")
            };
        }
        _ => {}
    }
}
