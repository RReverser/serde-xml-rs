pub use rstest::{fixture, rstest};
use simple_logger::SimpleLogger;
use std::fmt::Debug;

#[fixture]
fn logger() {
    let _ = SimpleLogger::new().init();
}

mod de {
    use super::*;
    use serde::Deserialize;
    use serde_xml_rs::from_str;

    #[rstest]
    #[case::string("<bla>This is a String</bla>", "This is a String".to_string())]
    #[case::string("<bla></bla>", "".to_string())]
    #[case::string("<bla>    </bla>", "".to_string())]
    #[case::string("<bla>&lt;boom/&gt;</bla>", "<boom/>".to_string())]
    #[case::string("<bla>&#9835;</bla>", "♫".to_string())]
    #[case::string("<bla>&#x266B;</bla>", "♫".to_string())]
    #[case::string("<bla>♫<![CDATA[<cookies/>]]>♫</bla>", "♫<cookies/>♫".to_string())]
    #[case::i64("<bla>0</bla>", 0i64)]
    #[case::i64("<bla>-2</bla>", -2i64)]
    #[case::i64("<bla>-1234</bla>", -1234i64)]
    #[case::i64("<bla> -1234 </bla>", -1234i64)]
    #[case::u64("<bla>0</bla>", 0u64)]
    #[case::u64("<bla>1234</bla>", 1234u64)]
    #[case::u64("<bla> 1234 </bla>", 1234u64)]
    #[case::bool("<bla>true</bla>", true)]
    #[case::bool("<bla>false</bla>", false)]
    #[case::unit("<bla/>", ())]
    #[case::f64("<bla>3.0</bla>", 3.0f64)]
    #[case::f64("<bla>3.1</bla>", 3.1f64)]
    #[case::f64("<bla>-1.2</bla>", -1.2f64)]
    #[case::f64("<bla>0.4</bla>", 0.4f64)]
    #[case::f64("<bla>0.4e5</bla>", 0.4e5f64)]
    #[case::f64("<bla>0.4e15</bla>", 0.4e15f64)]
    #[case::f64_precision_troubles("<bla>0.4e-01</bla>", 0.4e-01f64)]
    #[case::f64("<bla> 0.4e-01 </bla>", 0.4e-01f64)]
    #[case::option("<bla/>", Some("".to_string()))]
    #[case::option("<bla></bla>", Some("".to_string()))]
    #[case::option("<bla> </bla>", Some("".to_string()))]
    #[case::option("<bla>42</bla>", Some("42".to_string()))]
    fn element_ok<T, 'de>(_logger: (), #[case] document: &str, #[case] expected: T)
    where
        T: Deserialize<'de> + Debug + PartialEq,
    {
        let actual: T = from_str(document).unwrap();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("<bla>verum</bla>", Some(true))]
    fn element_ko<T, 'de>(_logger: (), #[case] document: &str, #[case] _type: Option<T>)
    where
        T: Deserialize<'de> + Debug + PartialEq,
    {
        let actual: Result<T, _> = from_str(document);
        assert!(actual.is_err());
    }

    #[derive(PartialEq, Debug, Deserialize)]
    struct DummyAttribute<T> {
        foo: T,
    }

    #[rstest]
    #[case(r#"<bla foo="true"/>"#, DummyAttribute { foo: true })]
    #[case(r#"<bla foo="false"/>"#, DummyAttribute { foo: false })]
    #[case(r#"<bla foo="1"/>"#, DummyAttribute { foo: true })]
    #[case(r#"<bla foo="0"/>"#, DummyAttribute { foo: false })]
    fn attribute_ok<T, 'de>(_logger: (), #[case] document: &str, #[case] expected: T)
    where
        T: Deserialize<'de> + Debug + PartialEq,
    {
        let actual: T = from_str(document).unwrap();
        assert_eq!(actual, expected);
    }
}

mod ser {
    use super::*;
    use serde::{self, Serialize};
    use serde_xml_rs::to_string;

    #[derive(Serialize, Debug)]
    #[serde(rename = "bla")]
    struct Dummy<T> {
        #[serde(rename = "$value")]
        value: T,
    }

    #[rstest]
    #[case::string("<bla>This is a String</bla>", "This is a String".to_string())]
    #[case::string("<bla></bla>", "".to_string())]
    #[case::string("<bla>&lt;boom/></bla>", "<boom/>".to_string())]
    #[case::string("<bla>♫</bla>", "♫".to_string())]
    #[case::string("<bla>♫&lt;cookies/>♫</bla>", "♫<cookies/>♫".to_string())]
    #[case::i64("<bla>0</bla>", 0i64)]
    #[case::i64("<bla>-2</bla>", -2i64)]
    #[case::i64("<bla>-1234</bla>", -1234i64)]
    #[case::u64("<bla>0</bla>", 0u64)]
    #[case::u64("<bla>1234</bla>", 1234u64)]
    #[case::bool("<bla>true</bla>", true)]
    #[case::bool("<bla>false</bla>", false)]
    #[case::unit("<bla />", ())]
    #[case::f64("<bla>3</bla>", 3.0f64)]
    #[case::f64("<bla>3.1</bla>", 3.1f64)]
    #[case::f64("<bla>-1.2</bla>", -1.2f64)]
    #[case::f64("<bla>0.4</bla>", 0.4f64)]
    #[case::f64("<bla>40000</bla>", 0.4e5f64)]
    #[case::f64("<bla>400000000000000</bla>", 0.4e15f64)]
    #[case::f64_precision_troubles("<bla>0.04</bla>", 0.4e-01f64)]
    #[case::option("<bla></bla>", Some("".to_string()))]
    #[case::option("<bla>42</bla>", Some("42".to_string()))]
    fn element_ok<T>(_logger: (), #[case] expected: &str, #[case] value: T)
    where
        T: Serialize + Debug,
    {
        let actual = to_string(&Dummy { value }).unwrap();
        assert_eq!(
            actual,
            format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, expected)
        );
    }

    #[derive(Serialize, Debug)]
    #[serde(rename = "bla")]
    struct DummyAttribute<T> {
        #[serde(rename = "@value")]
        value: T,
    }

    #[rstest]
    #[case::string(r#"<bla value="" />"#, "".to_string())]
    #[case::bool(r#"<bla value="true" />"#, true)]
    #[case::bool(r#"<bla value="false" />"#, false)]
    #[case::option(r#"<bla value="apple" />"#, Some("apple".to_string()))]
    fn attribute_ok<T>(_logger: (), #[case] expected: &str, #[case] value: T)
    where
        T: Serialize + Debug,
    {
        let actual = to_string(&DummyAttribute { value }).unwrap();
        assert_eq!(
            actual,
            format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, expected)
        );
    }

    #[rstest]
    #[case::option(r#"<bla />"#, None)]
    fn attribute_none_ok(_logger: (), #[case] expected: &str, #[case] value: Option<String>) {
        let actual = to_string(&DummyAttribute { value }).unwrap();
        assert_eq!(
            actual,
            format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, expected)
        );
    }
}
