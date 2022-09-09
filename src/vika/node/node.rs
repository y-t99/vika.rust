use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Node {
    pub id: String,

    pub name: String,

    #[serde(rename = "type")]
    pub node_type: String,

    pub icon: String,

    #[serde(rename = "isFav")]
    pub is_fav: bool,

    #[serde(default)]
    pub children: Vec<Node>,
}

#[derive(Deserialize, Serialize)]
pub struct GetNodesResp {
    pub nodes: Vec<Node>,
}
