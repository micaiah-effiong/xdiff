pub fn wrap(size: usize, i: isize) -> usize {
    let k: usize;
    if i < 0 {
        k = (size as isize + i) as usize;
    } else {
        k = i as usize;
    }

    return k;
}
