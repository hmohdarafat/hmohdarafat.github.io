pub struct ResumeInfo {
    pub name: &'static str,
    pub headline: &'static str,
    pub summary: &'static str,
    pub email: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
    pub location: &'static str,
    pub resume_pdf: &'static str,
}

pub struct SkillGroup {
    pub title: &'static str,
    pub skills: &'static [&'static str],
}

pub struct ExperienceItem {
    pub role: &'static str,
    pub company: &'static str,
    pub period: &'static str,
    pub details: &'static str,
    pub bullets: &'static [&'static str],
}

pub struct ProjectItem {
    pub title: &'static str,
    pub category: &'static str,
    pub details: &'static str,
    pub tech: &'static [&'static str],
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
    summary: "Full-stack Developer and Software Engineer with 4 years of experience building production web applications across Web3, SaaS, e-commerce, admin dashboards, customer dashboards, payment workflows, analytics, real-time features, and export-heavy business systems. Strong in building complete full-stack features from frontend UI to backend APIs, database models, authentication, role-based access, blockchain integration, reporting workflows, and deployment/debugging.",
    email: "hmohdarafat@gmail.com",
    github: "https://github.com/hmohdarafat",
    linkedin: "https://www.linkedin.com/in/mohdarafathossain/",
    location: "Dhaka, Bangladesh",
    resume_pdf: "/assets/Mohd_Arafat_Hossain_Resume.pdf",
};

pub const SKILL_GROUPS: &[SkillGroup] = &[
    SkillGroup {
        title: "Frontend",
        skills: &[
            "React",
            "Next.js",
            "Redux Toolkit",
            "JavaScript",
            "TypeScript",
            "HTML",
            "CSS",
            "Bootstrap",
            "Tailwind CSS",
            "Responsive UI",
            "Forms & Validation",
            "Admin Dashboards",
            "Customer Dashboards",
            "Charts & Reports",
            "API Integration",
        ],
    },
    SkillGroup {
        title: "Backend",
        skills: &[
            "Node.js",
            "Express.js",
            "REST APIs",
            "Authentication",
            "Role-based Access Control",
            "Payment/Billing Logic",
            "File Uploads",
            "PDF Export",
            "Excel Export",
            "CSV Export",
            "Background Jobs",
            "Production Debugging",
        ],
    },
    SkillGroup {
        title: "Database & Infrastructure",
        skills: &[
            "MongoDB",
            "PostgreSQL",
            "Redis",
            "BullMQ",
            "Socket.io",
            "Docker",
            "Linux",
            "Git",
            "Deployment Debugging",
            "System Design",
        ],
    },
    SkillGroup {
        title: "Web3 & Blockchain",
        skills: &[
            "Blockchain Integration",
            "Smart Contracts",
            "Wallet Integration",
            "Token Transfers",
            "NFT Minting",
            "NFT Marketplace Features",
            "Solidity",
            "Ethereum",
            "Polygon",
            "BNB Smart Chain",
            "Hardhat",
            "OpenZeppelin",
            "MetaMask",
            "Web3.js",
            "Ethers.js",
        ],
    },
];

pub const EXPERIENCE: &[ExperienceItem] = &[
    ExperienceItem {
        role: "Full-stack Blockchain Developer / Software Engineer",
        company: "Bdtask Limited",
        period: "2022 – Present",
        details: "Build and maintain full-stack web applications across Web3, SaaS, e-commerce, dashboard, payment, analytics, and real-time business workflows.",
        bullets: &[
            "Developed production web application features using React, Next.js, Node.js, Express.js, MongoDB, PostgreSQL, Redis, Socket.io, and REST APIs.",
            "Built frontend screens, dashboards, forms, filters, tables, charts, reports, and API-integrated user workflows.",
            "Implemented backend APIs, authentication, role-based access control, database models, payment/billing logic, file uploads, and export workflows.",
            "Worked on Web3 features including wallet connection, smart contract integration, token transfers, NFT flows, blockchain transactions, and Web3 payment-related features.",
            "Built and maintained SaaS, e-commerce, multi-vendor, admin dashboard, customer dashboard, analytics, reporting, and real-time application modules.",
            "Debugged frontend, backend, database, API, queue, real-time, and deployment-related issues in production-style applications.",
        ],
    },
    ExperienceItem {
        role: "Video Game Developer + 3D Artist",
        company: "Intellec IT Ltd.",
        period: "2021 – 2022",
        details: "Worked on various clients 3D assets & game features",
        bullets: &[
            "Created 3D assets and worked on gameplay and level design for various games.",
            "Worked with Unity3D, Unreal Engine, 3ds Max, zBrush, Substance Painter, Substance Designer",
        ],
    },
    ExperienceItem {
        role: "Video Game Developer + 3D Game Artist | Internship",
        company: "AAVA3D",
        period: "2020 – 2021",
        details: "Worked on a Unity3D desktop third-person action, role-playing, and adventure game called Dungeon Darkest.",
        bullets: &[
            "Worked on gameplay, level, and 3D asset-related tasks for a desktop game project.",
            "Used Unity3D and 3D game development workflows to contribute to interactive gameplay and visual systems.",
            "Gained early professional experience in software development, debugging, creative production, and project collaboration.",
        ],
    },
];

