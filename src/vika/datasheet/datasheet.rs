use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use super::PostFieldReq;
use super::PostFieldResp;

pub struct PostDatasheetReq {
    field: Map<String, Value>,
}

pub struct PostDatasheetReqBuilder {
    field: Map<String, Value>,
}

impl PostDatasheetReq {
    pub fn builder() -> PostDatasheetReqBuilder {
        PostDatasheetReqBuilder { field: Map::new() }
    }

    pub fn to_json_string(self) -> String {
        serde_json::to_string(&self.field).unwrap()
    }
}

impl PostDatasheetReqBuilder {
    pub fn name(mut self, name: String) -> PostDatasheetReqBuilder {
        self.field.insert(String::from("name"), json!(name));
        self
    }

    pub fn description(mut self, description: String) -> PostDatasheetReqBuilder {
        self.field
            .insert(String::from("description"), json!(description));
        self
    }

    pub fn folder_id(mut self, folder_id: String) -> PostDatasheetReqBuilder {
        self.field
            .insert(String::from("folderId"), json!(folder_id));
        self
    }

    pub fn pre_node_id(mut self, pre_node_id: String) -> PostDatasheetReqBuilder {
        self.field
            .insert(String::from("preNodeId"), json!(pre_node_id));
        self
    }

    pub fn fields(mut self, fields: Vec<PostFieldReq>) -> PostDatasheetReqBuilder {
        let mut fields_list: Vec<Value> = Vec::new();
        for field in fields {
            let value = field.to_json_string();
            let struct_value = serde_json::from_str(&value).unwrap();
            fields_list.push(struct_value);
        }
        self.field
            .insert(String::from("fields"), json!(fields_list));
        self
    }

    pub fn build(self) -> PostDatasheetReq {
        PostDatasheetReq { field: self.field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_string() {
        let mut fields: Vec<PostFieldReq> = Vec::new();
        let field = PostFieldReq::builder()
            .name(String::from("name"))
            .as_single_text_req()
            .default_value(String::from("value"))
            .build();
        fields.push(field);
        let post_datasheet_req = PostDatasheetReq::builder()
            .name(String::from("name"))
            .fields(fields)
            .build();
        assert_eq!(
            post_datasheet_req.to_json_string().as_str(),
            r#"{"fields":[{"name":"name","property":{"default_value":"value"},"type":"SingleText"}],"name":"name"}"#
        )
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PostDatasheetsResp {
    #[serde(default)]
    pub id: String,

    #[serde(default, rename = "createAt")]
    pub create_at: i32,

    #[serde(default)]
    pub fields: Vec<PostFieldResp>,
}
