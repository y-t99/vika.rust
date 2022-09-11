### vika.rust

#### 👋 Introduce

🦀 rust for [vikadata api](https://vika.cn/developers/api/introduction). To use this crate, it's good for you to understand some vikadata's concept. for example, space, node, datasheet, view, field, record and soon. You can go [here](https://vika.cn/help/manual-1-what-is-vikadata/), it's vika office produce manual.

[Vikadata](https://github.com/vikadata) is a Commercial version of Open Source [APITable](https://github.com/apitable) which added full enterprise-ready features. Vikadata is the on-line visual database. **It means that we can save structured data to vikadata by vikadata api, when we want to make some interesting and fun tools. Instead of using heavy database.**

#### 🚀 Using

🌌 Firstly, adding `vika_community` crate to your dependence.

```toml
# Cargo.toml
[dependencies]
vika_community = "0.1.2"
# or
# vika_community = { git = "https://github.com/y-t99/vika.rust.git" }
serde_json = "1.0"
```

🌠 Importing the crate, and create the `vika_client` by your vikadata's token.

```rust
use vika_community::*;

let vika_client: VIKAClient= VIKAClient::new("your_token".to_string());
```

🪐 Querying your all spaces.

```rust
let spaces: Vec<Space> = vika_client.spaces.query_all().unwrap();
for space in spaces {
    println!(
        "the space's id is {} and name is {}. you {} are the space's admin", 
        space.id, space.name, space.is_admin
    );
}
```

☁️ You can get the space's all nodes, and query one node's detail information.

```rust
let space_id = "spcXxx".to_string();
let node_id = "nodXxx".to_string();
// the space's all nodes
let nodes: Vec<Node> = vika_client
        .spaces.space(&space_id)
        .nodes.query_all()
        .unwrap();
for node in nodes {
    println!(
        "the node info [id: {}, name: {}, type: {}, icon: {}, isFav: {}]. \nthe node's children is: {:?}",
        node.id, node.name, node.node_type, node.icon, node.is_fav, node.children
    );
}
// the specified node
let node: Node = vika_client
        .spaces.space(&space_id)
        .nodes.node(&node_id)
        .unwrap();
println!(
    "the node info: id: {}, name {}, type {}, icon {}, isFav {} \n. the node's children is: {:?}",
    node.id, node.name, node.node_type, node.icon, node.is_fav, node.children
)
```

🌈 The datasheet's field manager.

```rust
use serde_json::Value;

let space_id = "spcXxx".to_string();
let datasheet_id = "dstXxx".to_string();
let fields = vika_client
    .spaces.space(&space_id)
    .datasheets.datasheet(&datasheet_id)
    .fields;
// the new field info
let field_post_req = PostFieldReq::builder()
    .name("name".to_string())
    .as_single_text_req()
    .default_value("default_value".to_string())
    .build();
// the datasheet append the new field
let new_field: PostFieldResp = fields.post(field_post_req).unwrap();
// query the datasheet's all fields info
let fields_value: Vec<Value> = fields.query_all().unwrap();
for field_value in fields_value {
   for field_value in fields_value {
        println!(
            "the field info [id: {}, name: {}, type: {}] \n the field's property: [{}].",
            field_value["id"], field_value["name"], field_value["type"], field_value["property"]
        );
    }
}
// delete field by the field's id
fields.field_id(&new_field.id).delete();
```

☄️ We can crete the datasheet.

```rust
let space_id = "spaceXxx".to_string();
let datasheets = vika_client.spaces.space(&space_id).datasheets;
let field_post_req = PostFieldReq::builder()
    .name("field_name".to_string())
    .as_single_text_req()
    .default_value("default_value".to_string())
    .build();
let datasheet_post_req: PostDatasheetReq = PostDatasheetReq::builder()
    .name("datasheet_name".to_string())
    .description("description".to_string())
    .folder_id("fodXxx".to_string())
    .pre_node_id("dstXxx".to_string())
    .fields(vec![field_post_req])
    .build();
let new_datasheet: PostDatasheetsResp = datasheets.post(datasheet_post_req).unwrap();
```

✨ Query the datasheet's record.

```rust
let space_id = "spcXxx".to_string();
let datasheet_id= "dstXxx".to_string();
let records = vika_client
    .spaces.space(&space_id)
    .datasheets.datasheet(&datasheet_id)
    .records;
let records_req = GetRecordsReq::builder()
    .page_size(1)
    .max_records(1)
    .page_num(1)
    .field_key(FieldID::ID)
    .sort(vec![
        ("fldXxx".to_string(), Sort::DESC)
    ])
    .filter_by_formula("formula".to_string())
    .build();
let records_resp: GetRecordsResp = records.query(records_req).unwrap();
println!(
    "the records' page [current page number.: {}, the page's record size: {}, total number of records that met filter: {}]. \n records: [{:?}]",
    records_resp.page_num, records_resp.page_size, records_resp.total, records_resp.records
)
```

🛸 Managing the records.

```rust
// add records
let filed_map = vec![
    "fldXxx".to_string(),
    "fldYyy".to_string(),
    "fldZzz".to_string(),
];
let record = RecordMap::builder()
    .put_string(&filed_map[0], "value".to_string())
    .put_strings(&filed_map[1], vec!["value".to_string()])
    .build();
let post_records_req = PostRecordsReq::builder()
    .field_key(FieldID::ID)
    .records(vec![record])
    .build();
let post_records_resp: PostRecordsResp = records.post(post_records_req).unwrap();
let record_id = post_records_resp.records[0].record_id.clone();
// update records
let record: RecordMap = RecordMap::builder()
    .record_id(record_id.clone())
    .put_string(&filed_map[2], "value".to_string())
    .build();
let patch_records_req: PatchRecordsReq = PatchRecordsReq::builder()
    .field_key(FieldID::ID)
    .records(vec![record])
    .build();
// delete records
records.delete(vec![record_id.clone()]);
```

#### 🚚 Waiting & Expecting

🛠️. more manager: view manager

👣. better error reporting

💪. more robust low-level code

🛹. easier API

🦀. more rust