pub const PROJECTS: &[ProjectItem] = &[
    ProjectItem {
        title: "Web3 / NFT Marketplace Applications",
        category: "Blockchain / Web3",
        details: "Worked on Web3 application features including wallet connection, smart contract integration, NFT minting/listing/buying/selling flows, token transfers, blockchain transaction handling, and Web3 payment flows.",
        tech: &[
            "React",
            "Next.js",
            "Node.js",
            "Express.js",
            "Solidity",
            "Hardhat",
            "OpenZeppelin",
            "MetaMask",
            "Web3.js",
            "Ethers.js",
        ],
    },
    ProjectItem {
        title: "SaaS Admin & Customer Dashboard Systems",
        category: "SaaS / Business Applications",
        details: "Built dashboard-driven business applications with admin panels, customer dashboards, user management, role-based access, reporting, analytics, tables, filters, charts, and API-connected workflows.",
        tech: &[
            "React",
            "Redux Toolkit",
            "Node.js",
            "Express.js",
            "MongoDB",
            "PostgreSQL",
            "Redis",
            "REST APIs",
        ],
    },
    ProjectItem {
        title: "Multi-vendor E-commerce & Payment Workflows",
        category: "E-commerce / Payments",
        details: "Worked on multi-vendor and e-commerce-style systems with product/listing management, customer/vendor flows, order/transaction workflows, payment/billing logic, transaction history, invoices, and reporting.",
        tech: &[
            "React",
            "Next.js",
            "Node.js",
            "Express.js",
            "MongoDB",
            "PostgreSQL",
            "Payment APIs",
            "PDF Export",
        ],
    },
    ProjectItem {
        title: "Reports, Analytics & Export-heavy Systems",
        category: "Business Reporting",
        details: "Implemented reporting and export workflows for business applications, including downloadable reports, analytics views, PDF export, Excel export, CSV export, file uploads, and data-heavy admin interfaces.",
        tech: &[
            "React",
            "Node.js",
            "Express.js",
            "MongoDB",
            "PostgreSQL",
            "PDF Export",
            "Excel Export",
            "CSV Export",
        ],
    },
    ProjectItem {
        title: "Real-time Web Application Features",
        category: "Real-time Systems",
        details: "Built real-time application features using Socket.io for live updates, dashboard synchronization, event-driven UI updates, and backend-to-frontend communication.",
        tech: &[
            "React",
            "Node.js",
            "Express.js",
            "Socket.io",
            "Redis",
            "REST APIs",
        ],
    },
    ProjectItem {
        title: "Education / Exam Management Workflows",
        category: "Education Technology",
        details: "Worked on education and exam-related workflows involving admin interfaces, data management, reports, exports, user roles, and structured business processes.",
        tech: &[
            "React",
            "Redux Toolkit",
            "Node.js",
            "Express.js",
            "MongoDB",
            "Redis",
            "PDF Export",
            "Excel Export",
        ],
    },
    ProjectItem {
        title: "Dungeon Darkest",
        category: "Game Development",
        details: "Worked on a Unity3D desktop third-person action, role-playing, and adventure game during professional experience at AAVA3D.",
        tech: &[
            "Unity3D",
            "Game Development",
            "3D Game Art",
            "Desktop Game",
        ],
    },
];

pub const EDUCATION: &[EducationItem] = &[
    EducationItem {
        degree: "BSc in Computer Science and Engineering",
        school: "Canadian University of Bangladesh",
        period: "2016 – 2020",
        details: "Studied computer science and software engineering fundamentals, supporting professional work in full-stack web development, backend systems, databases, web application architecture, and practical software development.",
    },
];