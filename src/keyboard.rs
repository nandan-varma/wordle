use raylib::prelude::KeyboardKey;

// i know i know
pub fn map_key_to_char(key: KeyboardKey) -> Option<char> {
    match key {
        KeyboardKey::KEY_A => Some('A'),
        KeyboardKey::KEY_B => Some('B'),
        KeyboardKey::KEY_C => Some('C'),
        KeyboardKey::KEY_D => Some('D'),
        KeyboardKey::KEY_E => Some('E'),
        KeyboardKey::KEY_F => Some('F'),
        KeyboardKey::KEY_G => Some('G'),
        KeyboardKey::KEY_H => Some('H'),
        KeyboardKey::KEY_I => Some('I'),
        KeyboardKey::KEY_J => Some('J'),
        KeyboardKey::KEY_K => Some('K'),
        KeyboardKey::KEY_L => Some('L'),
        KeyboardKey::KEY_M => Some('M'),
        KeyboardKey::KEY_N => Some('N'),
        KeyboardKey::KEY_O => Some('O'),
        KeyboardKey::KEY_P => Some('P'),
        KeyboardKey::KEY_Q => Some('Q'),
        KeyboardKey::KEY_R => Some('R'),
        KeyboardKey::KEY_S => Some('S'),
        KeyboardKey::KEY_T => Some('T'),
        KeyboardKey::KEY_U => Some('U'),
        KeyboardKey::KEY_V => Some('V'),
        KeyboardKey::KEY_W => Some('W'),
        KeyboardKey::KEY_X => Some('X'),
        KeyboardKey::KEY_Y => Some('Y'),
        KeyboardKey::KEY_Z => Some('Z'),
        KeyboardKey::KEY_BACKSPACE => Some('/'),
        _ => None,
    }
}
