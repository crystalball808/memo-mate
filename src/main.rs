use std::{fs::read_to_string, process::exit};

use clap::Command;

use memo_mate::{
    notification::{parse_notifications, NOTIFICATIONS_FILE_PATH},
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
        .subcommand(Command::new("create").about("Create a new notification type"))
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
        Some(("list", _)) => println!("TODO"),
        Some(("create", _)) => println!("TODO"),
        Some(("delete", _)) => println!("TODO"),
        _ => {}
    }
}
