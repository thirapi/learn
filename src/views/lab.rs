use rune::{El, col, el, row, text, ScrollSpec};
use rune_kit::{Badge, BadgeVariant, Button, ButtonVariant, Card, ControlSize, Kit, SegmentedControl, Switch};
use crate::icons;
use crate::state::State;
use crate::theme;

pub fn render(state: &State, kit: &Kit) -> El {
    let c = &kit.c;
    let ty = theme::typography();

    let lab_badge = row()
        .center_y()
        .gap(5.0)
        .h(22.0)
        .px(8.0)
        .radius(kit.radius.xs)
        .fill(c.layer_02)
        .border(0.5, c.border_base)
        .child(rune::icon(&icons::FLASK, 12.0).color(c.icon_accent))
        .child(text("LABORATORIUM KOMPUTASI").font(kit.ty.small_medium).color(c.text_base));

    let title = text("Lab Interaktif Logika & Relasi")
        .font(ty.display)
        .color(c.text_strong);

    let subtitle = text("Eksperimen langsung dengan simulasi kalkulasi tabel kebenaran dan penguji otomatis 4 sifat relasi biner.")
        .font(ty.caption)
        .color(c.text_muted);

    let header = col().w_full().gap(8.0).pb(8.0).child(lab_badge).child(title).child(subtitle);

    // 1. Truth Table Simulator with rune-kit Card & Switch
    let p_switch = Switch::new(state.truth_p)
        .label(format!("Variabel p: {}", if state.truth_p { "TRUE (1)" } else { "FALSE (0)" }))
        .standard()
        .on_toggle(|s: &mut State| s.truth_p = !s.truth_p)
        .build(kit);

    let q_switch = Switch::new(state.truth_q)
        .label(format!("Variabel q: {}", if state.truth_q { "TRUE (1)" } else { "FALSE (0)" }))
        .standard()
        .on_toggle(|s: &mut State| s.truth_q = !s.truth_q)
        .build(kit);

    let toggles_row = row().w_full().gap(24.0).flex_wrap().center_y().child(p_switch).child(q_switch);

    let op_ctrl = SegmentedControl::new(
        &["AND (∧)", "OR (∨)", "IF-THEN (→)", "IFF (↔)", "XOR (⊕)"],
        Some(state.truth_op),
    )
    .width(None)
    .on_pick(|s: &mut State, idx, _| s.truth_op = idx)
    .build(kit);

    let p = state.truth_p;
    let q = state.truth_q;
    let (res_bool, formula_name, rule_desc) = match state.truth_op {
        0 => (p && q, "p ∧ q", "Konjungsi bernilai TRUE hanya jika kedua proposisi bernilai TRUE."),
        1 => (p || q, "p ∨ q", "Disjungsi bernilai FALSE hanya jika kedua proposisi bernilai FALSE."),
        2 => (!p || q, "p → q", "Implikasi bernilai FALSE HANYA jika anteseden (p) Benar dan konsekuen (q) Salah."),
        3 => (p == q, "p ↔ q", "Biimplikasi bernilai TRUE jika kedua proposisi bernilai identik (T-T atau F-F)."),
        _ => (p ^ q, "p ⊕ q", "XOR bernilai TRUE jika tepat salah satu proposisi bernilai Benar."),
    };

    let result_card = Card::new()
        .accent(if res_bool { c.success.icon } else { c.danger.icon })
        .title(
            format!("HASIL OPERASI: {}", if res_bool { "TRUE (1)" } else { "FALSE (0)" }),
            Some(if res_bool { &icons::CIRCLE_CHECK } else { &icons::CIRCLE_X }),
        )
        .description(format!("Rumus: {} • {}", formula_name, rule_desc))
        .build(kit);

    // Dynamic 4-row Truth Table
    let table_header = row()
        .w_full()
        .padding(8.0)
        .radius(kit.radius.sm)
        .fill(c.layer_02)
        .child(el().w(80.0).child(text("Nilai p").font(ty.caption_medium).color(c.text_muted)))
        .child(el().w(80.0).child(text("Nilai q").font(ty.caption_medium).color(c.text_muted)))
        .child(el().w(140.0).child(text(formula_name).font(ty.caption_medium).color(c.text_muted)))
        .child(el().grow(1.0).child(text("Status").font(ty.caption_medium).color(c.text_muted)));

    let rows_data = [
        (true, true),
        (true, false),
        (false, true),
        (false, false),
    ];

    let mut table_col = col().w_full().gap(4.0).child(table_header);

    for (rp, rq) in rows_data {
        let is_current = (rp == p) && (rq == q);
        let row_res = match state.truth_op {
            0 => rp && rq,
            1 => rp || rq,
            2 => !rp || rq,
            3 => rp == rq,
            _ => rp ^ rq,
        };

        let row_el = row()
            .w_full()
            .px(8.0)
            .py(7.0)
            .radius(kit.radius.xs)
            .fill(if is_current { c.layer_03 } else { rune::Color::TRANSPARENT })
            .center_y()
            .child(el().w(80.0).child(text(if rp { "T" } else { "F" }).font(ty.body_medium).color(c.text_base)))
            .child(el().w(80.0).child(text(if rq { "T" } else { "F" }).font(ty.body_medium).color(c.text_base)))
            .child(el().w(140.0).child(
                Badge::new(if row_res { "TRUE (1)" } else { "FALSE (0)" })
                    .variant(if row_res { BadgeVariant::Accent } else { BadgeVariant::Neutral })
                    .build(kit)
            ))
            .child(el().grow(1.0).child(text(if is_current { "◀ Baris Terpilih" } else { "" }).font(ty.caption_medium).color(c.text_accent)));

        table_col = table_col.child(row_el);
    }

    let truth_table_card = Card::new()
        .accent(c.icon_base)
        .title("1. Generator & Simulator Tabel Kebenaran", Some(&icons::FLASK))
        .child(
            col().gap(12.0)
                .child(toggles_row)
                .child(op_ctrl)
                .child(result_card)
                .child(table_col)
        )
        .build(kit);

    // 2. Relation Property Tester on {1, 2, 3}
    let mut grid_col = col().gap(6.0);
    for a in 1..=3 {
        let mut grid_row = row().gap(6.0);
        for b in 1..=3 {
            let has_pair = state.rel_pairs.contains(&(a, b));
            let pair_btn = Button::new(format!("({}, {})", a, b))
                .variant(if has_pair { ButtonVariant::Submit } else { ButtonVariant::Neutral })
                .control_size(ControlSize::Small)
                .on_press(move |s: &mut State| {
                    if s.rel_pairs.contains(&(a, b)) {
                        s.rel_pairs.remove(&(a, b));
                    } else {
                        s.rel_pairs.insert((a, b));
                    }
                })
                .build(kit);
            grid_row = grid_row.child(pair_btn);
        }
        grid_col = grid_col.child(grid_row);
    }

    let is_ref = state.is_rel_reflexive();
    let is_sym = state.is_rel_symmetric();
    let is_asym = state.is_rel_antisymmetric();
    let is_tra = state.is_rel_transitive();

    let prop_card = |name: &str, status: bool, true_desc: &str, false_desc: &str| {
        Card::new()
            .accent(if status { c.success.icon } else { c.danger.icon })
            .title(
                format!("{}: {}", name, if status { "MEMENUHI" } else { "TIDAK" }),
                Some(if status { &icons::CIRCLE_CHECK } else { &icons::CIRCLE_X }),
            )
            .description(if status { true_desc } else { false_desc })
            .build(kit)
    };

    let p1 = prop_card("Refleksif", is_ref, "Semua (x, x) ada dalam relasi.", "Belum semua (x, x) masuk.");
    let p2 = prop_card("Simetris", is_sym, "Setiap (a, b) memiliki (b, a).", "Ada (a, b) tanpa (b, a).");
    let p3 = prop_card("Antisimetris", is_asym, "Tidak ada (a, b) dan (b, a) untuk a ≠ b.", "Ada timbal balik (a, b) & (b, a).");
    let p4 = prop_card("Transitif", is_tra, "Lompatan (a, b) & (b, c) menghasilkan (a, c).", "Ada lompatan tak lengkap.");

    let prop_grid = col()
        .w_full()
        .gap(8.0)
        .child(row().w_full().gap(8.0).flex_wrap().child(p1.grow(1.0).shrink()).child(p2.grow(1.0).shrink()))
        .child(row().w_full().gap(8.0).flex_wrap().child(p3.grow(1.0).shrink()).child(p4.grow(1.0).shrink()));

    let relation_content = row()
        .w_full()
        .gap(16.0)
        .flex_wrap()
        .child(
            col().gap(6.0)
                .child(text("Matriks R (Klik untuk toggle):").font(ty.caption).color(c.text_muted))
                .child(grid_col)
        )
        .child(prop_grid.grow(1.0).shrink());

    let relation_card = Card::new()
        .accent(c.icon_base)
        .title("2. Penguji 4 Sifat Relasi Biner Himpunan A = {1, 2, 3}", Some(&icons::GRID_3X3))
        .description("Klik tombol pasangan terurut (a, b) untuk menambah atau menghapus anggota himpunan relasi R.")
        .child(relation_content)
        .build(kit);

    // Responsive Inner Container (Max Width 860px)
    let lab_column = col()
        .w_full()
        .max_w(860.0)
        .gap(22.0)
        .child(header)
        .child(truth_table_card)
        .child(relation_card)
        .pb(56.0);

    // Scrollable Outer Container centered horizontally
    let center_wrapper = col()
        .w_full()
        .items_center()
        .px(24.0)
        .py(20.0)
        .child(lab_column);

    col()
        .w_full()
        .h_full()
        .fill(c.page)
        .child(center_wrapper.scroll(ScrollSpec { stick_bottom: false }).grow(1.0).shrink())
}
