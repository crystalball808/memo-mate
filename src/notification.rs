use anyhow::{bail, Result};
use std::fs::{read_to_string, File};
use std::io::{self, Write};
use std::time::{Duration, Instant};

pub struct NotificationData {
    index: usize,
    interval_secs: u64,
    pub title: String,
    instant: Instant,
}

impl NotificationData {
    pub fn is_ready(&self) -> bool {
        self.instant.elapsed().as_secs() >= self.interval_secs
    }
    pub fn reset(&mut self) {
        let reseted = self.instant + Duration::from_secs(self.interval_secs);
        self.instant = reseted;
    }
}
const NOTIFICATIONS_FILE_PATH: &'static str = "./notifications.memo";

pub fn get_notifications() -> Result<Vec<NotificationData>> {
    // Sit Straight;10
    // Drink some water;25
    let content = read_to_string(NOTIFICATIONS_FILE_PATH)?;
    let rows: Vec<NotificationData> = content
        .split("\n")
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let args: Vec<&str> = line.split(";").collect();

            let Some(title) = args.get(0) else {
                bail!("Memo file, line {line}: Tried to read interval")
            };

            let Some(interval_secs) = args.get(1) else {
                bail!("Memo file, line {line}: Tried to read interval")
            };
            let interval_secs: u64 = interval_secs.parse()?;

            Ok(NotificationData {
                index,
                interval_secs,
                title: title.to_string(),
                instant: Instant::now(),
            })
        })
        .collect::<Result<Vec<NotificationData>>>()?;

    Ok(rows)
}

fn write_notification_to_file() -> io::Result<()> {
    let mut file_handle = File::create("./notifications.memo")?;
    file_handle.write(b"Hello, world!!!")?;

    Ok(())
}
