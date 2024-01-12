use std::{fs::File, thread};

use core::time;
use daemonize::Daemonize;
use notify_rust::Notification;

pub fn start_daemon() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemon = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every daemon needs a pidfile
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`
        .umask(0o027); // Set the umask for the process

    match daemon.start() {
        Ok(_) => println!("Daemon started successfully"),
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }

    loop {
        Notification::new()
            .summary("Test notificaton")
            .appname("MemoMate")
            .show()
            .unwrap();
        thread::sleep(time::Duration::from_secs(10));
    }
}

pub fn stop_daemon() {
    // Read the PID from the pidfile
    let pid = std::fs::read_to_string("/tmp/test.pid")
        .expect("Unable to read pid file")
        .trim()
        .parse::<u32>()
        .expect("Unable to parse pid");

    // Send a signal to terminate the daemon process
    // Depending on your system and requirements, you might need a different approach
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(pid as i32),
        nix::sys::signal::Signal::SIGTERM,
    )
    .expect("Failed to kill the daemon");
}
