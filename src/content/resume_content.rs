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
    headline: "Full-stack Developer / Software Engineer",
    summary: "Full-stack Developer and Software Engineer with 4 years of experience building practical web applications across Web3, SaaS, e-commerce, admin dashboards, customer dashboards, payment workflows, analytics, real-time features, and export-heavy business systems. Experienced in React, Node.js, Express, MongoDB, Redis, Socket.io, REST APIs, authentication, role-based access, smart contract integration, blockchain workflows, file/PDF/Excel exports, reporting features, and production-style debugging. Strong at building complete full-stack features from frontend UI to backend logic, database models, API integration, and deployment/debugging.",
    email: "hmohdarafat@gmail.com",
    github: "https://github.com/hmohdarafat",
    linkedin: "https://www.linkedin.com/in/mohdarafathossain/",
};

pub const SKILLS: &[&str] = &[
    "Full-stack Development",
    "Software Engineering",
    "Web Application Development",
    "React",
    "Next.js",
    "Redux Toolkit",
    "Node.js",
    "Express.js",
    "MongoDB",
    "PostgreSQL",
    "Redis",
    "BullMQ",
    "Socket.io",
    "REST APIs",
    "Authentication",
    "Role-based Access Control",
    "Admin Dashboards",
    "Customer Dashboards",
    "SaaS Applications",
    "E-commerce Systems",
    "Multi-vendor Systems",
    "Payment/Billing Workflows",
    "Reports & Analytics",
    "Real-time Features",
    "File Uploads",
    "PDF Export",
    "Excel Export",
    "CSV Export",
    "Blockchain Integration",
    "Smart Contracts",
    "Wallet Integration",
    "Token Transfers",
    "NFT Features",
    "Solidity",
    "Ethereum",
    "Polygon",
    "BNB Smart Chain",
    "Hardhat",
    "OpenZeppelin",
    "MetaMask",
    "Docker",
    "Linux",
    "Git",
    "Debugging",
    "System Design",
];

pub const EXPERIENCE: &[ExperienceItem] = &[
    ExperienceItem {
        role: "Full-stack Blockchain Developer / Software Engineer",
        company: "Bdtask Limited",
        period: "2022 – Present",
        details: "Build and maintain full-stack web applications across Web3, SaaS, e-commerce, dashboard, payment, analytics, and real-time business workflows. Work across frontend interfaces, backend APIs, database models, authentication, role-based access, reporting, file exports, smart contract integration, wallet flows, token transfers, and production-style debugging.",
    },
    ExperienceItem {
        role: "Video Game Developer + 3D Game Artist",
        company: "AAVA3D",
        period: "2021 – 2022",
        details: "Worked on a Unity3D desktop third-person action / role-playing / adventure game called 'Dungeon Darkest'.",
    },
];

pub const EDUCATION: &[EducationItem] = &[
    EducationItem {
        degree: "
        BSc in Computer Science and Engineering",
        school: "Canadian University of Bangladesh",
        period: "Bachelor's Degree",
        details: "Studied core computer science and software engineering topics, supporting professional work in full-stack web development, backend systems, databases, web application architecture, and practical software development.",
    },
];