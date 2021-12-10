use crate::*;

impl serde::ser::Serialize for Language {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        s.serialize_str(self.to_639_3())
    }
}

struct LanguageVisitor;

impl<'a> serde::de::Visitor<'a> for LanguageVisitor {
    type Value = Language;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("borrowed str or bytes")
    }

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Language::from_639_3(v).or_else(|| Language::from_639_1(v)) {
            Some(l) => Ok(l),
            None => Err(serde::de::Error::unknown_variant(
                v,
                &["any valid ISO 639-1 or 639-3 code"],
            )),
        }
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match str::from_utf8(v) {
            Ok(s) => self.visit_borrowed_str(s),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(v),
                &self,
            )),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Language {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(LanguageVisitor)
    }
}
