#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use serde_xml_rs::{from_str, to_string};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Node {
    Boolean(bool),
    Identifier { value: String, index: u32 },
    EOF,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Nodes {
    #[serde(rename = "$value")]
    items: Vec<Node>,
}


#[test]
fn basic_struct() {
    let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
    let should_be = Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    };

    let item: Item = from_str(src).unwrap();
    assert_eq!(item, should_be);

    let reserialized_item = to_string(&item).unwrap();
    assert_eq!(src, reserialized_item);
}

#[test]
fn repeated_basic_struct() {
    let src = r#"<Item><name>Banana</name><source>Store</source></Item><Item><name>Split</name><source>Store</source></Item>"#;
    let should_be = vec![Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    }, Item {
        name: "Split".to_string(),
        source: "Store".to_string(),
    }];

    let items: Vec<Item> = from_str(src).unwrap();
    assert_eq!(items, should_be);

    let reserialized_item = to_string(&items).unwrap();
    assert_eq!(src, reserialized_item);
}


#[test]
#[ignore]
fn round_trip_list_of_enums() {
    // Construct some inputs
    let nodes = Nodes {
        items: vec![
            Node::Boolean(true),
            Node::Identifier {
                value: "foo".to_string(),
                index: 5,
            },
            Node::EOF,
        ],
    };

    let should_be = r#"
    <Nodes>
        <Boolean>
            true
        </Boolean>
        <Identifier>
            <value>foo</value>
            <index>5</index>
        </Identifier>
        <EOF />
    </Nodes>"#;

    let serialized_nodes = to_string(&nodes).unwrap();
    assert_eq!(serialized_nodes, should_be);

    // Then turn it back into a `Nodes` struct and make sure it's the same
    // as the original
    let deserialized_nodes: Nodes = from_str(serialized_nodes.as_str()).unwrap();
    assert_eq!(deserialized_nodes, nodes);
}
