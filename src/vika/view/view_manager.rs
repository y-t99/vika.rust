use crate::FieldsManager;

use super::{Config, GetViewsResp, Response, View};
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;

pub struct ViewsManager {
    pub config: Config,

    pub space_id: String,

    pub datasheet_id: String,
}

impl ViewsManager {
    pub fn query_all(&self) -> Option<Vec<View>> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/datasheets/{}/views",
            &config.protocol, &config.host, &config.version, &self.datasheet_id
        );
        let client = Client::new();
        let body = client
            .get(url)
            .header(AUTHORIZATION, config.token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<GetViewsResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data.views)
        } else {
            None
        }
    }

    pub fn view(&self, view_id: String) -> ViewManager {
        ViewManager {
            config: self.config.clone(),
            space_id: self.space_id.clone(),
            datasheet_id: self.datasheet_id.clone(),
            view_id: view_id.clone(),
            fields: FieldsManager {
                config: self.config.clone(),
                space_id: self.space_id.clone(),
                datasheet_id: self.datasheet_id.clone(),
                view_id: view_id.clone(),
            },
        }
    }
}

pub struct ViewManager {
    pub config: Config,

    pub space_id: String,

    pub datasheet_id: String,

    pub view_id: String,

    pub fields: FieldsManager,
}
