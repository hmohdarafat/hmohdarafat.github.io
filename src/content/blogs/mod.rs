pub mod build_this_site;
pub mod dioxus_file_structure;
pub mod github_pages_deployment;
pub mod rust_for_frontend;
pub mod system_design_learning;
pub mod lsm_tree_notes;
pub mod redis_cache_notes;
pub mod kafka_notes;
pub mod linux_workflow;
pub mod interview_prep_plan;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlogPostData {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub body: &'static str,
}

// Keep this array sorted newest-first. The blog index shows only the first 10 posts.
pub const POSTS: &[BlogPostData] = &[
    build_this_site::POST,
    dioxus_file_structure::POST,
    github_pages_deployment::POST,
    rust_for_frontend::POST,
    system_design_learning::POST,
    lsm_tree_notes::POST,
    redis_cache_notes::POST,
    kafka_notes::POST,
    linux_workflow::POST,
    interview_prep_plan::POST,
];

pub fn latest_posts() -> impl Iterator<Item = &'static BlogPostData> {
    POSTS.iter().take(10)
}

pub fn find_post(slug: &str) -> Option<&'static BlogPostData> {
    POSTS.iter().find(|post| post.slug == slug)
}
