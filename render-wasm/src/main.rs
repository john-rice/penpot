mod debug;
mod images;
mod math;
mod render;
mod shapes;
mod state;
mod utils;
mod view;

use skia_safe as skia;

use crate::state::State;
use crate::utils::uuid_from_u32_quartet;

static mut STATE: Option<Box<State>> = None;

extern "C" {
    fn emscripten_GetProcAddress(
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_void;
}

fn init_gl() {
    unsafe {
        gl::load_with(|addr| {
            let addr = std::ffi::CString::new(addr).unwrap();
            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
        });
    }
}

/// This is called from JS after the WebGL context has been created.
#[no_mangle]
pub extern "C" fn init(width: i32, height: i32) {
    let state_box = Box::new(State::new(width, height, 2048));
    unsafe {
        STATE = Some(state_box);
    }
}

#[no_mangle]
pub extern "C" fn set_render_options(debug: u32, dpr: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    let render_state = state.render_state();

    render_state.set_debug_flags(debug);
    render_state.set_dpr(dpr);
}

#[no_mangle]
pub unsafe extern "C" fn render() {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.render_all(true);
}

#[no_mangle]
pub unsafe extern "C" fn render_without_cache() {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.render_all(false);
}

#[no_mangle]
pub unsafe extern "C" fn navigate() {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.navigate();
}

#[no_mangle]
pub extern "C" fn reset_canvas() {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.render_state().reset_canvas();
}

#[no_mangle]
pub extern "C" fn resize_viewbox(width: i32, height: i32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.resize(width, height);
}

#[no_mangle]
pub extern "C" fn set_view(zoom: f32, x: f32, y: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.render_state().viewbox.set_all(zoom, x, y);
}

#[no_mangle]
pub extern "C" fn set_view_zoom(zoom: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.render_state().viewbox.set_zoom(zoom);
}

#[no_mangle]
pub extern "C" fn set_view_xy(x: f32, y: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    state.render_state().viewbox.set_pan_xy(x, y);
}

#[no_mangle]
pub extern "C" fn use_shape(a: u32, b: u32, c: u32, d: u32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    let id = uuid_from_u32_quartet(a, b, c, d);
    state.use_shape(id);
}

#[no_mangle]
pub unsafe extern "C" fn set_shape_selrect(left: f32, top: f32, right: f32, bottom: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");

    if let Some(shape) = state.current_shape() {
        shape.selrect.set_ltrb(left, top, right, bottom);
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_shape_rotation(rotation: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.rotation = rotation;
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_shape_transform(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.transform.a = a;
        shape.transform.b = b;
        shape.transform.c = c;
        shape.transform.d = d;
        shape.transform.e = e;
        shape.transform.f = f;
    }
}

#[no_mangle]
pub extern "C" fn add_shape_child(a: u32, b: u32, c: u32, d: u32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    let id = uuid_from_u32_quartet(a, b, c, d);
    if let Some(shape) = state.current_shape() {
        shape.children.push(id);
    }
}

#[no_mangle]
pub extern "C" fn clear_shape_children() {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.children.clear();
    }
}

#[no_mangle]
pub extern "C" fn add_shape_solid_fill(r: u8, g: u8, b: u8, a: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        let alpha: u8 = (a * 0xff as f32).floor() as u8;
        let color = skia::Color::from_argb(alpha, r, g, b);
        shape.add_fill(shapes::Fill::from(color));
    }
}

#[no_mangle]
pub extern "C" fn add_shape_linear_fill(
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    opacity: f32,
) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.add_fill(shapes::Fill::new_linear_gradient(
            (start_x, start_y),
            (end_x, end_y),
            opacity,
        ))
    }
}

#[no_mangle]
pub extern "C" fn add_shape_fill_stop(r: u8, g: u8, b: u8, a: f32, offset: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        let alpha: u8 = (a * 0xff as f32).floor() as u8;
        let color = skia::Color::from_argb(alpha, r, g, b);
        shape
            .add_gradient_stop(color, offset)
            .expect("got no fill or an invalid one");
    }
}

#[no_mangle]
pub extern "C" fn clear_shape_fills() {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.clear_fills();
    }
}

#[no_mangle]
pub extern "C" fn set_shape_blend_mode(mode: i32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.set_blend_mode(shapes::BlendMode::from(mode));
    }
}

#[no_mangle]
pub extern "C" fn set_shape_opacity(opacity: f32) {
    let state = unsafe { STATE.as_mut() }.expect("got an invalid state pointer");
    if let Some(shape) = state.current_shape() {
        shape.opacity = opacity;
    }
}

fn main() {
    init_gl();
}
