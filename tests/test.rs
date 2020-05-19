#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;
extern crate simple_logger;

use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[test]
fn simple_struct_from_attributes() {
    let _ = simple_logger::init();

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
    let _ = simple_logger::init();

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
    let _ = simple_logger::init();

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

    #[serde(default)]
    sub_projects: Option<Vec<SubProject>>,
}

#[test]
fn nested_collection() {
    let _ = simple_logger::init();

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
            sub_projects: None,
        }
    );
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SubProject {
    name: String,

    #[serde(default)]
    items: Vec<Item>,
}

#[test]
fn nested_collection_with_nested_collection() {
    let _ = simple_logger::init();

    let s = r##"
        <project name="my_project">
          <!-- these items do not have a wrapping 'items' tag -->
          <item name="hello1" source="world1.rs" />
          <item name="hello2" source="world2.rs" />
          <sub_projects>
            <sub_project name="child1">
              <!-- these items are wrapped in a 'items' tag -->
              <items>
                <item name="foo1" source="bar1.rs" />
                <item name="foo2" source="bar2.rs" />
              </items>
            </sub_project>
          </sub_projects>
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
            sub_projects: Some(vec![
                SubProject {
                    name: "child1".to_string(),
                    items: vec![
                        Item {
                            name: "foo1".to_string(),
                            source: "bar1.rs".to_string(),
                        },
                        Item {
                            name: "foo2".to_string(),
                            source: "bar2.rs".to_string(),
                        },
                    ],
                },
            ]),
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
    let _ = simple_logger::init();

    let s = r##"
        <enums>
            <A>test</A>
            <B name="hello" flag="true" />
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

#[test]
fn nested_collection_with_nested_empty_collection() {
    let _ = simple_logger::init();

    let s = r##"
        <project name="my_project">
          <!-- these items do not have a wrapping 'items' tag -->
          <item name="hello1" source="world1.rs" />
          <item name="hello2" source="world2.rs" />
          <sub_projects>
            <sub_project name="child1">
              <!-- these items are wrapped in a 'items' tag -->
              <items></items>
            </sub_project>
          </sub_projects>
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
            sub_projects: Some(vec![
                SubProject {
                    name: "child1".to_string(),
                    items: vec![],
                },
            ]),
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Shelter {
    animals: Vec<Animal>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Animal {
    name: String,
    foo: String,
}

/// don't require a wrapper struct Vec<?>
#[test]
fn vec_container() {
    let _ = simple_logger::init();

    let s = r##"
    <shelter>
      <animals>
        <animal name="walter" foo="dog" />
        <animal name="max" foo="cat" />
      </animals>
    </shelter>
    "##;

    let shelter: Shelter = from_str(s).unwrap();

    assert_eq!(
        shelter.animals,
        vec![
            Animal {
                name: "walter".to_string(),
                foo: "dog".to_string(),
            },
            Animal {
                name: "max".to_string(),
                foo: "cat".to_string(),
            },
        ]
    );
}

#[test]
fn empty_vec_container() {
    let _ = simple_logger::init();

    let s = r##"
    <shelter>
      <animals>
      </animals>
    </shelter>
    "##;

    let shelter: Shelter = from_str(s).unwrap();
    assert_eq!(shelter.animals, Vec::new());

    let s = r##"
    <shelter>
      <animals/>
    </shelter>
    "##;

    let shelter: Shelter = from_str(s).unwrap();
    assert_eq!(shelter.animals, Vec::new());
}
