pub struct Endpoints {
    search: &'static str,
    statuses: &'static str,
}

impl Endpoints {
    pub fn new() -> Self {
        Self {
            search: "/api/v1/accounts/verify_credentials",
            statuses: "/api/v1/statuses",
        }
    }

    pub fn get_search(&self) -> &str {
        self.search
    }
    
    pub fn get_statuses(&self) -> &str {
        self.statuses
    }
}
