use rune::{El, col, el, row, text, ScrollSpec, Color};
use rune_kit::{Button, ButtonVariant, Card, ControlSize, Kit, Note};
use crate::icons;
use crate::state::State;
use crate::theme;

pub fn render(state: &State, kit: &Kit) -> El {
    let module = state.current_module();
    let chunk = state.current_chunk();
    let c = &kit.c;
    let ty = theme::typography();

    // 1. Sidebar Toggle Banner (if collapsed)
    let mut top_header = col().w_full().gap(10.0).pb(12.0);

    if !state.sidebar_open {
        let expand_btn = Button::new("Buka Kurikulum Perkuliahan")
            .icon(&icons::SIDEBAR)
            .variant(ButtonVariant::Neutral)
            .control_size(ControlSize::Small)
            .on_press(|s: &mut State| s.sidebar_open = true)
            .build(kit);

        top_header = top_header.child(
            row().w_full().child(expand_btn).pb(4.0)
        );
    }

    // Breadcrumbs & Title
    let breadcrumbs = text(format!("Matematika Diskrit › Pertemuan {} • {}", module.number, module.title))
        .font(ty.caption_medium)
        .color(c.text_accent);

    let chunk_title = text(format!("{}: {}", chunk.id, chunk.title))
        .font(ty.display)
        .color(c.text_strong);

    // Badges Row
    let is_completed = state.completed_chunks.contains(chunk.id);

    let time_badge = row()
        .center_y()
        .gap(5.0)
        .h(22.0)
        .px(8.0)
        .radius(kit.radius.xs)
        .fill(c.layer_02)
        .border(0.5, c.border_base)
        .child(rune::icon(&icons::CLOCK, 13.0).color(c.icon_muted))
        .child(text(chunk.reading_time).font(kit.ty.small_medium).color(c.text_muted));

    let method_badge = row()
        .center_y()
        .gap(5.0)
        .h(22.0)
        .px(8.0)
        .radius(kit.radius.xs)
        .fill(c.layer_02)
        .border(0.5, c.border_base)
        .child(rune::icon(&icons::BRAIN, 13.0).color(c.icon_accent))
        .child(text("Dual Coding Method").font(kit.ty.small_medium).color(c.text_base));

    let status_badge = if is_completed {
        row()
            .center_y()
            .gap(5.0)
            .h(22.0)
            .px(8.0)
            .radius(kit.radius.xs)
            .fill(c.success.icon.alpha(0.12))
            .border(0.5, c.success.icon.alpha(0.3))
            .child(rune::icon(&icons::CHECK, 11.0).color(c.success.icon))
            .child(text("Sudah Selesai").font(kit.ty.small_medium).color(c.success.fg))
    } else {
        row()
            .center_y()
            .gap(5.0)
            .h(22.0)
            .px(8.0)
            .radius(kit.radius.xs)
            .fill(c.layer_02)
            .border(0.5, c.border_base)
            .child(rune::icon(&icons::CIRCLE_X, 11.0).color(c.text_faint))
            .child(text("Belum Selesai").font(kit.ty.small_medium).color(c.text_muted))
    };

    let badges_row = row()
        .center_y()
        .gap(8.0)
        .flex_wrap()
        .child(time_badge)
        .child(method_badge)
        .child(status_badge);

    top_header = top_header
        .child(breadcrumbs)
        .child(chunk_title)
        .child(badges_row);

    // 2. Interactive Learning Mode Selector (Intuisi vs Formal)
    let is_diffuse = state.is_diffuse_mode;

    let diffuse_btn = row()
        .px(16.0)
        .py(10.0)
        .radius(kit.radius.sm)
        .fill(if is_diffuse { c.bg_accent } else { Color::TRANSPARENT })
        .hover(if is_diffuse { c.bg_accent } else { c.layer_02 })
        .center()
        .gap(8.0)
        .child(rune::icon(&icons::BULB, 15.0).color(if is_diffuse { Color::WHITE } else { c.text_base }))
        .child(
            text("Mode Intuisi (Analogi & Pola Pikir)")
                .font(if is_diffuse { ty.body_medium } else { ty.body })
                .color(if is_diffuse { Color::WHITE } else { c.text_base })
        )
        .on_press(|s: &mut State| s.is_diffuse_mode = true);

    let focused_btn = row()
        .px(16.0)
        .py(10.0)
        .radius(kit.radius.sm)
        .fill(if !is_diffuse { c.bg_accent } else { Color::TRANSPARENT })
        .hover(if !is_diffuse { c.bg_accent } else { c.layer_02 })
        .center()
        .gap(8.0)
        .child(rune::icon(&icons::COMPASS, 15.0).color(if !is_diffuse { Color::WHITE } else { c.text_base }))
        .child(
            text("Mode Formal (Definisi & Notasi Matematis)")
                .font(if !is_diffuse { ty.body_medium } else { ty.body })
                .color(if !is_diffuse { Color::WHITE } else { c.text_base })
        )
        .on_press(|s: &mut State| s.is_diffuse_mode = false);

    let mode_switcher = row()
        .w_full()
        .padding(4.0)
        .radius(kit.radius.md)
        .fill(c.layer_01)
        .border(1.0, c.border_base)
        .child(diffuse_btn.grow(1.0))
        .child(focused_btn.grow(1.0));

    // 3. Modern Reading Container powered by rune-kit Note & Card
    let current_content = if state.is_diffuse_mode {
        chunk.diffuse_content
    } else {
        chunk.focused_content
    };

    let callout_intro = if is_diffuse {
        "> [!tip] Mode Intuisi (Diffuse Mode)\n> Pahami ide inti dan pola pikir menggunakan analogi dunia nyata di bawah ini sebelum mendalami notasi formal matematika."
    } else {
        "> [!note] Mode Formal (Focused Mode)\n> Landasan teori matematika ketat: perhatikan himpunan, kuantor, dan pembuktian matematis secara seksama."
    };

    let combined_doc = format!("{}\n\n{}", callout_intro, current_content);
    let note_content = Note::new(&combined_doc).build(kit);

    let content_box = Card::new()
        .accent(c.icon_base)
        .title(
            if is_diffuse { "Pemahaman Intuitif & Analogi Nyata" } else { "Landasan Teori Formal & Notasi Simbolik" },
            Some(&icons::GRADUATION_CAP),
        )
        .description(format!("Topik {}: {}", chunk.id, chunk.title))
        .child(note_content)
        .build(kit);

    // 4. Modern Active Recall Quiz Section (Tactile Multiple-Choice Tiles)
    let recall_title = row()
        .center_y()
        .gap(8.0)
        .child(
            row()
                .center_y()
                .gap(5.0)
                .h(22.0)
                .px(8.0)
                .radius(kit.radius.xs)
                .fill(c.warning.icon.alpha(0.12))
                .border(0.5, c.warning.icon.alpha(0.3))
                .child(rune::icon(&icons::ZAP, 12.0).color(c.warning.icon))
                .child(text("CHECKPOINT EVALUASI").font(kit.ty.small_medium).color(c.warning.fg))
        )
        .child(
            text("Active Recall Checkpoint")
                .font(ty.title_lg)
                .color(c.text_strong)
        );

    let recall_subtitle = text("Jawab pertanyaan berikut tanpa melihat kembali teks materi di atas untuk mengunci retensi memori jangka panjang.")
        .font(ty.caption)
        .color(c.text_muted);

    let recall_header = col()
        .w_full()
        .gap(4.0)
        .pt(20.0)
        .child(recall_title)
        .child(recall_subtitle);

    let mut recall_cards = col().w_full().gap(18.0).child(recall_header);

    let m_idx = state.active_module_idx;
    let c_idx = state.active_chunk_idx;

    for (q_idx, q) in chunk.quiz.iter().enumerate() {
        let key = format!("{}-{}-{}", m_idx, c_idx, q_idx);
        let selected_opt = state.selected_answers.get(&key).copied();

        let mut options_col = col().w_full().gap(8.0).pt(4.0);

        for (opt_idx, opt_text) in q.options.iter().enumerate() {
            let opt_key = key.clone();
            let is_selected = selected_opt == Some(opt_idx);
            let is_correct_choice = opt_idx == q.correct_idx;

            let letter = match opt_idx {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => "D",
            };

            // Styling states
            let (tile_bg, tile_border, letter_bg, letter_color, text_color) = match selected_opt {
                Some(_) if is_selected && is_correct_choice => (
                    c.success.icon.alpha(if state.dark_mode { 0.22 } else { 0.10 }),
                    c.success.icon,
                    c.success.icon,
                    Color::WHITE,
                    c.text_strong,
                ),
                Some(_) if is_selected && !is_correct_choice => (
                    c.danger.icon.alpha(if state.dark_mode { 0.22 } else { 0.10 }),
                    c.danger.icon,
                    c.danger.icon,
                    Color::WHITE,
                    c.text_strong,
                ),
                Some(_) if is_correct_choice => (
                    c.success.icon.alpha(if state.dark_mode { 0.15 } else { 0.08 }),
                    c.success.icon.alpha(0.7),
                    c.success.icon.alpha(0.7),
                    Color::WHITE,
                    c.text_base,
                ),
                _ => (
                    c.layer_01,
                    c.border_base,
                    c.layer_02,
                    c.text_muted,
                    c.text_base,
                ),
            };

            let letter_pill = el()
                .size(28.0, 28.0)
                .radius(14.0)
                .fill(letter_bg)
                .center()
                .child(text(letter).font(ty.caption_medium).color(letter_color));

            let opt_label = text(*opt_text)
                .font(if is_selected { ty.body_medium } else { ty.body })
                .color(text_color)
                .grow(1.0);

            let mut opt_row = row()
                .w_full()
                .px(14.0)
                .py(12.0)
                .radius(kit.radius.md)
                .fill(tile_bg)
                .border(1.0, tile_border)
                .hover(c.layer_02)
                .center_y()
                .gap(12.0)
                .child(letter_pill)
                .child(opt_label)
                .on_press(move |s: &mut State| {
                    s.selected_answers.insert(opt_key.clone(), opt_idx);
                    let cid = s.current_chunk().id;
                    s.mark_chunk_completed(cid);
                });

            if let Some(_) = selected_opt {
                if is_selected && is_correct_choice {
                    opt_row = opt_row.child(
                        row()
                            .center_y()
                            .gap(4.0)
                            .child(rune::icon(&icons::CHECK, 14.0).color(c.success.icon))
                            .child(text("Tepat!").font(ty.caption_medium).color(c.success.icon))
                    );
                } else if is_selected && !is_correct_choice {
                    opt_row = opt_row.child(
                        row()
                            .center_y()
                            .gap(4.0)
                            .child(rune::icon(&icons::CIRCLE_X, 14.0).color(c.danger.icon))
                            .child(text("Salah").font(ty.caption_medium).color(c.danger.icon))
                    );
                }
            }

            options_col = options_col.child(opt_row);
        }

        let mut card_builder = Card::new()
            .accent(c.icon_base)
            .title(format!("{}. {}", q_idx + 1, q.question), Some(&icons::HELP))
            .child(options_col);

        // Feedback Explanation Box using rune-kit Card
        if let Some(sel) = selected_opt {
            let is_correct = sel == q.correct_idx;
            let feedback_el = if is_correct {
                Card::new()
                    .accent(c.success.icon)
                    .title("Jawaban Tepat!", Some(&icons::CIRCLE_CHECK))
                    .description(q.explanation)
                    .build(kit)
            } else {
                Card::new()
                    .accent(c.danger.icon)
                    .title("Kurang Tepat, Penjelasan Konseptual:", Some(&icons::CIRCLE_X))
                    .description(q.explanation)
                    .build(kit)
            };

            card_builder = card_builder.child(feedback_el);
        }

        recall_cards = recall_cards.child(card_builder.build(kit));
    }

    // 5. Button for Bottom Navigation with Vector Arrow
    let chunk_id_copy = chunk.id;
    let complete_btn = Button::new("Tandai Selesai & Lanjut ke Topik Berikutnya")
        .icon(&icons::ARROW_RIGHT)
        .variant(ButtonVariant::Submit)
        .control_size(ControlSize::ExtraLarge)
        .on_press(move |s: &mut State| {
            s.mark_chunk_completed(chunk_id_copy);
            if s.active_chunk_idx + 1 < s.current_module().chunks.len() {
                s.active_chunk_idx += 1;
            } else if s.active_module_idx + 1 < s.modules.len() {
                s.active_module_idx += 1;
                s.active_chunk_idx = 0;
            }
        })
        .build(kit);

    // Responsive Inner Container (Max Width 860px)
    let read_column = col()
        .w_full()
        .max_w(860.0)
        .gap(22.0)
        .child(top_header)
        .child(mode_switcher)
        .child(content_box)
        .child(recall_cards)
        .child(complete_btn)
        .pb(56.0);

    // Scrollable Outer Container centered horizontally
    let center_wrapper = col()
        .w_full()
        .items_center()
        .px(24.0)
        .py(20.0)
        .child(read_column);

    col()
        .w_full()
        .h_full()
        .fill(c.page)
        .child(center_wrapper.scroll(ScrollSpec { stick_bottom: false }).grow(1.0).shrink())
}
