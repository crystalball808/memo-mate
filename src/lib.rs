use std::{fs::File, thread};

use core::time;
use daemonize::Daemonize;
use notification::NotificationData;
use notify_rust::Notification;

pub mod notification;

const PID_PATH: &str = "/tmp/memo-mate.pid";

pub fn start_daemon(mut notifications: Vec<NotificationData>) {
    println!("Starting a daemong with notifications:");
    for notification in &notifications {
        println!("{notification}")
    }
    let stdout = File::create("/tmp/memo-mate.out").unwrap();
    let stderr = File::create("/tmp/memo-mate.err").unwrap();

    let daemon = Daemonize::new()
        .pid_file(PID_PATH)
        .stdout(stdout)
        .stderr(stderr)
        .umask(0o027);

    match daemon.start() {
        Ok(_) => println!("Daemon started successfully"),
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }

    loop {
        for notification_data in notifications.iter_mut() {
            if notification_data.is_ready() {
                Notification::new()
                    .summary(&notification_data.title)
                    .appname("MemoMate")
                    .show()
                    .unwrap();
                notification_data.reset();
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn stop_daemon() {
    // Read the PID from the pidfile
    let pid = std::fs::read_to_string(PID_PATH)
        .expect("Unable to read pid file")
        .trim()
        .parse::<u32>()
        .expect("Unable to parse pid");

    // Send a signal to terminate the daemon process
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(pid as i32),
        nix::sys::signal::Signal::SIGTERM,
    )
    .expect("Failed to kill the daemon");
}
