use askama::Template;
use axum::{extract::State, response::IntoResponse, Json};

use crate::store::AppState;

pub async fn dashboard(State(state): State<AppState>) -> impl IntoResponse {
    let records = state.records.lock().unwrap().clone();
    Json(serde_json::json!({
        "totals": [
            {"label": "总记录", "value": records.len()},
            {"label": "进行中", "value": records.len()}
        ],
        "records": records
    }))
}
