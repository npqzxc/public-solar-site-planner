use askama::Template;
use axum::{extract::State, response::Html};

use crate::models::RecordItem;
use crate::store::AppState;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub title: &'static str,
    pub records: Vec<RecordItem>,
}

#[derive(Template)]
#[template(path = "records.html")]
pub struct RecordsTemplate {
    pub title: &'static str,
    pub records: Vec<RecordItem>,
}

#[derive(Template)]
#[template(path = "create.html")]
pub struct CreateTemplate {
    pub title: &'static str,
}

pub async fn dashboard_page(State(state): State<AppState>) -> Html<String> {
    let records = state.records.lock().unwrap().clone();
    Html(DashboardTemplate { title: "Solar Site Planner", records }.render().unwrap())
}

pub async fn records_page(State(state): State<AppState>) -> Html<String> {
    let records = state.records.lock().unwrap().clone();
    Html(RecordsTemplate { title: "Solar Site Planner", records }.render().unwrap())
}

pub async fn create_page() -> Html<String> {
    Html(CreateTemplate { title: "Solar Site Planner" }.render().unwrap())
}
