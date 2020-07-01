use crate::keyboard::KeysClick;
use serde::de::Visitor;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};
use std::convert::TryFrom;
use std::fmt;

struct KeysClickVisitor;

impl<'de> Visitor<'de> for KeysClickVisitor {
    type Value = KeysClick;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "an string of keyboard modifiers and keys separated with +"
        )
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match KeysClick::try_from(v) {
            Ok(k) => Ok(k),
            Err(_) => Err(Error::invalid_value(Unexpected::Str(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for KeysClick {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(KeysClickVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::keyboard::{KeyboardKey, KeyboardModifierKey, KeysClick};
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn test_keys_click_deserialize_ok() {
        let keys_click1 = KeysClick::new(KeyboardKey::L);

        let keys_click2 =
            KeysClick::new(KeyboardKey::H).add_modifier(KeyboardModifierKey::RightShift);

        let keys_click3 = KeysClick::new(KeyboardKey::H)
            .add_key(KeyboardKey::E)
            .add_key(KeyboardKey::L);

        assert_de_tokens(&keys_click1, &[Token::String("l")]);
        assert_de_tokens(&keys_click2, &[Token::String("right-shift+h")]);
        assert_de_tokens(&keys_click3, &[Token::String("h+e+l")]);
    }

    #[test]
    fn test_keys_click_deserialize_error() {
        assert_de_tokens_error::<KeysClick>(
            &[Token::Str("")],
            "invalid value: string \"\", expected an string of keyboard modifiers and keys separated with +" ,
        );
        assert_de_tokens_error::<KeysClick>(
            &[Token::Str("test")],
            "invalid value: string \"test\", expected an string of keyboard modifiers and keys separated with +" ,
        );
    }
}
