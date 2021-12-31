use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();

    for word in magazine {
        let entry = magazine_words.entry(word).or_insert(0);
        *entry += 1;
    }

    for word in note {
        match magazine_words.get_mut(word) {
            None | Some(0) => {
                return false;
            }
            Some(v) => {
                *v -= 1;
            }
        }
    }
    true
}
