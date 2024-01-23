use clap::Command;

use memo_mate::{notification::get_notifications, start_daemon, stop_daemon};

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
            let notifications = get_notifications();
            start_daemon(notifications);
        }
        Some(("stop", _)) => stop_daemon(),
        Some(("list", _)) => println!("TODO"),
        Some(("create", _)) => println!("TODO"),
        Some(("delete", _)) => println!("TODO"),
        _ => {}
    }
}
