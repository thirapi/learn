use rune::{El, col, el, row, text, ScrollSpec};
use rune_kit::{Button, ButtonVariant, ControlSize, Kit};
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

    let header = col().w_full().gap(8.0).pb(12.0).child(row().child(srs_badge)).child(title).child(subtitle);

    let total_cards = state.flashcards.len();
    let current_idx = state.current_card_idx.min(total_cards.saturating_sub(1));
    let card = &state.flashcards[current_idx];

    let counter = text(format!("Kartu {} dari {}", current_idx + 1, total_cards))
        .font(ty.small_medium)
        .color(c.text_muted);

    // Thin progress bar through the deck.
    let deck_frac = (current_idx + 1) as f32 / total_cards.max(1) as f32;
    let deck_bar = row()
        .w_full()
        .h(4.0)
        .radius(2.0)
        .fill(c.layer_03)
        .child(el().h(4.0).radius(2.0).fill(c.bg_accent).w_frac(deck_frac));

    let meta_row = col().w_full().gap(8.0).child(counter).child(deck_bar);

    // Flashcard surface: a raised card with the question/answer as the hero text.
    let is_flipped = state.card_flipped;

    let side_label = |glyph: &'static rune::Icon, label: String, color: rune::Color| {
        row()
            .center_y()
            .gap(6.0)
            .child(rune::icon(glyph, 13.0).color(color))
            .child(text(label).font(ty.caption_medium).color(color))
    };

    let topic_line = format!("Pertemuan {} • {}", card.module_number, card.topic);

    let mut card_body = col().w_full().gap(14.0);
    if !is_flipped {
        card_body = card_body
            .child(side_label(&icons::HELP, format!("PERTANYAAN • {}", topic_line.to_uppercase()), c.text_accent))
            .child(text(card.question).font(ty.title_md).color(c.text_strong).wrap().w_full())
            .child(
                row()
                    .center_y()
                    .gap(6.0)
                    .child(rune::icon(&icons::BULB, 13.0).color(c.warning.icon))
                    .child(text(format!("Petunjuk: {}", card.hint)).font(ty.small).color(c.text_muted).wrap().shrink()),
            )
            .child(
                row().pt(6.0).child(
                    Button::new("Lihat Jawaban")
                        .icon(&icons::REFRESH)
                        .variant(ButtonVariant::Contrast)
                        .control_size(ControlSize::Large)
                        .on_press(|s: &mut State| s.card_flipped = true)
                        .build(kit),
                ),
            );
    } else {
        card_body = card_body
            .child(side_label(&icons::CIRCLE_CHECK, format!("JAWABAN • {}", topic_line.to_uppercase()), c.success.fg))
            .child(text(card.answer).font(ty.body).color(c.text_base).wrap().w_full())
            .child(
                row().pt(6.0).child(
                    Button::new("Balik ke Pertanyaan")
                        .icon(&icons::REFRESH)
                        .variant(ButtonVariant::Neutral)
                        .control_size(ControlSize::Regular)
                        .on_press(|s: &mut State| s.card_flipped = false)
                        .build(kit),
                ),
            );
    }

    let card_widget = col()
        .w_full()
        .min_h(240.0)
        .padding(28.0)
        .radius(kit.radius.xl)
        .fill(c.layer_01)
        .border(1.0, if is_flipped { c.success.icon.alpha(0.5) } else { c.border_base })
        .box_shadow(theme::soft_card_shadow(state.dark_mode))
        .child(card_body);

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
            .label_with(|style, color| {
                row()
                    .center_y()
                    .gap(6.0)
                    .child(text("Kartu Berikutnya").font(style).color(color))
                    .child(rune::icon(&icons::ARROW_RIGHT, 14.0).color(color))
            })
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
        .px(32.0)
        .py(28.0)
        .child(srs_column);

    col()
        .w_full()
        .h_full()
        .fill(c.page)
        .child(center_wrapper.scroll(ScrollSpec { stick_bottom: false }).grow(1.0).shrink())
}
