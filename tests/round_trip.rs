#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use std::io::Cursor;
use serde_xml_rs::{serialize, deserialize};


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

    let item: Item = deserialize(Cursor::new(src)).unwrap();
    assert_eq!(item, should_be);

    let mut buffer = Vec::new();
    serialize(item, &mut buffer).unwrap();

    let reserialized_item = String::from_utf8(buffer).unwrap();
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

    // Create a buffer and serialize our nodes into it
    let mut buffer = Vec::new();
    serialize(&nodes, &mut buffer).unwrap();

    // We then check that the serialized string is the same as what we expect
    let serialized_nodes = String::from_utf8(buffer).unwrap();
    assert_eq!(serialized_nodes, should_be);

    // Then turn it back into a `Nodes` struct and make sure it's the same
    // as the original
    let deserialized_nodes: Nodes = deserialize(Cursor::new(serialized_nodes)).unwrap();
    assert_eq!(deserialized_nodes, nodes);
}
