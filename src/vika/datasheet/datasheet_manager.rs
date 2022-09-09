use crate::FieldsManager;

use super::{Config, PostDatasheetReq, PostDatasheetsResp, RecordsManager, Response, ViewsManager};
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
pub struct DatasheetsManager {
    pub config: Config,
    pub space_id: String,
}

impl DatasheetsManager {
    pub fn post(&self, datasheet: PostDatasheetReq) -> Option<PostDatasheetsResp> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/spaces/{}/datasheets",
            &config.protocol, &config.host, &config.version, &self.space_id
        );
        let client = Client::new();
        let body = client
            .post(url)
            .header(AUTHORIZATION, config.token.clone())
            .header(CONTENT_TYPE, String::from("application/json"))
            .body(datasheet.to_json_string())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<PostDatasheetsResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }

    pub fn datasheet(&self, datasheet_id: String) -> DatasheetManager {
        DatasheetManager {
            config: self.config.clone(),
            space_id: self.space_id.clone(),
            datasheet_id: datasheet_id.clone(),
            views: ViewsManager {
                config: self.config.clone(),
                space_id: self.space_id.clone(),
                datasheet_id: datasheet_id.clone(),
            },
            fields: FieldsManager::new(
                self.config.clone(),
                self.space_id.clone(),
                datasheet_id.clone(),
            ),
            records: RecordsManager {
                config: self.config.clone(),
                space_id: self.space_id.clone(),
                datasheet_id: datasheet_id.clone(),
            },
        }
    }
}

pub struct DatasheetManager {
    pub config: Config,
    pub space_id: String,
    pub datasheet_id: String,
    pub views: ViewsManager,
    pub fields: FieldsManager,
    pub records: RecordsManager,
}

impl DatasheetManager {}
