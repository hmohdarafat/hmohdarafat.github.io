use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "redis-cache-notes",
    title: "Redis Cache Notes",
    date: "2026-05-24",
    description: "How I think about Redis as a cache in backend system design.",
    tags: &["Redis", "Caching"],
    body: r#"Redis is commonly used as an in-memory cache to reduce database load and lower read latency.

## Cache key design

A key should clearly identify what the value represents. Example: user:123:profile.

## Cache invalidation

The hard part is not putting data into Redis. The hard part is deciding when cached data is expired or invalid.

## Interview angle

Always mention TTL, cache misses, stale data, and fallback behavior.
"#,
};
