use super::{Config, GetFieldsResp, Response, PostFieldReq, PostFieldResp};
use serde_json::Value;

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

pub struct FieldsManager {
    pub config: Config,

    pub space_id: String,

    pub datasheet_id: String,

    pub view_id: String,
}

impl FieldsManager {
    pub fn new(config: Config, space_id: String, datasheet_id: String) -> FieldsManager {
        FieldsManager {
            config,
            space_id,
            datasheet_id,
            view_id: String::from(""),
        }
    }

    pub fn query_all(&self) -> Option<Vec<Value>> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/datasheets/{}/fields",
            &config.protocol, &config.host, &config.version, &self.datasheet_id
        );
        let client = Client::new();
        let request_builder = client.get(url).header(AUTHORIZATION, config.token.clone());
        let body;
        if !self.view_id.eq("") {
            body = request_builder
                .query(&[("viewId", self.view_id.as_str())])
                .send()
                .unwrap()
                .text()
                .unwrap();
        } else {
            body = request_builder.send().unwrap().text().unwrap();
        }
        let resp: Response<GetFieldsResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data.fields)
        } else {
            None
        }
    }

    pub fn field_id(self, field_id: String) -> FieldManager {
        FieldManager {
            config: self.config.clone(),
            space_id: self.space_id.clone(),
            datasheet_id: self.datasheet_id.clone(),
            field_id,
        }
    }
}

pub struct FieldManager {
    config: Config,

    space_id: String,

    datasheet_id: String,

    field_id: String,
}

impl FieldManager {
    pub fn post(&self, field: PostFieldReq) -> Option<PostFieldResp>{
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/spaces/{}/datasheets/{}/fields",
            config.protocol, config.host, config.version, &self.space_id, &self.datasheet_id
        );
        let client = Client::new();
        let body = client
            .post(url)
            .header(AUTHORIZATION, config.token.clone())
            .header(CONTENT_TYPE, String::from("application/json"))
            .body(field.to_json_string())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<PostFieldResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }

    pub fn delete(&self) -> bool {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/spaces/{}/datasheets/{}/fields/{}",
            config.protocol,
            config.host,
            config.version,
            &self.space_id,
            &self.datasheet_id,
            &self.field_id
        );
        let client = Client::new();
        let body = client
            .delete(url)
            .header(AUTHORIZATION, config.token.clone())
            .header(CONTENT_TYPE, String::from("application/json"))
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<Value> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            true
        } else {
            false
        }
    }

}
