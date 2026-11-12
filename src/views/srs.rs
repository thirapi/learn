use rune::{El, col, row, text, ScrollSpec};
use rune_kit::{Badge, BadgeVariant, Button, ButtonVariant, Card, ControlSize, Kit};
use crate::icons;
use crate::state::State;
use crate::theme;

pub fn render(state: &State, kit: &Kit) -> El {
    let c = &kit.c;
    let ty = theme::typography();

    let srs_badge = row()
        .center_y()
        .gap(5.0)
        .h(22.0)
        .px(8.0)
        .radius(kit.radius.xs)
        .fill(c.layer_02)
        .border(0.5, c.border_base)
        .child(rune::icon(&icons::BRAIN, 13.0).color(c.icon_accent))
        .child(text("RETENSI MEMORI JANGKA PANJANG").font(kit.ty.small_medium).color(c.text_base));

    let title = text("Spaced Repetition (SRS Flashcards)")
        .font(ty.display)
        .color(c.text_strong);

    let subtitle = text("Kunci pemahaman konsep jangka panjang dengan pengulangan berjarak adaptif algoritma Leitner.")
        .font(ty.caption)
        .color(c.text_muted);

    let header = col().w_full().gap(8.0).pb(12.0).child(srs_badge).child(title).child(subtitle);

    let total_cards = state.flashcards.len();
    let current_idx = state.current_card_idx.min(total_cards.saturating_sub(1));
    let card = &state.flashcards[current_idx];

    let counter_badge = Badge::new(format!("Kartu {} dari {}", current_idx + 1, total_cards))
        .variant(BadgeVariant::Accent)
        .standard()
        .build(kit);

    let mod_badge = Badge::new(format!("Pertemuan {}", card.module_number))
        .variant(BadgeVariant::Subtle)
        .standard()
        .build(kit);

    let topic_text = text(card.topic)
        .font(ty.title_sm)
        .color(c.text_strong);

    let meta_row = row()
        .w_full()
        .center_y()
        .justify_between()
        .flex_wrap()
        .gap(8.0)
        .child(row().center_y().gap(8.0).child(counter_badge).child(mod_badge))
        .child(topic_text);

    // Flashcard Surface using rune-kit Card & Vector Icons
    let is_flipped = state.card_flipped;

    let card_widget = if !is_flipped {
        Card::new()
            .accent(c.icon_base)
            .title(format!("Pertanyaan: {}", card.topic), Some(&icons::HELP))
            .description(card.question)
            .child(
                row()
                    .center_y()
                    .gap(6.0)
                    .pt(8.0)
                    .child(Badge::new(format!("Petunjuk: {}", card.hint)).variant(BadgeVariant::Subtle).build(kit))
            )
            .action(
                Button::new("Buka & Lihat Jawaban")
                    .icon(&icons::REFRESH)
                    .variant(ButtonVariant::Contrast)
                    .control_size(ControlSize::Large)
                    .on_press(|s: &mut State| s.card_flipped = true)
                    .build(kit)
            )
            .build(kit)
    } else {
        Card::new()
            .accent(c.success.icon)
            .title(format!("Kunci Jawaban: {}", card.topic), Some(&icons::CIRCLE_CHECK))
            .description(card.answer)
            .action(
                Button::new("Balik ke Pertanyaan")
                    .icon(&icons::REFRESH)
                    .variant(ButtonVariant::Neutral)
                    .control_size(ControlSize::Large)
                    .on_press(|s: &mut State| s.card_flipped = false)
                    .build(kit)
            )
            .build(kit)
    };

    // SRS Action buttons with vector icons
    let cid = card.id;
    let srs_action_box = if is_flipped {
        let hard_btn = Button::new("Lupa / Sulit (Besok)")
            .icon(&icons::CIRCLE_X)
            .variant(ButtonVariant::Danger)
            .control_size(ControlSize::ExtraLarge)
            .on_press(move |s: &mut State| {
                s.card_levels.insert(cid, 1);
                s.card_flipped = false;
                if s.current_card_idx + 1 < s.flashcards.len() {
                    s.current_card_idx += 1;
                } else {
                    s.current_card_idx = 0;
                }
            })
            .build(kit);

        let good_btn = Button::new("Sedang (3 Hari)")
            .icon(&icons::REFRESH)
            .variant(ButtonVariant::Warning)
            .control_size(ControlSize::ExtraLarge)
            .on_press(move |s: &mut State| {
                s.card_levels.insert(cid, 2);
                s.card_flipped = false;
                if s.current_card_idx + 1 < s.flashcards.len() {
                    s.current_card_idx += 1;
                } else {
                    s.current_card_idx = 0;
                }
            })
            .build(kit);

        let easy_btn = Button::new("Mudah (7 Hari)")
            .icon(&icons::CHECK)
            .variant(ButtonVariant::Submit)
            .control_size(ControlSize::ExtraLarge)
            .on_press(move |s: &mut State| {
                s.card_levels.insert(cid, 3);
                s.card_flipped = false;
                if s.current_card_idx + 1 < s.flashcards.len() {
                    s.current_card_idx += 1;
                } else {
                    s.current_card_idx = 0;
                }
            })
            .build(kit);

        row()
            .w_full()
            .gap(12.0)
            .flex_wrap()
            .child(hard_btn.grow(1.0).shrink())
            .child(good_btn.grow(1.0).shrink())
            .child(easy_btn.grow(1.0).shrink())
    } else {
        let prev_btn = Button::new("Kartu Sebelumnya")
            .icon(&icons::ARROW_LEFT)
            .variant(ButtonVariant::Neutral)
            .control_size(ControlSize::Regular)
            .on_press(|s: &mut State| {
                s.card_flipped = false;
                if s.current_card_idx > 0 {
                    s.current_card_idx -= 1;
                } else {
                    s.current_card_idx = s.flashcards.len() - 1;
                }
            })
            .build(kit);

        let next_btn = Button::new("Kartu Berikutnya")
            .icon(&icons::ARROW_RIGHT)
            .variant(ButtonVariant::Neutral)
            .control_size(ControlSize::Regular)
            .on_press(|s: &mut State| {
                s.card_flipped = false;
                if s.current_card_idx + 1 < s.flashcards.len() {
                    s.current_card_idx += 1;
                } else {
                    s.current_card_idx = 0;
                }
            })
            .build(kit);

        row()
            .w_full()
            .justify_between()
            .gap(12.0)
            .flex_wrap()
            .child(prev_btn)
            .child(next_btn)
    };

    // Responsive Inner Container (Max Width 720px)
    let srs_column = col()
        .w_full()
        .max_w(720.0)
        .gap(20.0)
        .child(header)
        .child(meta_row)
        .child(card_widget)
        .child(srs_action_box)
        .pb(56.0);

    // Scrollable Outer Container centered horizontally
    let center_wrapper = col()
        .w_full()
        .items_center()
        .px(24.0)
        .py(20.0)
        .child(srs_column);

    col()
        .w_full()
        .h_full()
        .fill(c.page)
        .child(center_wrapper.scroll(ScrollSpec { stick_bottom: false }).grow(1.0).shrink())
}
