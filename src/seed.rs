use crate::models::RecordItem;

pub fn default_records() -> Vec<RecordItem> {
    vec![
        RecordItem {
            id: "r1".into(),
            title: "Solar Site Planner 首轮安排".into(),
            owner: "dylan".into(),
            status: "进行中".into(),
            priority: "高".into(),
            note: "先处理核心事项和时间节点。".into(),
        },
        RecordItem {
            id: "r2".into(),
            title: "Solar Site Planner 协调会".into(),
            owner: "nina".into(),
            status: "待处理".into(),
            priority: "中".into(),
            note: "整理当前问题和后续动作。".into(),
        },
    ]
}
