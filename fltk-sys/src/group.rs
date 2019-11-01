/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Group {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Group_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Group;
}
extern "C" {
    pub fn Fl_Group_set_label(arg1: *mut Fl_Group, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Group_redraw(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_show(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_hide(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_activate(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_deactivate(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_redraw_label(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_resize(
        arg1: *mut Fl_Group,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Group_tooltip(arg1: *mut Fl_Group) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Group_set_tooltip(arg1: *mut Fl_Group, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Group_get_type(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_type(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_color(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_color(arg1: *mut Fl_Group, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_label_color(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_color(arg1: *mut Fl_Group, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_label_font(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_font(arg1: *mut Fl_Group, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_label_size(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_size(arg1: *mut Fl_Group, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_label_type(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_type(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_box(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_box(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_changed(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_changed(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_clear_changed(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_begin(self_: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_end(self_: *mut Fl_Group);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Pack {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Pack_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Pack;
}
extern "C" {
    pub fn Fl_Pack_set_label(arg1: *mut Fl_Pack, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Pack_redraw(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_show(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_hide(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_activate(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_deactivate(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_redraw_label(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_resize(
        arg1: *mut Fl_Pack,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pack_tooltip(arg1: *mut Fl_Pack) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Pack_set_tooltip(arg1: *mut Fl_Pack, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Pack_get_type(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_type(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_color(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_color(arg1: *mut Fl_Pack, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_label_color(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_color(arg1: *mut Fl_Pack, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_label_font(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_font(arg1: *mut Fl_Pack, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_label_size(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_size(arg1: *mut Fl_Pack, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_label_type(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_type(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_box(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_box(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_changed(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_changed(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_clear_changed(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_begin(self_: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_end(self_: *mut Fl_Pack);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Scroll {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Scroll_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Scroll;
}
extern "C" {
    pub fn Fl_Scroll_set_label(arg1: *mut Fl_Scroll, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Scroll_redraw(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_show(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_hide(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_activate(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_deactivate(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_redraw_label(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_resize(
        arg1: *mut Fl_Scroll,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Scroll_tooltip(arg1: *mut Fl_Scroll) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Scroll_set_tooltip(arg1: *mut Fl_Scroll, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Scroll_get_type(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_type(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_color(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_color(arg1: *mut Fl_Scroll, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_label_color(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_color(arg1: *mut Fl_Scroll, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_label_font(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_font(arg1: *mut Fl_Scroll, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_label_size(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_size(arg1: *mut Fl_Scroll, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_label_type(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_type(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_box(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_box(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_changed(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_changed(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_clear_changed(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_begin(self_: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_end(self_: *mut Fl_Scroll);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tabs {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tabs_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tabs;
}
extern "C" {
    pub fn Fl_Tabs_set_label(arg1: *mut Fl_Tabs, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tabs_redraw(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_show(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_hide(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_activate(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_deactivate(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_redraw_label(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_resize(
        arg1: *mut Fl_Tabs,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tabs_tooltip(arg1: *mut Fl_Tabs) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tabs_set_tooltip(arg1: *mut Fl_Tabs, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tabs_get_type(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_type(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_color(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_color(arg1: *mut Fl_Tabs, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_label_color(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_color(arg1: *mut Fl_Tabs, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_label_font(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_font(arg1: *mut Fl_Tabs, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_label_size(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_size(arg1: *mut Fl_Tabs, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_label_type(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_type(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_box(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_box(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_changed(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_changed(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_clear_changed(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_begin(self_: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_end(self_: *mut Fl_Tabs);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tile {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tile_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tile;
}
extern "C" {
    pub fn Fl_Tile_set_label(arg1: *mut Fl_Tile, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tile_redraw(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_show(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_hide(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_activate(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_deactivate(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_redraw_label(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_resize(
        arg1: *mut Fl_Tile,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tile_tooltip(arg1: *mut Fl_Tile) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tile_set_tooltip(arg1: *mut Fl_Tile, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tile_get_type(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_type(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_color(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_color(arg1: *mut Fl_Tile, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_label_color(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_color(arg1: *mut Fl_Tile, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_label_font(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_font(arg1: *mut Fl_Tile, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_label_size(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_size(arg1: *mut Fl_Tile, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_label_type(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_type(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_box(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_box(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_changed(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_changed(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_clear_changed(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_begin(self_: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_end(self_: *mut Fl_Tile);
}
