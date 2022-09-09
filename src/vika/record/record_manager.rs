use super::{
    Config, GetRecordsReq, GetRecordsResp, PatchRecordsReq, PatchRecordsResp, PostRecordsReq,
    PostRecordsResp, Response, BaseResponse
};
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

pub struct RecordsManager {
    pub config: Config,

    pub space_id: String,

    pub datasheet_id: String,
}

impl RecordsManager {
    pub fn query(&self, req: GetRecordsReq) -> Option<GetRecordsResp> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/datasheets/{}/records",
            &config.protocol, &config.host, &config.version, &self.datasheet_id
        );
        let client = Client::new();
        let parameters = &req.to_parameter_vec();
        let body = client
            .get(url)
            .query(parameters)
            .header(AUTHORIZATION, config.token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<GetRecordsResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }

    pub fn post(&self, records: PostRecordsReq) -> Option<PostRecordsResp> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/datasheets/{}/records",
            &config.protocol, &config.host, &config.version, &self.datasheet_id
        );
        let client = Client::new();
        let body = client
            .post(url)
            .header(AUTHORIZATION, config.token.clone())
            .header(CONTENT_TYPE, String::from("application/json"))
            .body(records.to_json_string())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<PostRecordsResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }

    pub fn patch(&self, records: PatchRecordsReq) -> Option<PatchRecordsResp> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/datasheets/{}/records",
            &config.protocol, &config.host, &config.version, &self.datasheet_id
        );
        let client = Client::new();
        let body = client
            .patch(url)
            .header(AUTHORIZATION, config.token.clone())
            .header(CONTENT_TYPE, String::from("application/json"))
            .body(records.to_json_string())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<PatchRecordsResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }

    pub fn delete(&self, record_ids: Vec<String>) -> Option<bool> {
        let mut values: Vec<(String, String)> = Vec::new();
        for record_id in record_ids {
            values.push(("recordIds".to_string(), record_id));
        }
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/datasheets/{}/records",
            &config.protocol, &config.host, &config.version, &self.datasheet_id
        );
        let client = Client::new();
        let resp_body = client
            .delete(url)
            .header(AUTHORIZATION, config.token.clone())
            .header(CONTENT_TYPE, String::from("application/json"))
            .query(&values)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: BaseResponse = serde_json::from_str(&resp_body).unwrap();
        if resp.code == 200 {
            Some(true)
        } else {
            None
        }
    }
}
