use std::sync::{Arc, Mutex};

use crate::models::RecordItem;

#[derive(Clone)]
pub struct AppState {
    pub records: Arc<Mutex<Vec<RecordItem>>>,
}

impl AppState {
    pub fn new(records: Vec<RecordItem>) -> Self {
        Self {
            records: Arc::new(Mutex::new(records)),
        }
    }
}
