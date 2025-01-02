use crate::shapes::Shape;
use skia_safe as skia;

use super::RenderState;

fn render_debug_view(render_state: &mut RenderState) {
    let mut paint = skia::Paint::default();
    paint.set_style(skia::PaintStyle::Stroke);
    paint.set_color(skia::Color::from_argb(255, 255, 0, 255));
    paint.set_stroke_width(1.);

    let mut scaled_rect = render_state.viewbox.area.clone();
    let x = 100. + scaled_rect.x() * 0.2;
    let y = 100. + scaled_rect.y() * 0.2;
    let width = scaled_rect.width() * 0.2;
    let height = scaled_rect.height() * 0.2;
    scaled_rect.set_xywh(x, y, width, height);

    render_state
        .debug_surface
        .canvas()
        .draw_rect(scaled_rect, &paint);
}

pub fn render_debug_element(render_state: &mut RenderState, element: &Shape, intersected: bool) {
    let mut paint = skia::Paint::default();
    paint.set_style(skia::PaintStyle::Stroke);
    paint.set_color(if intersected {
        skia::Color::from_argb(255, 255, 255, 0)
    } else {
        skia::Color::from_argb(255, 0, 255, 255)
    });
    paint.set_stroke_width(1.);

    let mut scaled_rect = element.bounds();
    let x = 100. + scaled_rect.x() * 0.2;
    let y = 100. + scaled_rect.y() * 0.2;
    let width = scaled_rect.width() * 0.2;
    let height = scaled_rect.height() * 0.2;
    scaled_rect.set_xywh(x, y, width, height);

    render_state
        .debug_surface
        .canvas()
        .draw_rect(scaled_rect, &paint);
}

pub fn render(render_state: &mut RenderState) {
    let paint = skia::Paint::default();
    render_debug_view(render_state);
    render_state.debug_surface.draw(
        &mut render_state.final_surface.canvas(),
        (0.0, 0.0),
        skia::SamplingOptions::new(skia::FilterMode::Linear, skia::MipmapMode::Nearest),
        Some(&paint),
    );
}
