/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget_Tracker {
    _unused: [u8; 0],
}
pub type Fl_Awake_Handler = ::core::option::Option<unsafe extern "C" fn(data: *mut cty::c_void)>;
extern "C" {
    pub fn Fl_run() -> cty::c_int;
}
extern "C" {
    pub fn Fl_check() -> cty::c_int;
}
extern "C" {
    pub fn Fl_ready() -> cty::c_int;
}
extern "C" {
    pub fn Fl_release();
}
extern "C" {
    pub fn Fl_reload_scheme() -> cty::c_int;
}
extern "C" {
    pub fn Fl_menu_linespacing() -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_menu_linespacing(H: cty::c_int);
}
extern "C" {
    pub fn Fl_lock() -> cty::c_int;
}
extern "C" {
    pub fn Fl_unlock();
}
extern "C" {
    pub fn Fl_awake_callback(handler: Fl_Awake_Handler, data: *mut cty::c_void) -> cty::c_int;
}
extern "C" {
    pub fn Fl_awake();
}
extern "C" {
    pub fn Fl_set_scrollbar_size(arg1: cty::c_int);
}
extern "C" {
    pub fn Fl_scrollbar_size() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_key() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_original_key() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_key_down(arg1: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_text() -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_event_button() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_clicks() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_x() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_y() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_x_root() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_y_root() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_dx() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_dy() -> cty::c_int;
}
extern "C" {
    pub fn Fl_get_mouse(arg1: *mut cty::c_int, arg2: *mut cty::c_int);
}
extern "C" {
    pub fn Fl_event_is_click() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_length() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_state() -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_x() -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_y() -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_h() -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_w() -> cty::c_int;
}
extern "C" {
    pub fn Fl_compose(del: *mut cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_compose_reset();
}
extern "C" {
    pub fn Fl_compose_state() -> cty::c_int;
}
extern "C" {
    pub fn Fl_copy(stuff: *const cty::c_char, len: cty::c_int, destination: cty::c_int);
}
extern "C" {
    pub fn Fl_paste_text(arg1: *mut Fl_Widget, src: cty::c_int);
}
extern "C" {
    pub fn Fl_paste_image(widget: *mut Fl_Widget, src: cty::c_int);
}
extern "C" {
    pub fn Fl_set_scheme(scheme: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn Fl_scheme() -> cty::c_int;
}
extern "C" {
    pub fn Fl_scheme_string() -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_visible_focus() -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_visible_focus(arg1: cty::c_int);
}
extern "C" {
    pub fn Fl_set_box_type(arg1: cty::c_int, arg2: cty::c_int);
}
extern "C" {
    pub fn Fl_box_shadow_width() -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_box_shadow_width(W: cty::c_int);
}
extern "C" {
    pub fn Fl_box_border_radius_max() -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_box_border_radius_max(R: cty::c_int);
}
extern "C" {
    pub fn Fl_get_rgb_color(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_set_color(c: cty::c_uint, r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_set_color_with_alpha(
        c: cty::c_uint,
        r: cty::c_uchar,
        g: cty::c_uchar,
        b: cty::c_uchar,
        a: cty::c_uchar,
    );
}
extern "C" {
    pub fn Fl_get_font(idx: cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_get_font_name(idx: cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_get_font_name2(idx: cty::c_int, attributes: *mut cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_get_font_sizes(font: cty::c_int, sizep: *mut *mut cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_fonts(c: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_font(arg1: cty::c_int, arg2: cty::c_int);
}
extern "C" {
    pub fn Fl_set_font2(arg1: cty::c_int, arg2: *const cty::c_char);
}
extern "C" {
    pub fn Fl_set_font_size(arg1: cty::c_int);
}
extern "C" {
    pub fn Fl_font_size() -> cty::c_int;
}
extern "C" {
    pub fn Fl_add_handler(
        ev_handler: ::core::option::Option<unsafe extern "C" fn(ev: cty::c_int) -> cty::c_int>,
    );
}
extern "C" {
    pub fn Fl_awake_msg(msg: *mut cty::c_void);
}
extern "C" {
    pub fn Fl_thread_msg() -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_wait() -> cty::c_int;
}
extern "C" {
    pub fn Fl_wait_for(arg1: f64) -> f64;
}
extern "C" {
    pub fn Fl_add_timeout(
        t: f64,
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_repeat_timeout(
        t: f64,
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_timeout(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_has_timeout(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_dnd() -> cty::c_int;
}
extern "C" {
    pub fn Fl_grab() -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_set_grab(arg1: *mut cty::c_void);
}
extern "C" {
    pub fn Fl_first_window() -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_next_window(arg1: *const cty::c_void) -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_modal() -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_should_program_quit() -> cty::c_int;
}
extern "C" {
    pub fn Fl_program_should_quit(flag: cty::c_int);
}
extern "C" {
    pub fn Fl_event_inside(
        arg1: cty::c_int,
        arg2: cty::c_int,
        arg3: cty::c_int,
        arg4: cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_belowmouse() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_delete_widget(w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_Tracker_new(w: *mut Fl_Widget) -> *mut Fl_Widget_Tracker;
}
extern "C" {
    pub fn Fl_Widget_Tracker_deleted(self_: *mut Fl_Widget_Tracker) -> cty::c_int;
}
extern "C" {
    pub fn Fl_Widget_Tracker_delete(self_: *mut Fl_Widget_Tracker);
}
extern "C" {
    pub fn Fl_init_all();
}
extern "C" {
    pub fn Fl_redraw();
}
extern "C" {
    pub fn Fl_event_shift() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_ctrl() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_command() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_alt() -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_damage(flag: cty::c_int);
}
extern "C" {
    pub fn Fl_damage() -> cty::c_int;
}
extern "C" {
    pub fn Fl_visual(arg1: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_own_colormap();
}
extern "C" {
    pub fn Fl_pushed() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_focus() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_set_focus(arg1: *mut cty::c_void);
}
extern "C" {
    pub fn Fl_version() -> f64;
}
extern "C" {
    pub fn Fl_api_version() -> cty::c_int;
}
extern "C" {
    pub fn Fl_abi_version() -> cty::c_int;
}
extern "C" {
    pub fn Fl_load_font(path: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn Fl_unload_font(path: *const cty::c_char);
}
extern "C" {
    pub fn Fl_foreground(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_background(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_background2(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_selection_color(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_inactive_color(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_get_system_colors();
}
extern "C" {
    pub fn Fl_handle(arg1: cty::c_int, arg2: *mut cty::c_void) -> cty::c_int;
}
extern "C" {
    pub fn Fl_handle_(arg1: cty::c_int, arg2: *mut cty::c_void) -> cty::c_int;
}
extern "C" {
    pub fn Fl_add_idle(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_has_idle(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_remove_idle(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>,
        arg2: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_flush();
}
extern "C" {
    pub fn Fl_set_screen_scale(n: cty::c_int, val: f32);
}
extern "C" {
    pub fn Fl_screen_scale(n: cty::c_int) -> f32;
}
extern "C" {
    pub fn Fl_screen_scaling_supported() -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_count() -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_num(x: cty::c_int, y: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_num_inside(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_screen_xywh(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
        n: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_xywh_at(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
        mx: cty::c_int,
        my: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_xywh_inside(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
        mx: cty::c_int,
        my: cty::c_int,
        mw: cty::c_int,
        mh: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_xywh_mouse(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_dpi(h: *mut f32, v: *mut f32, n: cty::c_int);
}
extern "C" {
    pub fn Fl_screen_work_area(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
        n: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_work_area_at(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
        mx: cty::c_int,
        my: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_work_area_mouse(
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
    );
}
extern "C" {
    pub fn Fl_keyboard_screen_scaling(value: cty::c_int);
}
extern "C" {
    pub fn Fl_open_display();
}
extern "C" {
    pub fn Fl_close_display();
}
extern "C" {
    pub fn Fl_box_dx(boxtype: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_box_dy(boxtype: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_box_dw(boxtype: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_box_dh(boxtype: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_mac_os_version() -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_clipboard() -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_event_clipboard_type() -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_clipboard_contains(type_: *const cty::c_char) -> cty::c_int;
}
extern "C" {
    pub fn Fl_event_dispatch(
        cb: ::core::option::Option<
            unsafe extern "C" fn(event: cty::c_int, arg1: *mut cty::c_void) -> cty::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_inactive(c: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_lighter(c: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_darker(c: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_set_box_type_cb(
        arg1: cty::c_int,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: cty::c_int,
                arg2: cty::c_int,
                arg3: cty::c_int,
                arg4: cty::c_int,
                arg5: cty::c_uint,
            ),
        >,
        arg2: cty::c_int,
        arg3: cty::c_int,
        arg4: cty::c_int,
        arg5: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_box_active() -> cty::c_int;
}
extern "C" {
    pub fn Fl_gray_ramp(i: cty::c_int) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_color_average(arg1: cty::c_uint, arg2: cty::c_uint, f: f32) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_contrast(c1: cty::c_uint, c2: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_rgb_color(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_rgb_color2(g: cty::c_uchar) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_cmap(c: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_box_color(c: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_set_box_color(c: cty::c_uint);
}
extern "C" {
    pub fn Fl_add_system_handler(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut cty::c_void, arg2: *mut cty::c_void) -> cty::c_int,
        >,
        arg2: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_system_handler(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut cty::c_void, arg2: *mut cty::c_void) -> cty::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_gl_visual(mode: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_add_clipboard_notify(
        cb: ::core::option::Option<
            unsafe extern "C" fn(source: cty::c_int, data: *mut cty::c_void),
        >,
        data: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_clipboard_notify(
        cb: ::core::option::Option<
            unsafe extern "C" fn(source: cty::c_int, data: *mut cty::c_void),
        >,
    );
}
extern "C" {
    pub fn Fl_open_callback(
        cb: ::core::option::Option<unsafe extern "C" fn(arg1: *const cty::c_char)>,
    );
}
extern "C" {
    pub fn Fl_disable_wayland();
}
extern "C" {
    pub fn Fl_Widget_Tracker_widget(t: *mut Fl_Widget_Tracker) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Widget_Tracker_exists(t: *mut Fl_Widget_Tracker) -> cty::c_int;
}
extern "C" {
    pub fn Fl_get_color_rgb(
        col: cty::c_uint,
        r: *mut cty::c_uchar,
        g: *mut cty::c_uchar,
        b: *mut cty::c_uchar,
    );
}
extern "C" {
    pub fn Fl_callback_reason() -> cty::c_int;
}
