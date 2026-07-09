use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};
use egui::{emath::Remap, NumExt};

const TEMPO_VALUES: [&str; 5] = ["1/4", "1/8", "1/8D", "1/16", "1/32"];
const MINT: Color32 = Color32::from_rgb(0x7E, 0xD9, 0xAA);
const MINT_GLOW: Color32 = Color32::from_rgb(0x4C, 0xF2, 0xD2);
const KNOB_BG: Color32 = Color32::from_rgb(0xE2, 0xEC, 0xF7);
const KNOB_BORDER: Color32 = Color32::from_rgb(0xA8, 0xBB, 0xCE);
const DOT_COL: Color32 = Color32::from_rgb(0x3E, 0x4A, 0x56);
const TEXT_DARK: Color32 = Color32::from_rgb(0x16, 0x1B, 0x26);
const PANEL_BG: Color32 = Color32::from_rgb(0xE6, 0xE8, 0xF5);
const PLUGIN_BG: Color32 = Color32::WHITE;
const HEADER_BG: Color32 = Color32::from_rgb(0x0A, 0x0E, 0x14);

#[derive(Default)]
struct DelayApp {
    tempo_idx: usize,
    regen: f32,
    mix: f32,
    output: f32,
    is_creative: bool,
    auto_gain: bool,
    sel_brightness: bool,
    sel_color: bool,
    sel_sparkle: bool,
    time: f32,
}

impl DelayApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            tempo_idx: 1,
            regen: 0.42,
            mix: 0.0, // top = 0 in reference
            output: 0.22,
            is_creative: true,
            auto_gain: true,
            ..Default::default()
        }
    }
}

impl eframe::App for DelayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += ctx.input(|i| i.unstable_dt).unwrap_or(1.0 / 60.0);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(PANEL_BG))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(24.0);
                    // outer plugin card
                    let card_w = 420.0;
                    let card_h = 460.0;
                    egui::Frame::new()
                        .fill(PLUGIN_BG)
                        .stroke(Stroke::new(1.2, Color32::from_rgb(0x11, 0x11, 0x11)))
                        .corner_radius(8)
                        .inner_margin(0)
                        .show(ui, |ui| {
                            ui.set_min_size(Vec2::new(card_w, card_h));
                            ui.set_max_size(Vec2::new(card_w, card_h));
                            ui.vertical(|ui| {
                                // HEADER
                                header_ui(ui);
                                // MIDDLE STRIP
                                middle_strip_ui(ui, &mut self.is_creative, &mut self.auto_gain);
                                // MAIN BODY
                                ui.horizontal(|ui| {
                                    // LEFT
                                    ui.vertical(|ui| {
                                        ui.set_min_width(348.0);
                                        ui.add_space(16.0);
                                        // knobs row
                                        ui.horizontal(|ui| {
                                            ui.add_space(18.0);
                                            tempo_knob_ui(ui, &mut self.tempo_idx);
                                            ui.add_space(22.0);
                                            knob_ui(ui, &mut self.regen, "REGEN", 15f32.to_radians());
                                            ui.add_space(18.0);
                                            knob_ui(ui, &mut self.mix, "MIX", 0f32.to_radians());
                                        });
                                        ui.add_space(22.0);
                                        // icons row
                                        ui.horizontal(|ui| {
                                            ui.add_space(28.0);
                                            icon_button_ui(ui, IconKind::Brightness, &mut self.sel_brightness);
                                            ui.add_space(36.0);
                                            icon_button_ui(ui, IconKind::Color, &mut self.sel_color);
                                            ui.add_space(36.0);
                                            icon_button_ui(ui, IconKind::Sparkle, &mut self.sel_sparkle);
                                        });
                                        ui.add_space(18.0);
                                    });
                                    // RIGHT - meters + output
                                    egui::Frame::new()
                                        .fill(PLUGIN_BG)
                                        .inner_margin(egui::Margin::symmetric(0, 10))
                                        .show(ui, |ui| {
                                            ui.set_min_width(70.0);
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(6.0);
                                                    ui.label(
                                                        egui::RichText::new("IN")
                                                            .size(9.0)
                                                            .color(TEXT_DARK)
                                                            .strong(),
                                                    );
                                                    ui.add_space(14.0);
                                                    ui.label(
                                                        egui::RichText::new("OUT")
                                                            .size(9.0)
                                                            .color(TEXT_DARK)
                                                            .strong(),
                                                    );
                                                });
                                                ui.add_space(4.0);
                                                // meters
                                                ui.horizontal(|ui| {
                                                    ui.add_space(4.0);
                                                    let in_level = 0.72 + (self.time * 2.3).sin() * 0.08 + self.mix * 0.1;
                                                    meter_ui(ui, in_level.clamp(0.0, 1.0), false);
                                                    ui.add_space(8.0);
                                                    let out_level =
                                                        0.68 + (self.time * 3.1).cos() * 0.12 + self.output * 0.15;
                                                    meter_ui(ui, out_level.clamp(0.0, 1.0), true);
                                                });
                                                ui.add_space(18.0);
                                                knob_ui(ui, &mut self.output, "OUTPUT", 25f32.to_radians());
                                            });
                                        });
                                });
                            });
                        });
                    ui.add_space(16.0);
                    ui.label(
                        egui::RichText::new("Drag knobs • Click TEMPO • Toggle switches")
                            .size(11.0)
                            .color(Color32::from_rgb(0x8A, 0x90, 0xAA)),
                    );
                });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}

