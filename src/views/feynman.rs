use rune::{El, col, row, text, ScrollSpec};
use rune_kit::{Button, ButtonVariant, Card, ControlSize, Kit};
use crate::icons;
use crate::state::{State, Tab};
use crate::theme;

pub fn render(state: &State, kit: &Kit) -> El {
    let c = &kit.c;
    let ty = theme::typography();

    let feynman_badge = row()
        .center_y()
        .gap(5.0)
        .h(22.0)
        .px(8.0)
        .radius(kit.radius.xs)
        .fill(c.layer_02)
        .border(0.5, c.border_base)
        .child(rune::icon(&icons::BULB, 13.0).color(c.warning.icon))
        .child(text("FEYNMAN PEDAGOGY STUDIO").font(kit.ty.small_medium).color(c.text_base));

    let title = text("Feynman Technique Studio")
        .font(ty.display)
        .color(c.text_strong);

    let subtitle = text("Metode belajar paling ampuh: 'Jika kamu tidak bisa menjelaskannya secara sederhana, kamu belum memahaminya.' — Richard Feynman")
        .font(ty.caption)
        .color(c.text_muted);

    let header = col().w_full().gap(8.0).pb(12.0).child(row().child(feynman_badge)).child(title).child(subtitle);

    let module = state.current_module();
    let chunk = state.current_chunk();

    // 1. Target Topic Card with Brain Vector Icon
    let target_card = Card::new()
        .accent(c.icon_base)
        .title(format!("Tantangan Eksplanasi: Pertemuan {} • {}", module.number, chunk.title), Some(&icons::BRAIN))
        .description("Buat satu analogi atau cerita pendek yang menjelaskan ide utama topik ini kepada orang awam tanpa menggunakan rumus abstrak!")
        .build(kit);

    // 2. Key Concepts Checklist using Vector Check Icons
    let mut checklist_col = col().w_full().gap(8.0).pt(6.0);
    for concept in chunk.key_concepts {
        let check_icon = rune::el().size(16.0, 16.0).center().child(rune::el().size(6.0, 6.0).radius(3.0).fill(c.success.icon));
        let row_el = row()
            .w_full()
            .center_y()
            .gap(10.0)
            .child(check_icon)
            .child(text(*concept).font(ty.body).color(c.text_base));
        checklist_col = checklist_col.child(row_el);
    }

    let checklist_card = Card::new()
        .accent(c.success.icon)
        .title("Checklist Konsep Kunci dalam Penjelasanmu:", Some(&icons::CHECKLIST))
        .child(checklist_col)
        .build(kit);

    // 3. 4-step Feynman Rubric using Vector Edit Icons
    let steps_data = [
        ("1. Identifikasi Titik Buta", "Buka catatan atau slide kuliah HANYA saat kamu mandek mencari padanan analogi yang pas.", c.warning.icon),
        ("2. Sederhanakan Kosakata", "Ganti istilah rumit (seperti 'injeksi', 'kardinalitas') dengan padanan nyata sehari-hari.", c.info.icon),
        ("3. Uji Coba ke Teman", "Bacakan penjelasanmu ke teman. Jika mereka mengangguk paham di detik ke-10, konsep sudah terkunci!", c.success.icon),
        ("4. Evaluasi Balik Formal", "Kembalilah ke 'Focused Mode' di tab Modul Belajar untuk memastikan analogimu selaras dengan matematika.", c.icon_base),
    ];

    let mut rubric_col = col().w_full().gap(10.0).pt(6.0);
    for (s_title, s_desc, accent_color) in steps_data {
        let card = Card::new()
            .accent(accent_color)
            .title(s_title, Some(&icons::EDIT))
            .description(s_desc)
            .build(kit);
        rubric_col = rubric_col.child(card);
    }

    let rubric_section = col()
        .w_full()
        .gap(8.0)
        .child(text("4 Langkah Panduan Praktik Teknik Feynman:").font(ty.title_sm).color(c.text_strong))
        .child(rubric_col);

    // Action Button with Vector Arrow
    let action_btn = Button::new("Kembali ke Modul Belajar untuk Menguji Pemahaman")
        .icon(&icons::ARROW_LEFT)
        .variant(ButtonVariant::Submit)
        .control_size(ControlSize::ExtraLarge)
        .on_press(|s: &mut State| {
            s.active_tab = Tab::Belajar;
        })
        .build(kit);

    // Responsive Inner Container (Max Width 860px)
    let feynman_column = col()
        .w_full()
        .max_w(720.0)
        .gap(22.0)
        .child(header)
        .child(target_card)
        .child(checklist_card)
        .child(rubric_section)
        .child(action_btn)
        .pb(56.0);

    // Scrollable Outer Container centered horizontally
    let center_wrapper = col()
        .w_full()
        .items_center()
        .px(32.0)
        .py(28.0)
        .child(feynman_column);

    col()
        .w_full()
        .h_full()
        .fill(c.page)
        .child(center_wrapper.scroll(ScrollSpec { stick_bottom: false }).grow(1.0).shrink())
}
