# serde-xml-rs

[![Build Status](https://travis-ci.org/RReverser/serde-xml-rs.svg?branch=master)](https://travis-ci.org/RReverser/serde-xml-rs)

`xml-rs` based deserializer for Serde (compatible with 1.0)

## Example usage

```rust
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

fn main() {
    let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
    let should_be = Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    };

    let item: Item = from_str(src).unwrap();
    assert_eq!(item, should_be);

    let reserialized_item = to_string(&item).unwrap();
    assert_eq!(src, reserialized_item);
}
```

## Tuning the serialization

Since XML has multiple ways to represent "sub-values" of a node (attributes,
child nodes, contents) the serializer understands various special names for
fields to steer where and how their contents should be rendered.

```rust
#[derive(Serialize)]
struct Node {
  // Fields with names starting with `@` are rendered as attributes (and must
  therefore be primitive types)
  #[serde(rename = "@my_attr")]
  my_attr: String,

  // Fields renamed to "$value" are rendered as content nodes of the structure
  (and must also be primitive types)
  #[serde(rename = "$value")]
  content: String,

  // Other fields are rendered as child nodes
  child: String,
}
```

The above might get rendered as:

```xml
<Node my_attr="foo">
    Hello World!
    <child>And me too!</child>
</Node>
```

Unit-variant `enum` objects can also be rendered in two ways, either as a
self-closing node or as a string content by default they're rendered as nodes,
but this can also be tuned:

```rust
enum Options {
    Live,
    #[serde(rename = "@Laugh")]
    Laugh,
    #[serde(rename = "@LOVE")]
    Love,
}
```

When an instance of this `enum` is serialized, `Options::Live` renders as
`<Live />` while `Laugh` renders as `Laugh` and `Love` renders as `LOVE`.
