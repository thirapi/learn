use rune::{El, el, row, text};
use rune_kit::{Button, ButtonVariant, ControlSize, IconButton, IconButtonVariant, Kit, ProgressCircle, SegmentedControl};
use crate::icons;
use crate::state::{State, Tab};
use crate::theme;

pub fn render(state: &State, kit: &Kit) -> El {
    let c = &kit.c;
    let ty = theme::typography();

    // Left: Brand / Logo & Sidebar Toggle
    let logo_icon = el()
        .size(32.0, 32.0)
        .radius(kit.radius.sm)
        .fill(c.bg_accent)
        .center()
        .child(rune::icon(&icons::GRADUATION_CAP, 16.0).color(rune::Color::WHITE));

    let logo_title = text("Learn")
        .font(ty.title_md)
        .color(c.text_strong);

    let mut brand_box = row()
        .center_y()
        .gap(10.0)
        .child(logo_icon)
        .child(logo_title);

    // Sidebar toggle button with vector icon
    if state.active_tab == Tab::Belajar {
        let sidebar_btn = Button::new("Silabus")
            .icon(&icons::SIDEBAR)
            .variant(if state.sidebar_open { ButtonVariant::Ghost } else { ButtonVariant::Neutral })
            .control_size(ControlSize::Small)
            .on_press(|s: &mut State| s.sidebar_open = !s.sidebar_open)
            .build(kit);
        brand_box = brand_box.child(sidebar_btn);
    }

    // Center: SegmentedControl (Rune-kit official component)
    let active_idx = match state.active_tab {
        Tab::Belajar => 0,
        Tab::Flashcards => 1,
        Tab::Lab => 2,
        Tab::Feynman => 3,
    };

    let tabs = ["Belajar", "Flashcards", "Lab Logika", "Feynman Studio"];
    let tab_ctrl = SegmentedControl::new(&tabs, Some(active_idx))
        .fit()
        .on_pick(|s: &mut State, idx, _| {
            s.active_tab = match idx {
                0 => Tab::Belajar,
                1 => Tab::Flashcards,
                2 => Tab::Lab,
                _ => Tab::Feynman,
            };
        })
        .build(kit);

    // Right: Badges, ProgressCircle & Vector Theme Toggle Button
    let pct = state.progress_percentage();
    let pct_f32 = ((pct / 100.0).clamp(0.0, 1.0)) as f32;

    // Status pills share one shape: icon/ring + short label.
    let pill = || {
        row()
            .center_y()
            .gap(6.0)
            .h(28.0)
            .px(10.0)
            .radius(14.0)
            .fill(c.layer_02)
    };

    let streak_badge = pill()
        .child(rune::icon(&icons::FLAME, 13.0).color(c.warning.icon))
        .child(text(format!("{} hari", state.daily_streak)).font(kit.ty.small_medium).color(c.text_base));

    let progress_widget = pill()
        .child(
            ProgressCircle::new(pct_f32)
                .size(14.0)
                .label("Persentase Selesai")
                .build(kit),
        )
        .child(text(format!("{:.0}% selesai", pct)).font(kit.ty.small_medium).color(c.text_base));

    let theme_icon = if state.dark_mode { &icons::SUN } else { &icons::MOON };
    let theme_btn = IconButton::new(theme_icon)
        .variant(IconButtonVariant::Neutral)
        .control_size(ControlSize::Small)
        .help(if state.dark_mode { "Beralih ke Mode Terang" } else { "Beralih ke Mode Gelap" })
        .on_press(|s: &mut State| {
            s.dark_mode = !s.dark_mode;
        })
        .build(kit);

    let right_box = row()
        .center_y()
        .gap(8.0)
        .child(streak_badge)
        .child(progress_widget)
        .child(theme_btn);

    // Modern Header Navbar with crisp 1px border and soft shadow
    row()
        .w_full()
        .h(58.0)
        .px(20.0)
        .center_y()
        .justify_between()
        .fill(c.surface_base)
        .box_shadow(rune::Shadow {
            y: 1.0,
            blur: 0.0,
            color: c.border_base,
            ..Default::default()
        })
        .child(brand_box)
        .child(tab_ctrl)
        .child(right_box)
}
