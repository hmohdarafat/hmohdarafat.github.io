use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "kafka-notes-for-interviews",
    title: "Kafka Notes for Interviews",
    date: "2026-05-23",
    description: "A compact mental model for Kafka topics, partitions, and consumers.",
    tags: &["Kafka", "Distributed Systems"],
    body: r#"Kafka is a distributed event log. Producers write events. Consumers read events.

## Topic and partition

A topic is a named stream of events. A partition is an ordered append-only log inside that topic.

## Consumer groups

Consumers in the same group split partitions so work can be processed in parallel.

## Tradeoff

Kafka is powerful for durable event pipelines, but it adds operational complexity.
"#,
};
