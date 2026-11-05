use rand::seq::IndexedRandom;
use rdev::{Key, simulate};

pub trait WritingMode {
    fn on_press_key(&self, key: rdev::Key) -> bool {
        let _ = key;
        true
    }

    fn on_release_key(&self, key: rdev::Key) -> bool {
        let _ = key;
        true
    }
}

pub struct RandomInsertionMode {
    pub insertion_chance: u32,
    pub words: Vec<String>,
}

impl WritingMode for RandomInsertionMode {
    fn on_press_key(&self, key: rdev::Key) -> bool {
        if key != Key::Space {
            return true;
        }

        let is_on = rand::random_ratio(self.insertion_chance, 100);

        if !is_on {
            return true;
        }

        let empty = "".to_owned();
        let word = self.words.choose(&mut rand::rng()).unwrap_or(&empty);

        _ = simulate(&rdev::EventType::KeyPress(Key::Space));
        _ = simulate(&rdev::EventType::KeyRelease(Key::Space));

        for char in word.chars() {
            let keys = char_to_keys(char);
            for key in keys {
                _ = simulate(&rdev::EventType::KeyPress(key.to_owned()));
            }
            for key in keys {
                _ = simulate(&rdev::EventType::KeyRelease(key.to_owned()));
            }
        }
        true
    }
}

fn char_to_keys(character: char) -> &'static [Key] {
    match character {
        'q' => &[Key::KeyQ],
        'w' => &[Key::KeyW],
        'e' => &[Key::KeyE],
        'r' => &[Key::KeyR],
        't' => &[Key::KeyT],
        'y' => &[Key::KeyY],
        'u' => &[Key::KeyU],
        'i' => &[Key::KeyI],
        'o' => &[Key::KeyO],
        'p' => &[Key::KeyP],
        'a' => &[Key::KeyA],
        's' => &[Key::KeyS],
        'd' => &[Key::KeyD],
        'f' => &[Key::KeyF],
        'g' => &[Key::KeyG],
        'h' => &[Key::KeyH],
        'j' => &[Key::KeyJ],
        'k' => &[Key::KeyK],
        'l' => &[Key::KeyL],
        'z' => &[Key::KeyZ],
        'x' => &[Key::KeyX],
        'c' => &[Key::KeyC],
        'v' => &[Key::KeyV],
        'b' => &[Key::KeyB],
        'n' => &[Key::KeyN],
        'm' => &[Key::KeyM],
        ':' => &[Key::ShiftLeft, Key::SemiColon],
        '1' => &[Key::Num1],
        '2' => &[Key::Num2],
        '3' => &[Key::Num3],
        '4' => &[Key::Num4],
        '5' => &[Key::Num5],
        '6' => &[Key::Num6],
        '7' => &[Key::Num7],
        '8' => &[Key::Num8],
        '9' => &[Key::Num9],
        '0' => &[Key::Num0],
        _ => &[Key::Space],
    }
}
