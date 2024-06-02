use crate::greedy_diff::diff;

mod greedy_diff;
mod utils;

fn main() {
    let a: Vec<&str> = vec!["a", "b", "c", "a", "b", "b", "a"];
    let b: Vec<&str> = vec!["c", "b", "a", "b", "a", "c"];

    assert_eq!(utils::utils::wrap(5, -3), 2);
    diff(&a, &b)
}
