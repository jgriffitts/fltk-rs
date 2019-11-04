/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Fl_Widget_callback(arg1: *mut Fl_Widget, cb: Fl_Callback);
}
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Menu_Item {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Menu_Bar {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Menu_Bar_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Bar;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label(arg1: *mut Fl_Menu_Bar, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Bar_redraw(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_show(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_hide(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_activate(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_deactivate(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_redraw_label(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_resize(
        arg1: *mut Fl_Menu_Bar,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_tooltip(arg1: *mut Fl_Menu_Bar) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_tooltip(arg1: *mut Fl_Menu_Bar, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Bar_get_type(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_type(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_color(arg1: *mut Fl_Menu_Bar, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_color(arg1: *mut Fl_Menu_Bar, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_font(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_font(arg1: *mut Fl_Menu_Bar, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_size(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_size(arg1: *mut Fl_Menu_Bar, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_label_type(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_label_type(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_box(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_box(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_changed(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_changed(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_clear_changed(arg1: *mut Fl_Menu_Bar);
}
extern "C" {
    pub fn Fl_Menu_Bar_align(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_align(arg1: *mut Fl_Menu_Bar, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_add(
        arg1: *mut Fl_Menu_Bar,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Bar_get_item(
        arg1: *mut Fl_Menu_Bar,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Bar_text_font(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_text_font(arg1: *mut Fl_Menu_Bar, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_text_size(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_text_size(arg1: *mut Fl_Menu_Bar, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Bar_text_color(arg1: *mut Fl_Menu_Bar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Bar_set_text_color(arg1: *mut Fl_Menu_Bar, c: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Menu_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Menu_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Button;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label(
        arg1: *mut Fl_Menu_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_redraw(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_show(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_hide(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_activate(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_deactivate(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_redraw_label(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_resize(
        arg1: *mut Fl_Menu_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_tooltip(arg1: *mut Fl_Menu_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Button_set_tooltip(
        arg1: *mut Fl_Menu_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_get_type(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_type(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_color(arg1: *mut Fl_Menu_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_label_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_color(arg1: *mut Fl_Menu_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_label_font(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_font(arg1: *mut Fl_Menu_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_label_size(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_size(arg1: *mut Fl_Menu_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_label_type(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_label_type(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_box(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_box(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_changed(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_changed(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_clear_changed(arg1: *mut Fl_Menu_Button);
}
extern "C" {
    pub fn Fl_Menu_Button_align(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_align(arg1: *mut Fl_Menu_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_add(
        arg1: *mut Fl_Menu_Button,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Menu_Button_get_item(
        arg1: *mut Fl_Menu_Button,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Menu_Button_text_font(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_text_font(arg1: *mut Fl_Menu_Button, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_text_size(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_text_size(arg1: *mut Fl_Menu_Button, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Button_text_color(arg1: *mut Fl_Menu_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Button_set_text_color(arg1: *mut Fl_Menu_Button, c: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Choice {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Choice_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Choice;
}
extern "C" {
    pub fn Fl_Choice_set_label(arg1: *mut Fl_Choice, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Choice_redraw(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_show(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_hide(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_activate(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_deactivate(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_redraw_label(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_resize(
        arg1: *mut Fl_Choice,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_tooltip(arg1: *mut Fl_Choice) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Choice_set_tooltip(arg1: *mut Fl_Choice, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Choice_get_type(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_type(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_color(arg1: *mut Fl_Choice, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_label_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_color(arg1: *mut Fl_Choice, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_label_font(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_font(arg1: *mut Fl_Choice, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_label_size(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_size(arg1: *mut Fl_Choice, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_label_type(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_label_type(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_box(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_box(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_changed(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_changed(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_clear_changed(arg1: *mut Fl_Choice);
}
extern "C" {
    pub fn Fl_Choice_align(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_align(arg1: *mut Fl_Choice, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_add(
        arg1: *mut Fl_Choice,
        name: *const ::std::os::raw::c_char,
        shortcut: ::std::os::raw::c_int,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Choice_get_item(
        arg1: *mut Fl_Choice,
        name: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Menu_Item;
}
extern "C" {
    pub fn Fl_Choice_text_font(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_text_font(arg1: *mut Fl_Choice, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_text_size(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_text_size(arg1: *mut Fl_Choice, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Choice_text_color(arg1: *mut Fl_Choice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Choice_set_text_color(arg1: *mut Fl_Choice, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_label(arg1: *mut Fl_Menu_Item) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label(arg1: *mut Fl_Menu_Item, a: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Menu_Item_label_type(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_type(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_label_color(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_color(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_label_font(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_font(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_label_size(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set_label_size(arg1: *mut Fl_Menu_Item, a: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Menu_Item_value(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_set(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_clear(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_visible(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_show(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_hide(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_active(arg1: *mut Fl_Menu_Item) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Menu_Item_activate(arg1: *mut Fl_Menu_Item);
}
extern "C" {
    pub fn Fl_Menu_Item_deactivate(arg1: *mut Fl_Menu_Item);
}
