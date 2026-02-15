use chrono::{DateTime, Utc, Duration};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct NewsEvent {
    pub title: String,
    pub impact: ImpactLevel,
    pub currency: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

pub struct NewsCalendar {
    events: Vec<NewsEvent>,
    blackout_before: Duration,
    blackout_after: Duration,
}

impl NewsCalendar {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            blackout_before: Duration::hours(1),
            blackout_after: Duration::hours(2),
        }
    }

    pub async fn fetch_events(&mut self) -> Result<()> {
        // TODO: Integrate with economic calendar API
        // Example: ForexFactory, Investing.com API, etc.
        tracing::info!("News calendar fetching not yet implemented - using empty calendar");
        Ok(())
    }

    pub fn is_safe_to_trade(&self, now: DateTime<Utc>) -> bool {
        for event in &self.events {
            if event.impact != ImpactLevel::High {
                continue;
            }

            let blackout_start = event.time - self.blackout_before;
            let blackout_end = event.time + self.blackout_after;

            if now >= blackout_start && now <= blackout_end {
                tracing::warn!(
                    "Trading blocked by event: {} at {}",
                    event.title,
                    event.time
                );
                return false;
            }
        }

        true
    }

    pub fn get_upcoming_events(&self, hours: i64) -> Vec<&NewsEvent> {
        let now = Utc::now();
        let cutoff = now + Duration::hours(hours);

        self.events
            .iter()
            .filter(|e| e.time >= now && e.time <= cutoff)
            .collect()
    }
}

impl Default for NewsCalendar {
    fn default() -> Self {
        Self::new()
    }
}
