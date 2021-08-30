pub fn unique(
    items: Vec<u16>
) -> bool {
    use std::iter::FromIterator;

    // no duplicate pairs
    std::collections::BTreeSet::from_iter(items.iter()).len() == items.len() 
}

pub fn correct_len(
    items: Vec<u16>,
    l: usize,
) -> bool {
    items.len() == l
}