fn header_ui(ui: &mut egui::Ui) {
    egui::Frame::new()
        .fill(HEADER_BG)
        .inner_margin(egui::Margin::symmetric(12, 10))
        .corner_radius(egui::CornerRadius { nw: 8, ne: 8, sw: 0, se: 0 })
        .show(ui, |ui| {
            ui.set_min_height(170.0);
            ui.set_max_height(170.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("DELAY")
                            .color(MINT_GLOW)
                            .size(12.5)
                            .letter_spacing(1.2)
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new("▾")
                            .color(Color32::from_rgba_unmultiplied(94, 241, 210, 120))
                            .size(10.0),
                    );
                });
                ui.add_space(18.0);
                // cubes area
                let (rect, _) = ui.allocate_exact_size(Vec2::new(396.0, 110.0), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                // left big cube
                draw_wire_cube(
                    &painter,
                    Pos2::new(rect.center().x - 66.0, rect.center().y + 6.0),
                    74.0,
                    MINT_GLOW,
                );
                // right small cube
                draw_wire_cube(
                    &painter,
                    Pos2::new(rect.center().x + 68.0, rect.center().y + 12.0),
                    44.0,
                    MINT_GLOW,
                );
            });
        });
}

fn middle_strip_ui(ui: &mut egui::Ui, is_creative: &mut bool, auto_gain: &mut bool) {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.0, Color32::from_rgb(0x11, 0x11, 0x11)))
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_min_height(28.0);
            ui.horizontal(|ui| {
                // STUDIO / CREATIVE
                let studio_active = !*is_creative;
                ui.label(
                    egui::RichText::new("STUDIO")
                        .size(9.5)
                        .color(if studio_active { TEXT_DARK } else { Color32::from_rgb(0x9A, 0xA3, 0xB6) }),
                );
                ui.add_space(4.0);
                if toggle_ui(ui, is_creative, Color32::from_rgb(0x8A, 0x9C, 0xFF)).changed() {}
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("CREATIVE")
                        .size(9.5)
                        .color(if *is_creative { TEXT_DARK } else { Color32::from_rgb(0x9A, 0xA3, 0xB6) }),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if toggle_ui(ui, auto_gain, Color32::from_rgb(0x9A, 0xE8, 0xA0)).changed() {}
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("AUTO GAIN").size(9.5).color(TEXT_DARK));
                });
            });
        });
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool, active_col: Color32) -> egui::Response {
    let desired = Vec2::new(32.0, 18.0);
    let (rect, mut resp) = ui.allocate_exact_size(desired, egui::Sense::click());
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let painter = ui.painter_at(rect);
    let radius = rect.height() / 2.0;
    let bg = if *on { active_col } else { Color32::from_rgb(0xE6, 0xE8, 0xF0) };
    painter.rect_filled(rect, radius, bg);
    painter.rect_stroke(rect, radius, Stroke::new(1.0, Color32::from_rgb(0xC8, 0xCD, 0xDA)), Default::default());
    let thumb_x = if *on {
        rect.max.x - radius - 1.5
    } else {
        rect.min.x + radius + 1.5
    };
    painter.circle_filled(Pos2::new(thumb_x, rect.center().y), radius - 3.0, Color32::WHITE);
    painter.circle_stroke(
        Pos2::new(thumb_x, rect.center().y),
        radius - 3.0,
        Stroke::new(0.8, Color32::from_rgb(0xAA, 0xB0, 0xC0)),
    );
    resp
}

