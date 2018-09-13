# serde-xml-rs

[![Build Status](https://travis-ci.org/RReverser/serde-xml-rs.svg?branch=master)](https://travis-ci.org/RReverser/serde-xml-rs)

xml-rs based deserializer for Serde (compatible with 0.9+)

## Usage

Use `serde_xml_rs::deserialize(...)` on any type that implements [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) as following:

```rust
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
```

Alternatively, you can use `serde_xml_rs::Deserializer` to create a deserializer from a preconfigured [`xml_rs::EventReader`](https://netvl.github.io/xml-rs/xml/reader/struct.EventReader.html).

## Parsing the "value" of a tag

If you have an input of the form `<foo abc="xyz">bar</foo>`, and you want to get at the`bar`, you can use the special name `$value`:

```rust
struct Foo {
    pub abc: String,
    #[serde(rename = "$value")]
    pub body: String,
}
```

## Parsed representations

Deserializer tries to be as intuitive as possible.

However, there are some edge cases where you might get unexpected errors, so it's best to check out [`tests`](tests/test.rs) for expectations.
