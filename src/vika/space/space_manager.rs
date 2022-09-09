use super::DatasheetsManager;
use super::GetSpacesResp;
use super::NodesManager;
use super::Response;
use super::{Config, Space};
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;

#[derive(Debug)]
pub struct SpacesManager {
    config: Config,
}

impl SpacesManager {
    pub fn new(config: Config) -> SpacesManager {
        SpacesManager { config }
    }

    pub fn query_all(&self) -> Option<Vec<Space>> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/spaces",
            &config.protocol, &config.host, &config.version
        );
        let client = Client::new();
        let body = client
            .get(url)
            .header(AUTHORIZATION, config.token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<GetSpacesResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data.spaces)
        } else {
            None
        }
    }

    pub fn space(&self, space_id: &String) -> SpaceManager {
        SpaceManager {
            _space_id: space_id.clone(),
            nodes: NodesManager {
                space_id: space_id.clone(),
                config: self.config.clone(),
            },
            datasheets: DatasheetsManager {
                config: self.config.clone(),
                space_id: space_id.clone(),
            },
        }
    }
}

pub struct SpaceManager {
    _space_id: String,

    pub nodes: NodesManager,

    pub datasheets: DatasheetsManager,
}
