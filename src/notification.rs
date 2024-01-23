use std::time::{Duration, Instant};

pub struct NotificationData {
    id: u32,
    interval_secs: u64,
    pub title: String,
    instant: Instant,
}
impl NotificationData {
    pub fn is_ready(&self) -> bool {
        return self.instant.elapsed().as_secs() >= self.interval_secs;
    }
    pub fn reset(&mut self) {
        let reseted = self.instant + Duration::from_secs(self.interval_secs);
        self.instant = reseted;
    }
}

pub fn get_notifications() -> Vec<NotificationData> {
    return vec![
        NotificationData {
            id: 1,
            interval_secs: 10,
            title: "Sit straight".to_owned(),
            instant: Instant::now(),
        },
        NotificationData {
            id: 2,
            interval_secs: 30,
            title: "Drink some water".to_owned(),
            instant: Instant::now(),
        },
    ];
}
