
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;

use serde_xml_rs::{from_str, to_string, wrap_primitives};

use serde::ser::Serializer;

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
    <list>
        <item name="hello" source="world.rs" />
        <item name="hello" source="world.rs" />
    </list>
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

    items: Vec<Item>,
}

#[test]
fn nested_collection() {
    init_logger();

    let s = r##"
        <project name="my_project">
            <items>
                <Item name="hello1" source="world1.rs" />
                <Item name="hello2" source="world2.rs" />
            </items>
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
    #[serde(rename = "$value")] items: Vec<MyEnum>,
}

#[test]
fn collection_of_enums() {
    init_logger();

    let s = r##"
        <enums>
            <items>
                <A>test</A>
                <B name="hello" flag="t" />
                <C />
            </items>
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


#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "wrapper")]
struct Wrapper {
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Type {
    Simple,
    Complex,
}

// Helper function for serializing Vec<String> as <identity>element<identity>
fn serialize_with_item_name<S>(item: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    wrap_primitives(item, serializer, "identity")
}


#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "group")]
pub struct Group {
    pub name: String,

    #[serde(rename = "type")] pub _type: Type,

    #[serde(serialize_with = "serialize_with_item_name")] pub members: MemberList,
    pub active: bool,
}

impl ::std::convert::From<MemberList> for Vec<String> {
    fn from(x: MemberList) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for MemberList {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::ser::Serialize for MemberList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_with_item_name(&self.0, serializer)
    }
}

#[serde(rename = "members")]
#[derive(Debug, Deserialize, PartialEq)]
pub struct MemberList(Vec<String>);

#[test]
fn deserialize_newtype_list() {
    let s = "\
             <?xml version=\"1.0\" encoding=\"UTF-8\"?>\
             <members>\
             <identity>bill</identity>\
             <identity>bob</identity>\
             <identity>dave</identity>\
             </members>\
             ";

    let members: MemberList = from_str(s).unwrap();
    let member_list = MemberList(vec![
        "bill".to_string(),
        "bob".to_string(),
        "dave".to_string(),
    ]);

    assert_eq!(members, member_list);
}

#[test]
fn deserialize_with_wrapped_list() {
    let s = r##"
        <wrapper>
          <groups>
            <group>
              <name>my group</name>
              <type>Simple</type>
              <members>
                <identity>bill</identity>
                <identity>bob</identity>
                <identity>dave</identity>
              </members>
              <active>true</active>
            </group>
          </groups>
        </wrapper>
    "##;

    let wrapper: Wrapper = from_str(s).unwrap();

    assert_eq!(
        wrapper,
        Wrapper {
            groups: vec![
                Group {
                    name: "my group".to_string(),
                    _type: Type::Simple,
                    members: MemberList(vec![
                        "bill".to_string(),
                        "bob".to_string(),
                        "dave".to_string(),
                    ]),
                    active: true,
                },
            ],
        }
    );
}

#[test]
fn serialize_with_wrapped_list() {
    let s = "\
             <?xml version=\"1.0\" encoding=\"UTF-8\"?>\
             <wrapper>\
             <groups>\
             <group>\
             <name>my group</name>\
             <type>Simple</type>\
             <members>\
             <identity>bill</identity>\
             <identity>bob</identity>\
             <identity>dave</identity>\
             </members>\
             <active>true</active>\
             </group>\
             </groups>\
             </wrapper>\
             ";

    let group = Wrapper {
        groups: vec![
            Group {
                name: "my group".to_string(),
                _type: Type::Simple,
                members: MemberList(vec![
                    "bill".to_string(),
                    "bob".to_string(),
                    "dave".to_string(),
                ]),
                active: true,
            },
        ],
    };

    assert_eq!(to_string(&group).unwrap(), s);
}

#[test]
fn serialize_with_empty_list() {
    let s = "\
             <?xml version=\"1.0\" encoding=\"UTF-8\"?>\
             <wrapper>\
             <groups>\
             <group>\
             <name>my group</name>\
             <type>Complex</type>\
             <members />\
             <active>true</active>\
             </group>\
             </groups>\
             </wrapper>\
             ";

    let group = Wrapper {
        groups: vec![
            Group {
                name: "my group".to_string(),
                _type: Type::Complex,
                members: MemberList(vec![]),
                active: true,
            },
        ],
    };

    assert_eq!(to_string(&group).unwrap(), s);
}

#[test]
fn deserialize_with_empty_list() {
    let s = r##"
        <wrapper>
          <groups>
            <group>
              <name>my group</name>
              <type>Complex</type>
              <members/>
              <active>true</active>
            </group>
          </groups>
        </wrapper>
    "##;

    let wrapper: Wrapper = from_str(s).unwrap();

    assert_eq!(
        wrapper,
        Wrapper {
            groups: vec![
                Group {
                    name: "my group".to_string(),
                    _type: Type::Complex,
                    members: MemberList(vec![]),
                    active: true,
                },
            ],
        }
    );
}

#[test]
fn deserialize_with_badly_formed_list() {
    let s = r##"
        <wrapper>
          <groups>
            <group>
              <name>my group</name>
              <members>THIS IS MALFORMED</members>
              <active>true</active>
            </group>
          </groups>
        </wrapper>
    "##;

    let wrapper: Result<Wrapper, _> = from_str(s);
    assert!(wrapper.is_err());
}
