pub mod vika;

pub use vika::*;

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::Value;

    #[test]
    fn test() {
        let vika_client: VIKAClient = VIKAClient::new("usk7MT3712irWT2nJeO21sN".to_string());
        let space_id = "spc73LrCV6vi4".to_string();
        let datasheet = vika_client.spaces.space(&space_id).datasheets;
        let field_post_req = PostFieldReq::builder()
            .name("name".to_string())
            .as_single_text_req()
            .default_value("default_value".to_string())
            .build();
        let datasheet_post_req: PostDatasheetReq = PostDatasheetReq::builder()
            .name("name".to_string())
            .description("description".to_string())
            .folder_id("fodWZeDELu12C".to_string())
            .pre_node_id("dst1E8tAt8JmXpqg2F".to_string())
            .fields(vec![field_post_req])
            .build();
        let new_datasheet: PostDatasheetsResp = datasheet.post(datasheet_post_req).unwrap();
    }
}
