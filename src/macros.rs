#[macro_export]
macro_rules! serialize_integers {
    ($($type_suffix:expr), *) => {
        paste! {
            $(
                #[inline]
                fn [<serialize_i $type_suffix>](self, value: [<i $type_suffix>]) -> Result<Self::Ok>
                {
                    let mut buffer = itoa::Buffer::new();
                    let s = buffer.format(value);
                    self.serialize_str(s)
                }

                #[inline]
                fn [<serialize_u $type_suffix>](self, value: [<u $type_suffix>]) -> Result<Self::Ok>
                {
                    let must_close_tag = self.build_start_tag()?;
                    let mut buffer = itoa::Buffer::new();
                    let s = buffer.format(value);
                    self.characters(s)?;
                    if must_close_tag {
                        self.end_tag()?;
                    }
                    Ok(())
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! deserialize_type_attr {
    ($($type:ty), *) => {
        paste! {
            $(
                fn [<deserialize_ $type>]<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                    visitor.[<visit_ $type>](self.0.parse()?)
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! deserialize_type {
    ($($type:ty), *) => {
        paste! {
            $(
                fn [<deserialize_ $type>]<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                    let value = self.prepare_parse_type::<V>()?.parse()?;
                    visitor.[<visit_ $type>](value)
                }
            )*
        }
    };
}