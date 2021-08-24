pub fn symmetrical(
    fwd: Vec<(u16, i16)>, 
    bwd: Vec<(i16, u16)>
) -> bool {
    // forward = {(a, b) for (b, a) in backward}
    let mut bwd_sort: Vec<(u16, i16)> = bwd.iter().map(|(x, y)| (*y, *x)).collect();
    bwd_sort.sort();
    fwd == bwd_sort
}

pub fn pair_unique(
    fwd: Vec<(u16, i16)>
) -> bool {
    use std::iter::FromIterator;

    // no duplicate pairs
    std::collections::BTreeSet::from_iter(fwd.iter()).len() == fwd.len() 
}

pub fn fwd_unique(
    fwd: Vec<(u16, i16)>, 
) -> bool {
    use std::iter::FromIterator;

    // no duplicates period
    // no duplicate u16s
    std::collections::BTreeSet::from_iter(fwd.iter().map(|(x, _)| x)).len() == fwd.len()
}

pub fn bwd_unique(
    bwd: Vec<(i16, u16)>, 
) -> bool {
    use std::iter::FromIterator;
    // no duplicate i16s
    std::collections::BTreeSet::from_iter(bwd.iter().map(|(x, _)| x)).len() == bwd.len()
}

pub fn fwd_equal(fwd: Vec<(u16, i16)>, mut_fwd: Vec<(u16, i16)>) -> bool {
    fwd == mut_fwd
}

pub fn fwd_correct_len(fwd: Vec<(u16, i16)>, l: usize) -> bool {
    fwd.len() == l
}

pub fn bwd_equal(bwd: Vec<(i16, u16)>, mut_bwd: Vec<(i16, u16)>) -> bool {
    bwd == mut_bwd
}

pub fn bwd_correct_len(bwd: Vec<(i16, u16)>, l: usize) -> bool {
    bwd.len() == l
}