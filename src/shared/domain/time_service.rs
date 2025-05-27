use chrono::Utc;

pub trait TimeService {
    fn get_current_time(&self) -> String;
}

pub struct TimeServiceImpl;

impl TimeService for TimeServiceImpl {
    fn get_current_time(&self) -> String {
        let now = Utc::now();
        now.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_system_time_service_impl_get_current_time() {
        let service = TimeServiceImpl;
        let current_time_str = service.get_current_time();

        let parsed_time = DateTime::parse_from_rfc3339(&current_time_str);

        assert!(!current_time_str.is_empty());
        assert!(
            parsed_time.is_ok(),
            "Failed to parse time string: {}",
            current_time_str
        );
    }
}
