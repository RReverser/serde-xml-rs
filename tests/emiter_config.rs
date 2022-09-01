use serde::Serialize;
use serde_xml_rs::Serializer;
use xml::EmitterConfig;

#[derive(Debug, Serialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[test]
fn serializer_should_accept_custom_emitter() {
    let item = Item {
        name: "john".to_string(),
        source: "outerworld".to_string(),
    };
    let mut output = Vec::new();
    {
        let w = EmitterConfig::default()
            .perform_indent(true)
            .create_writer(&mut output);
        let mut serializer = Serializer::with_writer(w);
        item.serialize(&mut serializer).unwrap();
    }
    assert_eq!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Item>\n  <name>john</name>\n  <source>outerworld</source>\n</Item>", String::from_utf8_lossy(&output));
}
