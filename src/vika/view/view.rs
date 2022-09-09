use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct View {
    pub id: String,

    pub name: String,

    #[serde(rename = "type")]
    pub view_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetViewsResp {
    #[serde(default)]
    pub views: Vec<View>,
}
