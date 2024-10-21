pub struct Endpoints {
    statuses: &'static str,
}

impl Endpoints {
    pub fn new() -> Self {
        Self {
            statuses: "/api/v1/statuses",
        }
    }

    pub fn get_statuses(&self) -> &str {
        self.statuses
    }
}
