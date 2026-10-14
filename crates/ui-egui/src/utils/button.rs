// import egui
use egui::{Color32, Response, Rounding, Sense, TextStyle, Ui, Vec2};

// define primary button
pub fn primary_button(ui: &mut Ui, text: impl Into<String>) -> Response {
    let text_string = text.into();
    let id = ui.make_persistent_id(&text_string);

    // configuration padding
    let padding = Vec2::new(16.0, 8.0);
    let font_id = TextStyle::Button.resolve(ui.style());

    // use white text
    let galley = ui.fonts(|f| f.layout_no_wrap(text_string, font_id, Color32::WHITE));
    let desired_size = galley.size() + padding * 2.0;

    // use area for interaction detection
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        // use soft animation
        let hover_factor = ui.ctx().animate_bool(id.with("hover"), response.hovered());
        let click_factor = ui.ctx().animate_bool(id.with("click"), response.is_pointer_button_down_on());

        // use fade effect
        let base_bg = Color32::from_rgb(0, 0, 0); // hex code for black
        let hover_bg = Color32::from_rgb(35, 38, 46);
        let click_bg = Color32::from_rgb(50, 56, 68);

        let current_bg = lerp_color(base_bg, hover_bg, hover_factor);
        let final_bg = lerp_color(current_bg, click_bg, click_factor);

        // use rounded corner
        let animated_rect = rect.expand(hover_factor * 1.0);
        ui.painter().rect_filled(animated_rect, Rounding::same(8.0), final_bg);

        // 6. Drawing Teks
        let text_pos = rect.min + padding;
        ui.painter().galley(text_pos, galley, Color32::WHITE);
    }

    response
}

// function helper for interpolate linear color
fn lerp_color(c1: Color32, c2: Color32, t: f32) -> Color32 {
    Color32::from_rgba_premultiplied(
        (c1.r() as f32 + (c2.r() as f32 - c1.r() as f32) * t) as u8,
        (c1.g() as f32 + (c2.g() as f32 - c1.g() as f32) * t) as u8,
        (c1.b() as f32 + (c2.b() as f32 - c1.b() as f32) * t) as u8,
        (c1.a() as f32 + (c2.a() as f32 - c1.a() as f32) * t) as u8,
    )
}