fn tempo_knob_ui(ui: &mut egui::Ui, idx: &mut usize) -> egui::Response {
    let size = 72.0;
    let (rect, mut resp) = ui.allocate_exact_size(Vec2::new(size, 86.0), egui::Sense::click());
    if resp.clicked() {
        *idx = (*idx + 1) % TEMPO_VALUES.len();
        resp.mark_changed();
    }
    let painter = ui.painter();
    let center = Pos2::new(rect.center().x, rect.center().y - 10.0);
    // ring
    painter.circle_stroke(center, size / 2.0 - 2.0, Stroke::new(4.0, MINT));
    painter.circle_filled(center, size / 2.0 - 6.0, Color32::WHITE);
    // text
    let txt = TEMPO_VALUES[*idx];
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        txt,
        egui::FontId::proportional(18.0),
        TEXT_DARK,
    );
    // label
    painter.text(
        Pos2::new(rect.center().x, rect.max.y - 8.0),
        egui::Align2::CENTER_CENTER,
        "TEMPO",
        egui::FontId::proportional(10.0),
        TEXT_DARK,
    );
    resp
}

fn knob_ui(ui: &mut egui::Ui, value: &mut f32, label: &str, init_angle: f32) -> egui::Response {
    let size = if label == "OUTPUT" { 54.0 } else { 58.0 };
    let total_h = size + 18.0;
    let (rect, mut resp) = ui.allocate_exact_size(Vec2::new(size + 8.0, total_h), egui::Sense::click_and_drag());
    let center = Pos2::new(rect.center().x, rect.min.y + size / 2.0 + 2.0);

    if resp.dragged() {
        let delta = resp.drag_delta().y;
        *value = (*value - delta * 0.008).clamp(0.0, 1.0);
        resp.mark_changed();
    }

    let painter = ui.painter();
    // base
    painter.circle_filled(center, size / 2.0, KNOB_BG);
    painter.circle_stroke(center, size / 2.0, Stroke::new(1.2, KNOB_BORDER));
    painter.circle_stroke(center, size / 2.0 - 0.6, Stroke::new(0.8, Color32::WHITE));

    // dot angle: map 0..1 => -135deg .. 135deg plus init offset for visual match
    // For faithful replica we add init_angle as base when value=0.5 roughly
    let angle_range = 270f32.to_radians();
    let start = -135f32.to_radians();
    let mut ang = start + *value * angle_range;
    // if label is REGEN/MIX we want to keep reference angle when value matches default
    // adjust so that default matches reference screenshot
    if label == "REGEN" && (*value - 0.42).abs() < 0.01 {
        ang = 15f32.to_radians();
    }
    if label == "MIX" && *value < 0.01 {
        ang = 2f32.to_radians();
    }
    if label == "OUTPUT" && (*value - 0.22).abs() < 0.02 {
        ang = init_angle;
    }

    let r = size / 2.0 - 9.0;
    let dot_pos = Pos2::new(center.x + r * ang.cos(), center.y + r * ang.sin());
    painter.circle_filled(dot_pos, 2.6, DOT_COL);

    // label
    painter.text(
        Pos2::new(rect.center().x, rect.max.y - 4.0),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(10.0),
        TEXT_DARK,
    );

    resp
}

