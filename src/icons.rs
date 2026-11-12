use rune::Icon;

// Re-export common icons from opencode & rune-kit
pub use rune_theme_opencode::icons::{
    ARROW_LEFT, ARROW_RIGHT, BRAIN, CHECKLIST, CHEVRON_LEFT, CIRCLE_CHECK, CIRCLE_X,
    EDIT, FLASK, GRADUATION_CAP, HELP, REFRESH,
};

pub use rune_kit::glyphs::CHECK;
pub use rune_kit::shell::SIDEBAR;

/// Modern sun icon for light theme toggle
pub static SUN: Icon = Icon {
    name: "app-sun",
    view: 16.0,
    body: r#"<circle cx="8" cy="8" r="3.2" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M8 1.5v1.5M8 13v1.5M1.5 8h1.5M13 8h1.5M3.4 3.4l1.1 1.1M11.5 11.5l1.1 1.1M3.4 12.6l1.1-1.1M11.5 4.5l1.1-1.1" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>"#,
};

/// Modern crescent moon icon for dark theme toggle
pub static MOON: Icon = Icon {
    name: "app-moon",
    view: 16.0,
    body: r#"<path d="M13.5 9.5A6 6 0 1 1 6.5 2.5a4.8 4.8 0 0 0 7 7z" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
};

/// Vector flame icon for daily streak
pub static FLAME: Icon = Icon {
    name: "app-flame",
    view: 16.0,
    body: r#"<path d="M8.5 2C8.5 2 5 5 5 8.5C5 10.4 6.6 12 8.5 12C10.4 12 12 10.4 12 8.5C12 6.5 10.5 4.5 10.5 4.5C10.5 4.5 9.8 6 8.8 6.5C7.8 7 7.5 5.5 8.5 2Z" fill="currentColor"/>"#,
};

/// Vector clock icon for reading duration
pub static CLOCK: Icon = Icon {
    name: "app-clock",
    view: 16.0,
    body: r#"<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M8 4.5V8L10.5 9.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>"#,
};

/// Vector lightbulb icon for Diffuse / Intuition mode
pub static BULB: Icon = Icon {
    name: "app-bulb",
    view: 16.0,
    body: r#"<path d="M6 12h4m-3 2h2M8 2a5 5 0 0 0-3.5 8.5c.7.7 1 1.5 1 2.5h5c0-1 .3-1.8 1-2.5A5 5 0 0 0 8 2z" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
};

/// Vector compass / math icon for Focused / Formal mode
pub static COMPASS: Icon = Icon {
    name: "app-compass",
    view: 16.0,
    body: r#"<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M10.5 5.5L7 7L5.5 10.5L9 9z" stroke="currentColor" stroke-width="1" fill="currentColor"/>"#,
};

/// Vector lightning bolt for Checkpoint Evaluasi
pub static ZAP: Icon = Icon {
    name: "app-zap",
    view: 16.0,
    body: r#"<path d="M8.5 2L3.5 9h4.5l-1 5 5.5-7H8l.5-5z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="none"/>"#,
};

/// Clean 3x3 grid matrix icon for Binary Relation Lab
pub static GRID_3X3: Icon = Icon {
    name: "app-grid-3x3",
    view: 16.0,
    body: r#"<rect x="2.5" y="2.5" width="11" height="11" rx="1.5" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M2.5 6.5h11M2.5 10.5h11M6.5 2.5v11M10.5 2.5v11" stroke="currentColor" stroke-width="1"/>"#,
};
