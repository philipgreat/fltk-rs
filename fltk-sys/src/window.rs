#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Window {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Window_new(
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        title: *const libc::c_char,
    ) -> *mut Fl_Window;

    pub fn Fl_Window_begin(arg1: *mut Fl_Window);

    pub fn Fl_Window_end(arg1: *mut Fl_Window);

    pub fn Fl_Window_show(arg1: *mut Fl_Window);

    pub fn Fl_Window_hide(arg1: *mut Fl_Window);

    pub fn Fl_Window_set_label(
        arg1: *mut Fl_Window,
        title: *const libc::c_char,
    );

    pub fn Fl_Window_redraw(
        arg1: *mut Fl_Window,
    );

    pub fn Fl_Window_activate(arg1: *mut Fl_Window);


    pub fn Fl_Window_deactivate(arg1: *mut Fl_Window);


    pub fn Fl_Window_redraw_label(arg1: *mut Fl_Window);


    pub fn Fl_Window_resize(
        arg1: *mut Fl_Window,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );

    pub fn Fl_Window_set_tooltip(arg1: *mut Fl_Window, txt: *const libc::c_char);

    pub fn Fl_Window_make_modal(arg1: *mut Fl_Window, boolean: u32);
    pub fn Fl_Window_fullscreen(arg1: *mut Fl_Window, boolean: u32);
    pub fn Fl_Window_make_current(arg1: *mut Fl_Window);
}
