#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;

use serde_xml_rs::from_str;

fn init_logger() {
    use log::{LogLevel, LogMetadata, LogRecord};

    struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &LogMetadata) -> bool {
            metadata.level() <= LogLevel::Debug
        }

        fn log(&self, record: &LogRecord) {
            if self.enabled(record.metadata()) {
                println!("{} - {}", record.level(), record.args());
            }
        }
    }

    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Debug);
        Box::new(SimpleLogger)
    });
}

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[test]
fn simple_struct_from_attributes() {
    init_logger();

    let s = r##"
        <item name="hello" source="world.rs" />
    "##;

    let item: Item = from_str(s).unwrap();

    assert_eq!(
        item,
        Item {
            name: "hello".to_string(),
            source: "world.rs".to_string(),
        }
    );
}

#[test]
fn multiple_roots_attributes() {
    init_logger();

    let s = r##"
        <item name="hello" source="world.rs" />
        <item name="hello" source="world.rs" />
    "##;

    let item: Vec<Item> = from_str(s).unwrap();

    assert_eq!(
        item,
        vec![
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            },
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            },
        ]
    );
}

#[test]
fn simple_struct_from_attribute_and_child() {
    init_logger();

    let s = r##"
        <item name="hello">
            <source>world.rs</source>
        </item>
    "##;

    let item: Item = from_str(s).unwrap();

    assert_eq!(
        item,
        Item {
            name: "hello".to_string(),
            source: "world.rs".to_string(),
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Project {
    name: String,

    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

#[test]
fn nested_collection() {
    init_logger();

    let s = r##"
        <project name="my_project">
            <item name="hello1" source="world1.rs" />
            <item name="hello2" source="world2.rs" />
        </project>
    "##;

    let project: Project = from_str(s).unwrap();

    assert_eq!(
        project,
        Project {
            name: "my_project".to_string(),
            items: vec![
                Item {
                    name: "hello1".to_string(),
                    source: "world1.rs".to_string(),
                },
                Item {
                    name: "hello2".to_string(),
                    source: "world2.rs".to_string(),
                },
            ],
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
enum MyEnum {
    A(String),
    B { name: String, flag: bool },
    C,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MyEnums {
    #[serde(rename = "$value")]
    items: Vec<MyEnum>,
}

#[test]
fn collection_of_enums() {
    init_logger();

    let s = r##"
        <enums>
            <A>test</A>
            <B name="hello" flag="t" />
            <C />
        </enums>
    "##;

    let project: MyEnums = from_str(s).unwrap();

    assert_eq!(
        project,
        MyEnums {
            items: vec![
                MyEnum::A("test".to_string()),
                MyEnum::B {
                    name: "hello".to_string(),
                    flag: true,
                },
                MyEnum::C,
            ],
        }
    );
}
