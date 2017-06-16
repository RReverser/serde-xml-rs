#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
    active: bool,
}

#[test]
fn simple_struct_from_attributes() {
    let s = r##"
        <item name="hello" source="world.rs" active="true" />
    "##;

    let item: Item = deserialize(s.as_bytes()).unwrap();

    assert_eq!(item, Item {
        name: "hello".to_string(),
        source: "world.rs".to_string(),
        active: true,
    });
}

#[test]
fn simple_struct_from_attribute_and_child() {
    let s = r##"
        <item name="hello">
            <source>world.rs</source>
            <active>false</active>
        </item>
    "##;

    let item: Item = deserialize(s.as_bytes()).unwrap();

    assert_eq!(item, Item {
        name: "hello".to_string(),
        source: "world.rs".to_string(),
        active: false,
    });
}

#[derive(Debug, Deserialize, PartialEq)]
struct Project {
    name: String,

    #[serde(rename = "item", default)]
    items: Vec<Item>
}

#[test]
fn nested_collection() {
    let s = r##"
        <project name="my_project">
            <item name="hello1" source="world1.rs" active="true" />
            <item name="hello2" source="world2.rs" active="false" />
        </project>
    "##;

    let project: Project = deserialize(s.as_bytes()).unwrap();

    assert_eq!(project, Project {
        name: "my_project".to_string(),
        items: vec![
            Item { name: "hello1".to_string(), source: "world1.rs".to_string(), active: true },
            Item { name: "hello2".to_string(), source: "world2.rs".to_string(), active: false }
        ]
    });
}

#[derive(Debug, Deserialize, PartialEq)]
enum MyEnum {
    A(String),
    B { name: String, flag: bool },
    C
}

#[derive(Debug, Deserialize, PartialEq)]
struct MyEnums {
    #[serde(rename = "$value")]
    items: Vec<MyEnum>
}

#[test]
fn collection_of_enums() {
    let s = r##"
        <enums>
            <A>test</A>
            <B name="hello" flag="true" />
            <C />
        </enums>
    "##;

    let project: MyEnums = deserialize(s.as_bytes()).unwrap();

    assert_eq!(project, MyEnums {
        items: vec![
            MyEnum::A("test".to_string()),
            MyEnum::B { name: "hello".to_string(), flag: true },
            MyEnum::C
        ]
    });
}
