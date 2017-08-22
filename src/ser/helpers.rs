use serde::ser::Serialize;
use ser::to_string;

/// In XML, various things can only be serialized if they are (or aren't)
/// primitive types. This is a **hack** to try and detect this.
pub fn is_primitive<S>(thing: &S) -> bool
where
    S: Serialize + ?Sized,
{
    let contains_xml_characters = match to_string(&thing) {
        Ok(repr) => repr.contains("<") && repr.contains(">") && repr.contains("</"),

        // if we can't serialize the caller will return an error anyway so
        // it doesn't matter what we return here.
        Err(_) => false,
    };

    !contains_xml_characters
}
