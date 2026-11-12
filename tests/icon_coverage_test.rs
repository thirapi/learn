use learn::icons;

#[test]
fn test_all_custom_and_theme_icons_render() {
    let icons_to_test = [
        ("SUN", &icons::SUN),
        ("MOON", &icons::MOON),
        ("FLAME", &icons::FLAME),
        ("CLOCK", &icons::CLOCK),
        ("BULB", &icons::BULB),
        ("COMPASS", &icons::COMPASS),
        ("ZAP", &icons::ZAP),
        ("GRID_3X3", &icons::GRID_3X3),
        ("CHECK", &icons::CHECK),
        ("SIDEBAR", &icons::SIDEBAR),
        ("CIRCLE_CHECK", &icons::CIRCLE_CHECK),
        ("CIRCLE_X", &icons::CIRCLE_X),
        ("FLASK", &icons::FLASK),
        ("CHEVRON_LEFT", &icons::CHEVRON_LEFT),
        ("GRADUATION_CAP", &icons::GRADUATION_CAP),
        ("HELP", &icons::HELP),
        ("ARROW_RIGHT", &icons::ARROW_RIGHT),
        ("ARROW_LEFT", &icons::ARROW_LEFT),
        ("BRAIN", &icons::BRAIN),
        ("CHECKLIST", &icons::CHECKLIST),
        ("EDIT", &icons::EDIT),
        ("REFRESH", &icons::REFRESH),
    ];

    for (name, icon) in icons_to_test {
        for sz in [10, 12, 14, 16, 20] {
            let cov = rune::text::icon_coverage(icon, sz);
            let non_zero = cov.iter().filter(|&&c| c > 0).count();
            println!("{name} at {sz}px: non-zero pixels = {non_zero} / {}", sz * sz);
            assert!(non_zero > 0, "Icon {name} produced 0 pixels at {sz}px!");
        }
    }
}
