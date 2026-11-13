use rune::{El, col, el, row, text, ScrollSpec};
use rune_kit::{Badge, BadgeVariant, Card, Kit, SegmentedControl, Switch};
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

    let header = col().w_full().gap(8.0).pb(8.0).child(row().child(lab_badge)).child(title).child(subtitle);

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
        &["AND ∧", "OR ∨", "IF-THEN →", "IFF ↔", "XOR"],
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

    // Dynamic 4-row Truth Table (fixed column widths, current row highlighted)
    let cell = |w: f32, e: El| el().w(w).child(e);
    let table_header = row()
        .w_full()
        .px(12.0)
        .py(8.0)
        .radius(kit.radius.sm)
        .fill(c.layer_02)
        .child(cell(72.0, text("p").font(ty.caption_medium).color(c.text_muted)))
        .child(cell(72.0, text("q").font(ty.caption_medium).color(c.text_muted)))
        .child(cell(140.0, text(formula_name).font(ty.caption_medium).color(c.text_muted)));

    let rows_data = [(true, true), (true, false), (false, true), (false, false)];

    let mut table_col = col().w_full().gap(2.0).child(table_header);

    for (rp, rq) in rows_data {
        let is_current = (rp == p) && (rq == q);
        let row_res = match state.truth_op {
            0 => rp && rq,
            1 => rp || rq,
            2 => !rp || rq,
            3 => rp == rq,
            _ => rp ^ rq,
        };
        let tf = |v: bool| if v { "T" } else { "F" };
        let val_font = if is_current { ty.body_medium } else { ty.body };

        let mut row_el = row()
            .w_full()
            .px(12.0)
            .py(6.0)
            .radius(kit.radius.sm)
            .fill(if is_current { c.bg_accent.alpha(0.10) } else { rune::Color::TRANSPARENT })
            .border(1.0, if is_current { c.bg_accent.alpha(0.35) } else { rune::Color::TRANSPARENT })
            .center_y()
            .child(cell(72.0, text(tf(rp)).font(val_font).color(c.text_base)))
            .child(cell(72.0, text(tf(rq)).font(val_font).color(c.text_base)))
            .child(cell(
                140.0,
                Badge::new(if row_res { "TRUE (1)" } else { "FALSE (0)" })
                    .variant(if row_res { BadgeVariant::Accent } else { BadgeVariant::Neutral })
                    .build(kit),
            ));
        if is_current {
            row_el = row_el.child(text("Input saat ini").font(ty.caption_medium).color(c.text_accent));
        }

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

    // 2. Relation Property Tester on {1, 2, 3}: a clickable boolean matrix M_R
    const CELL: f32 = 44.0;
    let head = |t: String| {
        el().size(CELL, 28.0).center().child(text(t).font(ty.caption_medium).color(c.text_muted))
    };
    let mut header_row = row().gap(6.0).child(el().size(28.0, 28.0));
    for b in 1..=3 {
        header_row = header_row.child(head(format!("{}", b)));
    }
    let mut grid_col = col().gap(6.0).child(header_row);
    for a in 1..=3 {
        let mut grid_row = row()
            .gap(6.0)
            .center_y()
            .child(el().size(28.0, CELL).center().child(text(format!("{}", a)).font(ty.caption_medium).color(c.text_muted)));
        for b in 1..=3 {
            let has_pair = state.rel_pairs.contains(&(a, b));
            let pair_cell = el()
                .size(CELL, CELL)
                .radius(kit.radius.sm)
                .fill(if has_pair { c.bg_accent } else { c.layer_01 })
                .border(1.0, if has_pair { c.bg_accent } else { c.border_strong })
                .hover(if has_pair { c.bg_accent.alpha(0.85) } else { c.layer_02 })
                .center()
                .role("button")
                .label(format!("Pasangan ({}, {})", a, b))
                .child(
                    text(if has_pair { "1" } else { "0" })
                        .font(ty.body_medium)
                        .color(if has_pair { rune::Color::WHITE } else { c.text_faint }),
                )
                .on_press(move |s: &mut State| {
                    if !s.rel_pairs.remove(&(a, b)) {
                        s.rel_pairs.insert((a, b));
                    }
                });
            grid_row = grid_row.child(pair_cell);
        }
        grid_col = grid_col.child(grid_row);
    }

    let mut pairs: Vec<_> = state.rel_pairs.iter().copied().collect();
    pairs.sort();
    let pairs_text = if pairs.is_empty() {
        "R = ∅".to_string()
    } else {
        format!(
            "R = {{{}}}",
            pairs.iter().map(|(a, b)| format!("({}, {})", a, b)).collect::<Vec<_>>().join(", ")
        )
    };

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

    let matrix_box = col()
        .gap(8.0)
        .child(text("Matriks M_R (baris a, kolom b) — klik sel untuk mengubah").font(ty.caption).color(c.text_muted))
        .child(grid_col)
        .child(text(pairs_text).font(ty.small).color(c.text_base).wrap().w_full());

    let relation_content = col()
        .w_full()
        .gap(16.0)
        .child(matrix_box)
        .child(prop_grid);

    let relation_card = Card::new()
        .accent(c.icon_base)
        .title("2. Penguji 4 Sifat Relasi Biner Himpunan A = {1, 2, 3}", Some(&icons::GRID_3X3))
        .child(relation_content)
        .build(kit);

    // Responsive Inner Container (Max Width 860px)
    let lab_column = col()
        .w_full()
        .max_w(720.0)
        .gap(22.0)
        .child(header)
        .child(truth_table_card)
        .child(relation_card)
        .pb(56.0);

    // Scrollable Outer Container centered horizontally
    let center_wrapper = col()
        .w_full()
        .items_center()
        .px(32.0)
        .py(28.0)
        .child(lab_column);

    col()
        .w_full()
        .h_full()
        .fill(c.page)
        .child(center_wrapper.scroll(ScrollSpec { stick_bottom: false }).grow(1.0).shrink())
}
