use rune::{El, el, col, row, text, Color, Tx};
use rune_kit::{
    Badge, BadgeVariant, Button, ButtonVariant, ControlSize, Kit, ProgressCircle,
    Sidebar, SidebarRow, SidebarSection,
};
use crate::icons;
use crate::state::State;
use crate::theme;

pub fn render(state: &State, kit: &Kit) -> El {
    let c = &kit.c;
    let ty = theme::typography();

    // 1. Header with Title, ProgressCircle, and Vector Collapse Button
    let title = text("Kurikulum Kuliah")
        .font(ty.title_sm)
        .color(c.text_strong);

    let close_btn = Button::new("Tutup")
        .icon(&icons::CHEVRON_LEFT)
        .variant(ButtonVariant::Ghost)
        .control_size(ControlSize::Small)
        .on_press(|s: &mut State| {
            s.sidebar_open = false;
        })
        .build(kit);

    let title_row = row()
        .w_full()
        .center_y()
        .justify_between()
        .child(title)
        .child(close_btn);

    let pct = state.progress_percentage();
    let pct_f32 = ((pct / 100.0).clamp(0.0, 1.0)) as f32;

    let prog_circle = ProgressCircle::new(pct_f32)
        .size(24.0)
        .label("Progres Selesai")
        .build(kit);

    let subtitle = text("7 Modul • 21 Topik Mikro")
        .font(ty.caption)
        .color(c.text_muted);

    let progress_box = row()
        .w_full()
        .center_y()
        .justify_between()
        .px(10.0)
        .py(8.0)
        .radius(kit.radius.sm)
        .fill(c.layer_01)
        .border(1.0, c.border_base)
        .child(
            row()
                .center_y()
                .gap(8.0)
                .child(prog_circle)
                .child(
                    col()
                        .child(text(format!("{:.0}% Selesai", pct)).font(ty.caption_medium).color(c.text_strong))
                        .child(subtitle)
                )
        )
        .child(
            Badge::new("Aktif")
                .variant(BadgeVariant::Accent)
                .standard()
                .build(kit)
        );

    let header_box = col()
        .w_full()
        .gap(8.0)
        .pb(8.0)
        .child(title_row)
        .child(progress_box);

    // 2. Official rune-kit Sidebar with SidebarSection & SidebarRow
    let mut sidebar_widget = Sidebar::new()
        .label("Silabus Perkuliahan Matematika Diskrit")
        .header(header_box);

    for (m_idx, module) in state.modules.iter().enumerate() {
        let is_selected_module = state.active_module_idx == m_idx;

        let mut section = SidebarSection::new(format!("P{} • {}", module.number, module.title))
            .count(module.chunks.len())
            .collapsed(!is_selected_module)
            .on_toggle(move |s: &mut State, _tx: &mut Tx| {
                s.active_module_idx = m_idx;
                s.active_chunk_idx = 0;
            });

        for (c_idx, chunk) in module.chunks.iter().enumerate() {
            let is_selected_chunk = is_selected_module && state.active_chunk_idx == c_idx;
            let is_completed = state.completed_chunks.contains(chunk.id);

            let status_badge = if is_completed {
                el()
                    .size(16.0, 16.0)
                    .radius(8.0)
                    .fill(c.success.icon)
                    .center()
                    .child(rune::icon(&icons::CHECK, 10.0).color(Color::WHITE))
            } else if is_selected_chunk {
                el()
                    .size(16.0, 16.0)
                    .radius(8.0)
                    .fill(c.bg_accent)
                    .center()
                    .child(text(format!("{}", c_idx + 1)).font(ty.caption_medium).color(Color::WHITE))
            } else {
                el()
                    .size(16.0, 16.0)
                    .radius(8.0)
                    .fill(c.layer_02)
                    .center()
                    .child(text(format!("{}", c_idx + 1)).font(ty.caption).color(c.text_muted))
            };

            let row_item = SidebarRow::new(chunk.title)
                .avatar(status_badge)
                .detail(chunk.reading_time)
                .selected(is_selected_chunk)
                .on_select(move |s: &mut State, _tx: &mut Tx| {
                    s.active_module_idx = m_idx;
                    s.active_chunk_idx = c_idx;
                })
                .build(kit);

            section = section.row(row_item);
        }

        sidebar_widget = sidebar_widget.child(section.build(kit));
    }

    // Modern Outer Sidebar Column
    col()
        .w(300.0)
        .h_full()
        .fill(c.surface_base)
        .box_shadow(rune::Shadow {
            x: 1.0,
            blur: 0.0,
            color: c.border_base,
            ..Default::default()
        })
        .child(sidebar_widget.build(kit))
}
