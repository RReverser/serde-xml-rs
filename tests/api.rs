pub use rstest::{fixture, rstest};
use simple_logger::SimpleLogger;

#[fixture]
fn logger() {
    let _ = SimpleLogger::new().init();
}

mod given_custom_event_writer {
    use super::*;
    use indoc::indoc;
    use serde::Serialize;
    use xml::EmitterConfig;

    #[derive(Debug, Serialize)]
    struct Document {
        content: Content,
    }

    #[derive(Debug, Serialize)]
    struct Content {
        text: String,
    }

    #[fixture]
    fn document() -> Document {
        Document {
            content: Content {
                text: "content text".into(),
            },
        }
    }

    #[rstest]
    fn should_accept_custom_event_writer(_logger: (), document: Document) {
        let mut output = Vec::new();
        let writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut output);
        let mut s = serde_xml_rs::ser::Serializer::new_from_writer(writer);

        document.serialize(&mut s).unwrap();
        let actual = String::from_utf8(output).unwrap();

        assert_eq!(
            actual,
            indoc!(
                r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <Document>
              <content>
                <text>content text</text>
              </content>
            </Document>"#
            )
        );
    }
}