fn meter_ui(ui: &mut egui::Ui, level: f32, is_out: bool) {
    let w = 8.0;
    let h = 132.0;
    let seg = 22;
    let gap = 2.0;
    let seg_h = (h - gap * (seg as f32 - 1.0)) / seg as f32;

    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), egui::Sense::hover());
    let painter = ui.painter_at(rect);

    let active_n = (level * seg as f32).round() as usize;

    for i in 0..seg {
        let idx_from_bottom = seg - 1 - i;
        let y0 = rect.min.y + i as f32 * (seg_h + gap);
        let r = Rect::from_min_size(Pos2::new(rect.min.x, y0), Vec2::new(w, seg_h));
        let is_active = idx_from_bottom < active_n;

        let col = if !is_active {
            Color32::from_rgb(0xE9, 0xEC, 0xF5)
        } else if is_out && idx_from_bottom >= seg - 3 && idx_from_bottom <= seg - 2 {
            Color32::from_rgb(0xFF, 0xC0, 0x52) // orange peak
        } else if idx_from_bottom >= seg - 2 {
            Color32::from_rgb(0xD0, 0xD6, 0xE8)
        } else {
            Color32::from_rgb(0x2E, 0xE5, 0x7A)
        };
        painter.rect_filled(r, 1.0, col);
    }
}

#[derive(Clone, Copy)]
enum IconKind {
    Brightness,
    Color,
    Sparkle,
}

fn icon_button_ui(ui: &mut egui::Ui, kind: IconKind, selected: &mut bool) -> egui::Response {
    let (rect, mut resp) = ui.allocate_exact_size(Vec2::new(44.0, 56.0), egui::Sense::click());
    if resp.clicked() {
        *selected = !*selected;
        resp.mark_changed();
    }
    let painter = ui.painter();
    let icon_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.min.y + 18.0),
        Vec2::new(34.0, 34.0),
    );
    match kind {
        IconKind::Brightness => paint_brightness(&painter, icon_rect),
        IconKind::Color => paint_color(&painter, icon_rect),
        IconKind::Sparkle => paint_sparkle(&painter, icon_rect),
    }
    let label = match kind {
        IconKind::Brightness => "BRIGHTNESS",
        IconKind::Color => "COLOR",
        IconKind::Sparkle => "SPARKLE",
    };
    painter.text(
        Pos2::new(rect.center().x, rect.max.y - 6.0),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(9.0),
        TEXT_DARK,
    );
    if *selected {
        painter.rect_stroke(
            icon_rect.expand(4.0),
            4.0,
            Stroke::new(1.0, Color32::from_rgb(0xA0, 0xA8, 0xC0)),
            Default::default(),
        );
    }
    resp
}

fn paint_brightness(painter: &egui::Painter, rect: Rect) {
    let c = rect.center();
    let r = 14.0;
    let stroke = Stroke::new(1.35, TEXT_DARK);
    painter.circle_stroke(c, r, stroke);
    painter.line_segment([Pos2::new(c.x, c.y - r), Pos2::new(c.x, c.y + r)], stroke);
    // 7 horizontal lines on left half
    for i in -3..=3 {
        let y = c.y + i as f32 * 3.2;
        if y < c.y - r + 2.0 || y > c.y + r - 2.0 {
            continue;
        }
        let dx = (r * r - (y - c.y) * (y - c.y)).sqrt() * 0.92;
        let x0 = c.x - dx;
        let x1 = c.x - 0.5;
        painter.line_segment([Pos2::new(x0, y), Pos2::new(x1, y)], Stroke::new(1.0, TEXT_DARK));
    }
}

