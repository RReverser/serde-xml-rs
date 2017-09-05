#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate serde;

use serde::ser::Serializer;
use serde_xml_rs::{from_str, to_string, wrap_primitives};


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

// Helper function for serializing Vec<String> as <identity>element<identity>
fn wrap_in_item<S>(item: &Vec<Node>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    wrap_primitives(item, serializer, "item")
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Nodes {
    #[serde(serialize_with = "wrap_in_item")]
    items: Vec<Node>,
}


#[test]
fn basic_struct() {
    let src = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
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

    let should_be = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
        <Nodes>\
            <items>\
                <item><Boolean>true</Boolean></item>\
                <item><Identifier>\
                    <value>foo</value>\
                    <index>5</index>\
                </Identifier></item>\
                <item>EOF</item>\
            </items>\
        </Nodes>";

    let serialized_nodes = to_string(&nodes).unwrap();
    assert_eq!(serialized_nodes, should_be);

    // Then turn it back into a `Nodes` struct and make sure it's the same
    // as the original
    let deserialized_nodes: Nodes = from_str(serialized_nodes.as_str()).unwrap();
    assert_eq!(deserialized_nodes, nodes);
}
