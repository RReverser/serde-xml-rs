mod common;

use common::init_logger;
use serde::Deserialize;
use serde_xml_rs::{from_str, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "nesteddef")]
pub struct NestedDef {
    #[serde(rename = "$value")]
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Definition {
    #[serde(rename = "messageDefinition")]
    Message(MessageDefinition),

    #[serde(rename = "enumerationDefinition")]
    Enum(EnumerationDefinition),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MessageDefinition {
    pub name: String,

    #[serde(rename = "$value")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnumerationDefinition {
    pub name: String,

    #[serde(rename = "entry")]
    pub entries: Vec<EnumerationVariant>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnumerationVariant {
    pub value: u16,
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Field {
    #[serde(rename = "field")]
    Field { name: String },
}

// this order of XML is broken since 0.5, worked in 0.4.1
#[test]
fn vec_of_struct_first_parseable() {
    init_logger();

    let s = r##"
        <nesteddef>
            <enumerationDefinition name="some enum">
                <entry value="0" name="first" />
                <entry value="1" name="second" />
            </enumerationDefinition>
            <messageDefinition name="msg name">
                <field name="msg_field1" />
                <field name="msg_field2" />
                <field name="msg_field3" />
            </messageDefinition>
        </nesteddef>
    "##;

    let _ok: NestedDef = from_str(s).unwrap();
}

// this order of xml is parseable since 0.4.1 (potentially earlier did not try)
#[test]
fn vec_of_struct_second_parseable() {
    init_logger();

    let s = r##"
        <nesteddef>
            <messageDefinition name="msg name">
                <field name="msg_field1" />
                <field name="msg_field2" />
                <field name="msg_field3" />
            </messageDefinition>
            <enumerationDefinition name="some enum">
                <entry value="0" name="first" />
                <entry value="1" name="second" />
            </enumerationDefinition>
        </nesteddef>
    "##;

    let _ok: NestedDef = from_str(s).unwrap();
}
