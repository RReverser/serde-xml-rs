#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;

#[derive(Debug, Deserialize)]
struct Item {
    pub name: String,
    pub source: String
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>
}

#[test]
fn it_works() {
    let s = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs" />
        </Project>
    "##;
    let project: Project = deserialize(s.as_bytes()).unwrap();
    println!("{:#?}", project);
}