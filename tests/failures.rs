mod common;

use common::init_logger;
use log::info;
use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[test]
fn simple_struct_from_attributes_should_fail() {
    init_logger();

    let s = r##"
        <item name="hello" source="world.rs />
    "##;

    let item: Result<Item, _> = from_str(s);
    let err = item.unwrap_err();
    info!("simple_struct_from_attributes_should_fail(): {}", err);
}

#[test]
fn multiple_roots_attributes_should_fail() {
    init_logger();

    let s = r##"
        <item name="hello" source="world.rs" />
        <item name="hello source="world.rs" />
    "##;

    let item: Result<Vec<Item>, _> = from_str(s);
    let err = item.unwrap_err();
    info!("multiple_roots_attributes_should_fail(): {}", err);
}
