use crate::{from_str, to_string};
use rstest::rstest;
use serde::{Deserialize, Serialize};

mod given_empty_struct {
    use super::*;

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    struct Document {}

    #[rstest]
    #[case("<document><!-- comment --></document>")]
    #[case("<document></document><!-- comment -->")]
    #[case("<!-- comment --><document></document>")]
    #[test_log::test]
    fn when_deserialize(#[case] text: &str) {
        let value = Document {};

        assert_eq!(from_str::<Document>(text).unwrap(), value);
    }
}

mod given_struct_with_comment_field {
    use super::*;

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename = "document")]
    struct Document<T> {
        #[serde(rename = "#comment")]
        comment: T,
    }

    #[rstest]
    #[case("<document><!-- abc --></document>", Document { comment: "abc".to_string() })]
    #[test_log::test]
    fn when_deserialize<'de, T>(#[case] text: &str, #[case] value: Document<T>)
    where
        T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>,
    {
        assert_eq!(from_str::<Document<T>>(text).unwrap(), value);
    }

    #[rstest]
    #[case(r#"<?xml version="1.0" encoding="UTF-8"?><document><!-- abc --></document>"#, Document { comment: "abc".to_string() })]
    #[test_log::test]
    fn when_serialize<'de, T>(#[case] text: &str, #[case] value: Document<T>)
    where
        T: std::fmt::Debug + PartialEq + serde::Serialize,
    {
        assert_eq!(to_string(&value).unwrap(), text);
    }
}

mod given_enum_with_comment_variant {
    use super::*;

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename = "document")]
    struct Document {
        content: Content,
    }

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    enum Content {
        #[serde(rename = "#comment")]
        Comment(String),
    }

    #[rstest]
    #[case(r#"<?xml version="1.0" encoding="UTF-8"?><document><content><!-- abc --></content></document>"#, Document { content: Content::Comment("abc".to_string()) })]
    #[test_log::test]
    fn when_deserialize(#[case] text: &str, #[case] value: Document) {
        assert_eq!(from_str::<Document>(text).unwrap(), value);
    }

    #[rstest]
    #[case(r#"<?xml version="1.0" encoding="UTF-8"?><document><content><!-- abc --></content></document>"#, Document { content: Content::Comment("abc".to_string()) })]
    #[test_log::test]
    fn when_serialize(#[case] text: &str, #[case] value: Document) {
        assert_eq!(to_string(&value).unwrap(), text);
    }
}

mod given_enum_sequence_with_comment_variant {
    use super::*;

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    #[serde(rename = "document")]
    struct Document {
        #[serde(rename = "#content", default)]
        content: Vec<Content>,
    }

    #[derive(PartialEq, Serialize, Deserialize, Debug)]
    enum Content {
        #[serde(rename = "a")]
        A(String),
        #[serde(rename = "#comment")]
        Comment(String),
    }

    #[rstest]
    #[case(r#"<document><!-- abc --></document>"#, vec![Content::Comment("abc".to_string())])]
    #[case(r#"<document><a><!-- ignored -->123</a></document>"#, vec![Content::A("123".to_string())])]
    #[test_log::test]
    fn when_deserialize(#[case] content_text: &str, #[case] content_value: Vec<Content>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(from_str::<Document>(&text).unwrap(), value);
    }

    #[rstest]
    #[case(r#"<document><!-- abc --></document>"#, vec![Content::Comment("abc".to_string())])]
    #[case(r#"<document><a>123</a></document>"#, vec![Content::A("123".to_string())])]
    #[test_log::test]
    fn when_serialize(#[case] content_text: &str, #[case] content_value: Vec<Content>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(to_string(&value).unwrap(), text);
    }
}
