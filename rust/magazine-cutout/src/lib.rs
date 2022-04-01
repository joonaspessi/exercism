// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();
    for word in magazine {
        *magazine_words.entry(word).or_insert(0) += 1;
    }

    let mut note_words = HashMap::new();
    for word in note {
        *note_words.entry(word).or_insert(0) += 1;
    }

    let mut ret_val = true;
    for (word, note_count) in note_words.iter() {
        if let Some(mag_count) = magazine_words.get(word) {
            if note_count <= mag_count {
                continue;
            }
        }
        ret_val = false;
    }

    println!("{:?}", magazine_words);
    ret_val
}
