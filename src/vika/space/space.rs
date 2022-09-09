use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Space {
    pub id: String,

    pub name: String,

    #[serde(default, rename = "isAdmin")]
    pub is_admin: bool,
}

#[derive(Deserialize, Serialize)]
pub struct GetSpacesResp {
    pub spaces: Vec<Space>,
}
