use super::Config;
use super::GetNodesResp;
use super::Node;
use super::Response;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
pub struct NodesManager {
    pub space_id: String,

    pub config: Config,
}

impl NodesManager {
    pub fn query_all(&self) -> Option<GetNodesResp> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/spaces/{}/nodes",
            &config.protocol, &config.host, &config.version, &self.space_id
        );
        let client = Client::new();
        let body = client
            .get(url)
            .header(AUTHORIZATION, config.token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<GetNodesResp> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }

    pub fn node(self, node_id: String) -> Option<Node> {
        let config: Config = self.config.clone();
        let url = format!(
            "{}://{}/fusion/{}/spaces/{}/nodes/{}",
            config.protocol, config.host, config.version, self.space_id, node_id
        );
        let client = Client::new();
        let body = client
            .get(url)
            .header(AUTHORIZATION, config.token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let resp: Response<Node> = serde_json::from_str(&body).unwrap();
        if resp.code == 200 {
            Some(resp.data)
        } else {
            None
        }
    }
}
