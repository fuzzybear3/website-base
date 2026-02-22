#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub href: &'static str,
    pub src: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub coming_soon: bool,
}

pub const PROJECTS: &[Project] = &[
    Project {
        title: "Travel Tracker",
        href: "https://travel.stevenguido.com/",
        src: "/static/travel.png",
        description: "Track and visualize trips and places visited around the world.",
        tags: &["software"],
        coming_soon: false,
    },
    Project {
        title: "New Project",
        href: "#",
        src: "",
        description: "Something new is in the works.",
        tags: &["hardware"],
        coming_soon: true,
    },
];
