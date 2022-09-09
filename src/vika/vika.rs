use super::Config;
use super::SpacesManager;

/** `VIKAClient` vika‘s client
 *
 *
 */
pub struct VIKAClient {
    _config: Config,

    pub spaces: SpacesManager,
}

impl VIKAClient {
    pub fn new(token: String) -> VIKAClient {
        let config = Config {
            protocol: String::from("https"),
            host: String::from("api.vika.cn"),
            version: String::from("v1"),
            token: format!("Bearer {}", token),
        };
        let copy = config.clone();
        VIKAClient {
            _config: config,

            spaces: SpacesManager::new(copy),
        }
    }
}
