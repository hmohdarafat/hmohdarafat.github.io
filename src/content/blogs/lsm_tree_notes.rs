use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "lsm-tree-notes",
    title: "LSM Tree Notes",
    date: "2026-05-25",
    description: "Short notes on why LSM trees are useful in write-heavy databases.",
    tags: &["Databases", "LSM Tree"],
    body: r#"An LSM tree is a database storage design optimized for high write throughput.

## Core idea

Writes first go to memory and are later flushed to immutable sorted files on disk.

## Why it helps

Sequential disk writes are cheaper than many random disk writes. That is why LSM-based systems can be strong for write-heavy workloads.

## Tradeoff

Reads may need extra help from bloom filters, indexes, and compaction because data can exist across multiple files.
"#,
};
