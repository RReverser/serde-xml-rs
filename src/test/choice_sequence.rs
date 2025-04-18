use crate::{from_str, to_string};
use rstest::rstest;
use serde::{Deserialize, Serialize};

mod given_child_element_choice_sequence {
    use rstest::fixture;

    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document")]
    struct Document {
        #[serde(rename = "#content")]
        contents: Vec<Content>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    enum Content {
        Unit,
        Newtype(String),
        EmptyStruct {},
        Struct {
            a: String,
            b: i32,
        },
        StructWithAttribute {
            #[serde(rename = "@id")]
            id: i32,
        },
    }

    #[fixture]
    fn text() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8"?><document><unit /><newtype>abc</newtype><empty-struct /><struct><a>abc</a><b>123</b></struct><struct-with-attribute id="123" /></document>"#
    }

    #[fixture]
    fn value() -> Document {
        Document {
            contents: vec![
                Content::Unit,
                Content::Newtype("abc".to_string()),
                Content::EmptyStruct {},
                Content::Struct {
                    a: "abc".to_string(),
                    b: 123,
                },
                Content::StructWithAttribute { id: 123 },
            ],
        }
    }

    #[rstest]
    #[test_log::test]
    fn when_deserialize(text: &str, value: Document) {
        assert_eq!(from_str::<Document>(&text).unwrap(), value);
    }

    #[rstest]
    #[test_log::test]
    fn when_serialize(text: &str, value: Document) {
        assert_eq!(to_string(&value).unwrap(), text);
    }
}
