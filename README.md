# serde-xml-rs

[![Build Status](https://travis-ci.org/RReverser/serde-xml-rs.svg?branch=master)](https://travis-ci.org/RReverser/serde-xml-rs)

`xml-rs` based serializer and deserializer for Serde (compatible with 1.0)

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
```

## Breaking changes in version 0.8.0

Notably:
- The `$value` name has been changed to `#content` (could become configurable in the future).
- Fields that are deserialized from attributes must now have a name that starts with a `@`. This aligns with what was introduced in the serializer.

See MIGRATION.md for more details, and tips on how to migrate.
