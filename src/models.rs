#[derive(Clone, serde::Serialize)]
pub struct RecordItem {
    pub id: String,
    pub title: String,
    pub owner: String,
    pub status: String,
    pub priority: String,
    pub note: String,
}
