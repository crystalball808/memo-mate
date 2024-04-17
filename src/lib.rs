use std::thread;

use core::time;
use notification::NotificationData;
use notify_rust::Notification;

pub mod notification;

pub fn start(mut notifications: Vec<NotificationData>) {
    println!("Starting a daemong with notifications:");
    for notification in &notifications {
        println!("{notification}")
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
