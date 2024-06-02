use std::vec;

use bfs_diff::diff;

// use crate::greedy_diff::diff;

mod bfs_diff;
// mod greedy_diff;
mod utils;

fn main() {
    // let a: Vec<&str> = vec!["a", "b", "c", "a", "b", "b", "a"];
    // let b: Vec<&str> = vec!["c", "b", "a", "b", "a", "c"];

    let old_code = vec![
        "void Chunk_copy(Chunk *src, size_t src_start, Chunk *dst, size_t dst_start, size_t n)",
        "{",
        "    if (!Chunk_bounds_check(src, src_start, n)) return;",
        "    if (!Chunk_bounds_check(dst, dst_start, n)) return;",
        "",
        "    memcpy(dst->data + dst_start, src->data + src_start, n);",
        "}",
        "",
        "int Chunk_bounds_check(Chunk *chunk, size_t start, size_t n)",
        "{",
        "    if (chunk == NULL) return 0;",
        "",
        "    return start <= chunk->length && n <= chunk->length - start;",
        "}",
    ];
    let new_code = vec![
        "int Chunk_bounds_check(Chunk *chunk, size_t start, size_t n)",
        "{",
        "    if (chunk == NULL) return 0;",
        "",
        "    return start <= chunk->length && n <= chunk->length - start;",
        "}",
        "",
        "void Chunk_copy(Chunk *src, size_t src_start, Chunk *dst, size_t dst_start, size_t n)",
        "{",
        "    if (!Chunk_bounds_check(src, src_start, n)) return;",
        "    if (!Chunk_bounds_check(dst, dst_start, n)) return;",
        "",
        "    memcpy(dst->data + dst_start, src->data + src_start, n);",
        "}",
    ];

    assert_eq!(utils::utils::wrap(5, -3), 2);
    diff(&old_code, &new_code);
}
