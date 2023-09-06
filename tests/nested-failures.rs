use serde_xml_rs as xml;

#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Value {
    #[serde(rename = "i32")]
    I32(i32),
    #[serde(rename = "struct")]
    Struct(Struct),
    #[serde(rename = "array")]
    Array(Array),
}

#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Values {
    #[serde(rename = "value")]
    Value(Value),
}

#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Struct {
    #[serde(rename = "$value")]
    members: Vec<Members>,
}


#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Member {
    name: String,
    value: Value,
}


#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Members {
    #[serde(rename = "member")]
    Member(Member),
}

#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Structs {
    #[serde(rename = "struct")]
    Struct(Struct),
}


#[repr(C)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Array {
    data: Vec<Values>,
}

#[test]
fn nested_struct() -> Result<(), xml::Error> {
    let exp_xml = r#"<?xml version="1.0"?>
    <value>
      <struct>
        <member>
          <name>outerStruct</name>
          <value>
            <array>
              <data>
                <value>
                  <struct>
                    <member>
                      <name>innerStruct</name>
                      <value>
                        <array>
                          <data>
                            <value>
                              <i32>0</i32>
                            </value>
                            <value>
                              <i32>1</i32>
                            </value>
                          </data>
                        </array>
                      </value>
                    </member>
                  </struct>
                </value>
              </data>
            </array>
          </value>
        </member>
      </struct>
    </value>
    "#;

    xml::from_str::<Values>(exp_xml)?;

    Ok(())
}
