pub mod curriculum;
pub mod icons;
pub mod state;
pub mod theme;
pub mod views;

use rune::{AppSpec, Cx, El, col, row};
use state::{State, Tab};

pub fn view(state: &State, _cx: &mut Cx<State>) -> El {
    let kit = theme::tailwind_kit(state.dark_mode);

    let navbar = views::navbar::render(state, &kit);

    let body = match state.active_tab {
        Tab::Belajar => {
            let mut r = row().w_full().grow(1.0).shrink();
            if state.sidebar_open {
                let sidebar = views::sidebar::render(state, &kit);
                r = r.child(sidebar);
            }
            let lesson = views::lesson::render(state, &kit);
            r.child(lesson.grow(1.0).shrink())
        }
        Tab::Flashcards => {
            views::srs::render(state, &kit).grow(1.0).shrink()
        }
        Tab::Lab => {
            views::lab::render(state, &kit).grow(1.0).shrink()
        }
        Tab::Feynman => {
            views::feynman::render(state, &kit).grow(1.0).shrink()
        }
    };

    col()
        .w_full()
        .h_full()
        .fill(kit.c.page)
        .child(navbar)
        .child(body)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn start() {
    rune::run(AppSpec {
        fonts: rune_theme_opencode::FONTS,
        families: rune_theme_opencode::FAMILIES,
        min_size: Some((720.0, 480.0)),
        scenes: &[
            ("dark", |s: &mut State| s.dark_mode = true),
            ("focused", |s: &mut State| s.is_diffuse_mode = false),
            ("flashcards", |s: &mut State| s.active_tab = Tab::Flashcards),
            ("lab", |s: &mut State| s.active_tab = Tab::Lab),
            ("feynman", |s: &mut State| s.active_tab = Tab::Feynman),
        ],
        ..AppSpec::new("DiskritLearn", (1160.0, 760.0), State::default, view)
    });
}
