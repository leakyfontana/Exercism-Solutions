// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::borrow::Borrow;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_snippets = HashMap::new();
    
    for word in magazine {
        *magazine_snippets.entry(word).or_insert(0) += 1;
    }

    for word in note {
        let entry = magazine_snippets.entry(word).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }

    true
}
