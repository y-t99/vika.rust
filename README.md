### vika.rust

#### 👋 Introduce

🦀 rust for vikadata api. To use this crate, it's good for you to understand some vikadata's concept. for example, space, node, datasheet, view, field, record and soon. You can go [here](https://vika.cn/help/manual-1-what-is-vikadata/), it's vika office produce manual.

[vikadata](https://github.com/vikadata) is a Commercial version of Open Source [APITable](https://github.com/apitable) which added full enterprise-ready features. Vikadata is the on-line visual database. **It means that we can save structured data to vikadata by vikadata api , when we want to make some interesting and fun tools. Instead of using heavy database**

#### 🚀 Using

🌌 Firstly, adding `vika_community` crate to your dependence.

```toml
# Cargo.toml
[dependencies]
vika_community = { git = "https://github.com/y-t99/vika.rust.git" }
```
🌠 Importing the crate, and create the `vika_client` by your vikadata's token.

```rust
use vika_community::*;

let vika_client: VIKAClient= VIKAClient::new("your_token".to_string());
```

🪐 Query your all spaces:

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
let nodes: Vec<Node> = vika_client
        .spaces.space(space_id)
        .nodes.query_all()
        .unwrap();
// the space's all nodes
for node in nodes {
    println!(
        "the node info[id: {}, name: {}, type: {}, icon: {}, isFav: {}]. \nthe node's children is: {:?}",
        node.id, node.name, node.node_type, node.icon, node.is_fav, node.children
    );
}
// the specified node
let node: Node = vika_client
        .spaces.space(space_id)
        .nodes.node(node_id)
        .unwrap();
println!(
    "the node info: id: {}, name {}, type {}, icon {}, isFav {} \n. the node's children is: {:?}",
    node.id, node.name, node.node_type, node.icon, node.is_fav, node.children
)
```