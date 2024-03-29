use anyhow::{bail, Result};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

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

impl Display for NotificationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ---
{}, interval: {} seconds",
            self.index, self.title, self.interval_secs
        )
    }
}
pub const NOTIFICATIONS_FILE_PATH: &'static str = "./notifications.memo";

pub fn parse_notifications(content: String) -> Result<Vec<NotificationData>> {
    // Sit Straight;10
    // Drink some water;25
    let rows: Vec<NotificationData> = content
        .trim()
        .split("\n")
        .enumerate()
        .map(|(index, line)| {
            let args: Vec<&str> = line.split(";").collect();

            let Some(title) = args.get(0) else {
                bail!("Memo file, line {index}: Tried to read interval")
            };

            let Some(interval_secs) = args.get(1) else {
                bail!("Memo file, line {index}: Tried to read interval")
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

pub fn append_notification(content: &mut String, title: &str, interval: &str) {
    content.push_str(&format!("{title};{interval}\n"));
}

mod tests {

    #[test]
    fn test_parse_notifications_valid_input() {
        let input = "Sit Straight;10\nDrink some water;25\n".to_string();
        let result = super::parse_notifications(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].index, 0);
        assert_eq!(result[0].title, "Sit Straight");
        assert_eq!(result[0].interval_secs, 10);
        assert_eq!(result[1].title, "Drink some water");
        assert_eq!(result[1].interval_secs, 25);
    }

    #[test]
    fn test_append_notification() {
        let mut content = "Sit Straight;10\nDrink some water;25\n".to_string();
        let title = "Foo";
        let interval = "300";
        super::append_notification(&mut content, title, interval);
        assert_eq!(content, "Sit Straight;10\nDrink some water;25\nFoo;300\n")
    }
}
