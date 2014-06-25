#![crate_id="simplediff"]

use std::collections::HashMap;
use std::hash::Hash;

#[deriving(Show)]
pub enum Change<T> {
    Added(Vec<T>),
    Removed(Vec<T>),
    Same(Vec<T>)
}

pub fn diff<T: Eq + Hash + Clone>(before: &[T], after: &[T]) -> Vec<Change<T>> {
    let mut old_index_map = HashMap::<T, Vec<uint>>::new();

    for (i, element) in before.iter().enumerate() {
        old_index_map.insert_or_update_with(element.clone(), vec!(i), |_, v| v.push(i));
    }

    let mut overlap = HashMap::<uint, uint>::new();

    let mut sub_start_old = 0;
    let mut sub_start_new = 0;
    let mut sub_length = 0;

    for (i_new, val) in after.iter().enumerate() {
        let mut _overlap = HashMap::<uint, uint>::new();

        // let val = old_index_map.find_or_insert_with(element.clone(), |_| vec!());

        for &i_old in old_index_map.find(val).unwrap_or(&vec!()).iter() {
            _overlap.insert(i_old, overlap.find(&(i_old - 1)).unwrap_or(&0) + 1);

            if *_overlap.get(&i_old) > sub_length {
                sub_length = *_overlap.get(&i_old);
                sub_start_old = i_old - sub_length + 1;
                sub_start_new = i_new - sub_length + 1;
            }
        }

        overlap = _overlap;
    }

    if sub_length == 0 {
        let mut result = vec!();

        if !before.is_empty() { result.push(Removed(Vec::from_slice(before))); }
        if !after.is_empty() { result.push(Added(Vec::from_slice(after))); }

        return result;
    }

    let mut result = vec!();
    result.push_all_move(diff(before.slice_to(sub_start_old), after.slice_to(sub_start_new)));
    result.push(Same(Vec::from_slice(after.slice(sub_start_new, sub_start_new + sub_length))));
    result.push_all_move(diff(before.slice_from(sub_start_old + sub_length), after.slice_from(sub_start_new + sub_length)));

    result
}