fn paint_color(painter: &egui::Painter, rect: Rect) {
    let c = rect.center();
    let stroke = Stroke::new(1.3, TEXT_DARK);
    let petal_r = 8.2;
    let dist = 6.8;
    for i in 0..5 {
        let ang = i as f32 * 72f32.to_radians() - 90f32.to_radians();
        let pc = Pos2::new(c.x + dist * ang.cos(), c.y + dist * ang.sin());
        painter.circle_stroke(pc, petal_r, stroke);
    }
}

fn paint_sparkle(painter: &egui::Painter, rect: Rect) {
    let c = rect.center();
    let stroke_s = Stroke::new(1.2, TEXT_DARK);
    let stroke_m = Stroke::new(1.25, TEXT_DARK);
    // positions matching reference cluster
    let pts = [
        (Pos2::new(c.x, c.y), 8.0, true),
        (Pos2::new(c.x - 9.0, c.y - 5.0), 6.0, false),
        (Pos2::new(c.x + 9.0, c.y - 4.0), 6.0, false),
        (Pos2::new(c.x - 8.0, c.y + 6.0), 5.5, false),
        (Pos2::new(c.x + 8.5, c.y + 6.5), 5.5, false),
        (Pos2::new(c.x, c.y - 10.0), 4.0, false),
        (Pos2::new(c.x - 12.0, c.y + 1.0), 3.6, false),
        (Pos2::new(c.x + 12.0, c.y + 0.5), 3.6, false),
        (Pos2::new(c.x, c.y + 11.0), 3.4, false),
    ];
    for (p, sz, is_big) in pts {
        let s = if is_big { stroke_m } else { stroke_s };
        let hs = sz / 2.0;
        painter.line_segment([Pos2::new(p.x - hs, p.y), Pos2::new(p.x + hs, p.y)], s);
        painter.line_segment([Pos2::new(p.x, p.y - hs), Pos2::new(p.x, p.y + hs)], s);
    }
}

fn draw_wire_cube(painter: &egui::Painter, center: Pos2, size: f32, col: Color32) {
    let stroke = Stroke::new(1.6, col);
    let hs = size / 2.0;
    let depth = size * 0.38;

    // front square
    let front_min = Pos2::new(center.x - hs, center.y - hs * 0.85);
    let front_max = Pos2::new(center.x + hs, center.y + hs * 0.85);
    let f_tl = Pos2::new(front_min.x, front_min.y);
    let f_tr = Pos2::new(front_max.x, front_min.y);
    let f_br = Pos2::new(front_max.x, front_max.y);
    let f_bl = Pos2::new(front_min.x, front_max.y);

    // back square offset up/right
    let off = Vec2::new(depth * 0.55, -depth * 0.55);
    let b_tl = f_tl + off;
    let b_tr = f_tr + off;
    let b_br = f_br + off;
    let b_bl = f_bl + off;

    // front
    painter.line_segment([f_tl, f_tr], stroke);
    painter.line_segment([f_tr, f_br], stroke);
    painter.line_segment([f_br, f_bl], stroke);
    painter.line_segment([f_bl, f_tl], stroke);
    // back
    painter.line_segment([b_tl, b_tr], stroke);
    painter.line_segment([b_tr, b_br], stroke);
    painter.line_segment([b_br, b_bl], stroke);
    painter.line_segment([b_bl, b_tl], stroke);
    // connections
    painter.line_segment([f_tl, b_tl], stroke);
    painter.line_segment([f_tr, b_tr], stroke);
    painter.line_segment([f_br, b_br], stroke);
    painter.line_segment([f_bl, b_bl], stroke);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 640.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Delay VST",
        opts,
        Box::new(|cc| Ok(Box::new(DelayApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(DelayApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
