use crate::keyboard::KeysClick;
use serde::{Serialize, Serializer};

impl Serialize for KeysClick {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::keyboard::{KeyboardKey, KeyboardModifierKey, KeysClick};
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_keys_click_serialize() {
        let keys_click1 = KeysClick::new(KeyboardKey::L);

        let keys_click2 =
            KeysClick::new(KeyboardKey::H).add_modifier(KeyboardModifierKey::RightShift);

        let keys_click3 = KeysClick::new(KeyboardKey::H)
            .add_key(KeyboardKey::E)
            .add_key(KeyboardKey::L);

        assert_ser_tokens(&keys_click1, &[Token::String("l")]);
        assert_ser_tokens(&keys_click2, &[Token::String("right-shift+h")]);
        assert_ser_tokens(&keys_click3, &[Token::String("h+e+l")]);
    }
}
