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
        title: "Magic Eye Games",
        href: "https://magic-eye.stevenguido.com",
        src: "/static/magic_eye.png",
        description: "A collection of games rendered as Magic Eye stereograms. Includes 2-player Pong and more.",
        tags: &["software"],
        coming_soon: false,
    },
    Project {
        title: "Numazu Tabemono",
        href: "https://tabemono.stevenguido.com",
        src: "",
        description: "A food and restaurant leaderboard for the town of Numazu.",
        tags: &["software"],
        coming_soon: false,
    },
];
