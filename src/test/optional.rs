use crate::{from_str, to_string};
use rstest::rstest;
use serde::{Deserialize, Serialize};

mod given_struct_with_optional_field {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document")]
    struct Document {
        content: Option<String>,
    }

    #[rstest]
    #[case::some(r#"<content>abc</content>"#, Some("abc".to_string()))]
    #[case::some_empty(r#"<content></content>"#, Some("".to_string()))]
    #[case::some_empty(r#"<content />"#, Some("".to_string()))]
    #[case::none(r#""#, None)]
    #[test_log::test]
    fn when_deserialize(#[case] content_text: &str, #[case] content_value: Option<String>) {
        let text =
            format!(r#"<?xml version="1.0" encoding="UTF-8"?><document>{content_text}</document>"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(from_str::<Document>(&text).unwrap(), value);
    }

    #[rstest]
    #[case::some(r#"<document><content>abc</content></document>"#, Some("abc".to_string()))]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_serialize(#[case] content_text: &str, #[case] content_value: Option<String>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(to_string(&value).unwrap(), text);
    }
}

mod given_struct_with_optional_skipped_field {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document")]
    struct Document {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
    }

    #[rstest]
    #[case::some(r#"<document><content>abc</content></document>"#, Some("abc".to_string()))]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_serialize(#[case] content_text: &str, #[case] content_value: Option<String>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(to_string(&value).unwrap(), text);
    }
}

mod given_struct_with_optional_attribute {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document")]
    struct Document {
        #[serde(rename = "@content")]
        content: Option<String>,
    }

    #[rstest]
    #[case::some(r#"<document content="abc" />"#, Some("abc".to_string()))]
    #[case::none(r#"<document content="" />"#, None)]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_deserialize(#[case] content_text: &str, #[case] content_value: Option<String>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(from_str::<Document>(&text).unwrap(), value);
    }

    #[rstest]
    #[case::some(r#"<document content="abc" />"#, Some("abc".to_string()))]
    #[case::none(r#"<document content="" />"#, None)]
    #[test_log::test]
    fn when_serialize(#[case] content_text: &str, #[case] content_value: Option<String>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(to_string(&value).unwrap(), text);
    }
}

mod given_option_unit_field {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document")]
    struct Document {
        content: Option<()>,
    }

    #[rstest]
    #[case::some(r#"<document><content/></document>"#, Some(()))]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_deserialize(#[case] content_text: &str, #[case] content_value: Option<()>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(from_str::<Document>(&text).unwrap(), value);
    }

    #[rstest]
    #[case::some(r#"<document><content /></document>"#, Some(()))]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_serialize(#[case] content_text: &str, #[case] content_value: Option<()>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(to_string(&value).unwrap(), text);
    }
}

mod given_option_unit_struct_field {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "document")]
    struct Document {
        content: Option<Unit>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Unit;

    #[rstest]
    #[case::some(r#"<document><content/></document>"#, Some(Unit))]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_deserialize(#[case] content_text: &str, #[case] content_value: Option<Unit>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(from_str::<Document>(&text).unwrap(), value);
    }

    #[rstest]
    #[case::some(r#"<document><content /></document>"#, Some(Unit))]
    #[case::none(r#"<document />"#, None)]
    #[test_log::test]
    fn when_serialize(#[case] content_text: &str, #[case] content_value: Option<Unit>) {
        let text = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{content_text}"#);
        let value = Document {
            content: content_value,
        };
        assert_eq!(to_string(&value).unwrap(), text);
    }
}
