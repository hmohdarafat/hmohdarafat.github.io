pub struct ResumeInfo {
    pub name: &'static str,
    pub headline: &'static str,
    pub summary: &'static str,
    pub email: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
}

pub struct ExperienceItem {
    pub role: &'static str,
    pub company: &'static str,
    pub period: &'static str,
    pub details: &'static str,
}

pub struct EducationItem {
    pub degree: &'static str,
    pub school: &'static str,
    pub period: &'static str,
    pub details: &'static str,
}

pub const RESUME: ResumeInfo = ResumeInfo {
    name: "Mohd Arafat Hossain",
    headline: "Full-stack developer focused on Rust, web systems, and practical backend architecture.",
    summary: "I build web applications with attention to maintainable architecture, clear APIs, reliable state management, and operational simplicity. I am currently deepening my Rust, Dioxus, and system-design skills through project-based learning and technical writing.",
    email: "hmohdarafat@gmail.com",
    github: "https://github.com/hmohdarafat",
    linkedin: "https://www.linkedin.com/in/hmohdarafat/",
};

pub const SKILLS: &[&str] = &[
    "Rust",
    "Dioxus",
    "WebAssembly",
    "JavaScript",
    "React",
    "Node.js",
    "Express",
    "MongoDB",
    "Redis",
    "System Design",
    "Linux",
    "Git",
];

pub const EXPERIENCE: &[ExperienceItem] = &[
    ExperienceItem {
        role: "Full-stack Web Developer",
        company: "Project and product engineering",
        period: "Recent",
        details: "Built and maintained frontend and backend application flows, including authentication, APIs, data models, and user-facing admin workflows.",
    },
    ExperienceItem {
        role: "Rust/WebAssembly Learner",
        company: "Personal engineering practice",
        period: "Current",
        details: "Building personal projects with Rust and Dioxus to understand strongly typed UI development, WASM deployment, and static web publishing.",
    },
];

pub const EDUCATION: &[EducationItem] = &[
    EducationItem {
        degree: "Computer Science / Software Engineering Learning Path",
        school: "Project-based and interview-focused study",
        period: "Ongoing",
        details: "Focused on backend design, data structures, distributed systems, Rust, and practical full-stack development.",
    },
];
