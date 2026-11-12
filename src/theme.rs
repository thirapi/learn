use rune::{Color, TextStyle, Shadow};
use rune_kit::Kit;

fn c(s: &str) -> Color {
    rune::color::hex(s).expect("valid hex color")
}

pub struct AppTypography {
    /// Hero/Display title (26px, Bold)
    pub display: TextStyle,
    /// Large heading (22px, Bold)
    pub title_lg: TextStyle,
    /// Medium heading (18px, SemiBold)
    pub title_md: TextStyle,
    /// Small heading / card header (15.5px, SemiBold)
    pub title_sm: TextStyle,
    /// Standard reading body (15px, Regular, 24px line-height)
    pub body: TextStyle,
    /// Body medium (15px, Medium)
    pub body_medium: TextStyle,
    /// Small (13.5px, Regular)
    pub small: TextStyle,
    /// Small medium (13.5px, Medium)
    pub small_medium: TextStyle,
    /// Secondary/Subtle caption (12.5px, Regular)
    pub caption: TextStyle,
    /// Caption medium (12.5px, Medium)
    pub caption_medium: TextStyle,
}

pub fn typography() -> AppTypography {
    AppTypography {
        display: TextStyle::new(26.0, 700.0).lh(34.0),
        title_lg: TextStyle::new(22.0, 700.0).lh(30.0),
        title_md: TextStyle::new(18.0, 650.0).lh(26.0),
        title_sm: TextStyle::new(15.5, 600.0).lh(22.0),
        body: TextStyle::new(15.0, 400.0).lh(24.0),
        body_medium: TextStyle::new(15.0, 500.0).lh(24.0),
        small: TextStyle::new(13.5, 400.0).lh(19.0),
        small_medium: TextStyle::new(13.5, 500.0).lh(19.0),
        caption: TextStyle::new(12.5, 400.0).lh(17.0),
        caption_medium: TextStyle::new(12.5, 500.0).lh(17.0),
    }
}

pub fn soft_card_shadow(dark: bool) -> Shadow {
    if dark {
        Shadow {
            y: 4.0,
            blur: 16.0,
            color: Color::BLACK.alpha(0.4),
            ..Default::default()
        }
    } else {
        Shadow {
            y: 4.0,
            blur: 16.0,
            color: c("#0f172a").alpha(0.06),
            ..Default::default()
        }
    }
}

/// Builds a Kit calibrated strictly to Tailwind CSS v3/v4 design tokens
/// (Slate neutral scale, Indigo primary brand, Emerald/Rose/Amber status colors,
/// and rounded-md / rounded-lg / rounded-xl radii).
pub fn tailwind_kit(dark: bool) -> Kit {
    let mut kit = (*rune_theme_opencode::kit(dark)).clone();

    if dark {
        // Tailwind Slate Dark Palette (slate-950, slate-900, slate-800, etc.)
        kit.c.page = c("#020617");         // slate-950
        kit.c.bg_base = c("#0f172a");      // slate-900
        kit.c.surface_base = c("#0f172a"); // slate-900
        kit.c.layer_01 = c("#0f172a");     // slate-900
        kit.c.layer_02 = c("#1e293b");     // slate-800
        kit.c.layer_03 = c("#334155");     // slate-700
        kit.c.border_base = c("#1e293b");  // border-slate-800
        kit.c.border_muted = c("#1e293b");
        kit.c.border_weak = c("#1e293b");
        kit.c.border_strong = c("#334155"); // border-slate-700

        kit.c.text_base = c("#f8fafc");    // text-slate-50
        kit.c.text_strong = c("#ffffff");
        kit.c.text_muted = c("#94a3b8");   // text-slate-400
        kit.c.text_faint = c("#64748b");   // text-slate-500
        kit.c.text_accent = c("#818cf8");  // text-indigo-400

        // Tailwind Indigo Accent
        kit.c.bg_accent = c("#6366f1");    // bg-indigo-500
        kit.c.icon_base = c("#818cf8");    // text-indigo-400
        kit.c.icon_accent = c("#818cf8");

        // Tailwind Status Colors
        kit.c.success.icon = c("#10b981"); // emerald-500
        kit.c.success.fg = c("#34d399");
        kit.c.danger.icon = c("#f43f5e");  // rose-500
        kit.c.danger.fg = c("#fb7185");
        kit.c.warning.icon = c("#f59e0b"); // amber-500
        kit.c.warning.fg = c("#fbbf24");
        kit.c.info.icon = c("#38bdf8");    // sky-400
    } else {
        // Tailwind Slate Light Palette (slate-50, white, slate-100, slate-200)
        kit.c.page = c("#f8fafc");         // bg-slate-50
        kit.c.bg_base = c("#ffffff");      // bg-white
        kit.c.surface_base = c("#ffffff"); // bg-white
        kit.c.layer_01 = c("#ffffff");     // bg-white
        kit.c.layer_02 = c("#f1f5f9");     // bg-slate-100
        kit.c.layer_03 = c("#e2e8f0");     // bg-slate-200
        kit.c.border_base = c("#e2e8f0");  // border-slate-200
        kit.c.border_muted = c("#f1f5f9"); // border-slate-100
        kit.c.border_weak = c("#e2e8f0");  // border-slate-200
        kit.c.border_strong = c("#cbd5e1");// border-slate-300

        kit.c.text_base = c("#0f172a");    // text-slate-900
        kit.c.text_strong = c("#020617");  // text-slate-950
        kit.c.text_muted = c("#64748b");   // text-slate-500
        kit.c.text_faint = c("#94a3b8");   // text-slate-400
        kit.c.text_accent = c("#4f46e5");  // text-indigo-600

        // Tailwind Indigo Accent
        kit.c.bg_accent = c("#4f46e5");    // bg-indigo-600
        kit.c.icon_base = c("#4f46e5");    // text-indigo-600
        kit.c.icon_accent = c("#4f46e5");

        // Tailwind Status Colors
        kit.c.success.icon = c("#10b981"); // emerald-500
        kit.c.success.fg = c("#059669");   // emerald-600
        kit.c.danger.icon = c("#ef4444");  // red-500
        kit.c.danger.fg = c("#dc2626");    // red-600
        kit.c.warning.icon = c("#f59e0b"); // amber-500
        kit.c.warning.fg = c("#d97706");   // amber-600
        kit.c.info.icon = c("#0284c7");    // sky-600
    }

    // Upgraded Typography in Kit for modern reading scale
    kit.ty.body = TextStyle::new(15.0, 400.0).lh(24.0);
    kit.ty.body_medium = TextStyle::new(15.0, 500.0).lh(24.0);
    kit.ty.ui = TextStyle::new(14.0, 450.0).lh(20.0);
    kit.ty.ui_medium = TextStyle::new(14.0, 550.0).lh(20.0);

    // Tailwind Radii Tokens (rounded-md, rounded-lg, rounded-xl, rounded-2xl)
    kit.radius.xs = 4.0;
    kit.radius.sm = 8.0;  // rounded-md
    kit.radius.md = 10.0; // rounded-lg
    kit.radius.lg = 14.0; // rounded-xl
    kit.radius.xl = 18.0; // rounded-2xl

    kit
}
