// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/* automatically generated by rust-bindgen */

use std::libc::*;
use std::local_data::{local_data_get, local_data_set};
use std::ptr::{null, to_unsafe_ptr};
use std::str::NullTerminatedStr;
use std::cast;
use std::cast::transmute;
use std::vec::raw::to_ptr;

/* FIXME: global variable glutStrokeRoman */

/* FIXME: global variable glutStrokeMonoRoman */

/* FIXME: global variable glutBitmap9By15 */

/* FIXME: global variable glutBitmap8By13 */

/* FIXME: global variable glutBitmapTimesRoman10 */

/* FIXME: global variable glutBitmapTimesRoman24 */

/* FIXME: global variable glutBitmapHelvetica10 */

/* FIXME: global variable glutBitmapHelvetica12 */

/* FIXME: global variable glutBitmapHelvetica18 */

pub type GLenum = i32;
pub type GLint = i32;
pub type GLfloat = f32;
pub type GLdouble = f64;

pub struct Window(c_int);

pub static DOUBLE: c_uint = 2 as c_uint;

pub static ACTIVE_SHIFT: c_int = 1;
pub static ACTIVE_CTRL: c_int = 2;
pub static ACTIVE_ALT: c_int = 4;

// mouse buttons
pub static LEFT_BUTTON: c_int = 0;
pub static MIDDLE_BUTTON: c_int = 1;
pub static RIGHT_BUTTON: c_int = 2;

// mouse button callback state
pub static MOUSE_DOWN: c_int = 0;
pub static MOUSE_UP: c_int = 1;

static WINDOW_WIDTH: GLenum = 102;
static WINDOW_HEIGHT: GLenum = 103;

#[cfg(target_os="linux")]
pub static HAVE_PRECISE_MOUSE_WHEEL: bool = false;
#[cfg(target_os="macos")]
pub static HAVE_PRECISE_MOUSE_WHEEL: bool = true;

pub enum State {
    WindowWidth,
    WindowHeight
}

// XXX This function looks nonsensical after some forward porting
pub fn destroy<T>(_value: &[T]) {
    // let it drop
}

pub fn init() { 
    unsafe {
        let argc = 0 as c_int;
        let glut = ~"glut";
        let command = glut.as_bytes_with_null();
        let argv: (*u8, *u8) = (to_ptr(command), null());
        let argv_p = transmute(to_unsafe_ptr(&argv));

        glutInit(to_unsafe_ptr(&argc), argv_p);

        destroy(command);
    }
}

pub fn create_window(name: ~str) -> Window {
    unsafe {
        let bytes = name.as_bytes_with_null();
        return Window(glutCreateWindow(to_ptr(bytes) as *c_char));
    }
}

pub fn destroy_window(window: Window) {
    unsafe {
        glutDestroyWindow(*window);
    }
}

pub fn set_window(window: Window) {
    unsafe {
        glutSetWindow(*window);
    }
}

pub fn set_window_title(_window: Window, title: &str) {
    unsafe {
        let title = title.to_owned();
        let bytes = title.as_bytes_with_null();
        glutSetWindowTitle(to_ptr(bytes) as *c_char);
    }
}

pub fn reshape_window(window: Window, width: c_int, height: c_int) {
    unsafe {
        let current_window = glutGetWindow();
        glutSetWindow(*window);
        glutReshapeWindow(width, height);
        glutSetWindow(current_window);
    }
}

pub fn display_callback_tls_key(_callback: @@fn()) {
    // Empty.
}

pub extern fn display_callback() {
    unsafe {
        let callback = local_data_get(display_callback_tls_key).get();
        (*callback)();
    }
}

pub fn display_func(callback: @fn()) {
    unsafe {
        local_data_set(display_callback_tls_key, @callback);
        glutDisplayFunc(display_callback);
    }
}

pub fn keyboard_callback_tls_key(_: @@fn(key: c_uchar, x: c_int, y: c_int)) {
    // Empty.
}

pub extern fn keyboard_callback(key: c_uchar, x: c_int, y: c_int) {
    unsafe {
        let callback = local_data_get(keyboard_callback_tls_key).get();
        (*callback)(key, x, y)
    }
}

pub fn keyboard_func(callback: @fn(key: c_uchar, x: c_int, y: c_int)) {
    unsafe {
        local_data_set(keyboard_callback_tls_key, @callback);
        glutKeyboardFunc(keyboard_callback);
    }
}

pub fn mouse_callback_tls_key(_callback: @@fn(button: c_int, state: c_int, x: c_int, y: c_int)) {
    // Empty.
}

pub extern fn mouse_callback(button: c_int, state: c_int, x: c_int, y: c_int) {
    unsafe {
        let callback = local_data_get(mouse_callback_tls_key).get();
        (*callback)(button, state, x, y);
    }
}

pub fn mouse_func(callback: @fn(button: c_int, state: c_int, x: c_int, y: c_int)) {
    unsafe {
        local_data_set(mouse_callback_tls_key, @callback);
        glutMouseFunc(mouse_callback);
    }
}

pub fn motion_callback_tls_key(_callback: @@fn(x: c_int, y: c_int)) {
    // Empty.
}

pub extern fn motion_callback(x: c_int, y: c_int) {
    unsafe {
        let callback = local_data_get(motion_callback_tls_key).get();
        (*callback)(x, y);
    }
}

pub fn motion_func(callback: @fn(x: c_int, y: c_int)) {
    unsafe {
        local_data_set(motion_callback_tls_key, @callback);
        glutMotionFunc(motion_callback);
    }
}

pub fn passive_motion_callback_tls_key(_callback: @@fn(x: c_int, y: c_int)) {
    // Empty.
}

pub extern fn passive_motion_callback(x: c_int, y: c_int) {
    unsafe {
        let callback = local_data_get(passive_motion_callback_tls_key).get();
        (*callback)(x, y);
    }
}

pub fn passive_motion_func(callback: @fn(x: c_int, y: c_int)) {
    unsafe {
        local_data_set(passive_motion_callback_tls_key, @callback);
        glutPassiveMotionFunc(passive_motion_callback);
    }
}

pub fn timer_callback_tls_key(_callback: @~[@fn()]) {
    // Empty.
}

pub extern fn timer_callback(index: int) {
    unsafe {
        let callbacks = local_data_get(timer_callback_tls_key).get();
        ((*callbacks)[index as uint])();
    }
}

pub fn timer_func(msecs: u32, callback: @fn()) {
    unsafe {
        let callbacks;
        match local_data_get(timer_callback_tls_key) {
            None => {
                callbacks = @mut ~[];
                local_data_set(timer_callback_tls_key, cast::transmute(callbacks));
            }
            Some(existing_callbacks) => {
                callbacks = cast::transmute(existing_callbacks);
            }
        }

        callbacks.push(callback);
        let index = (callbacks.len() - 1) as c_int;
        glutTimerFunc(msecs, timer_callback, index);
    }
}

pub fn reshape_callback_tls_key(_callback: @@fn(x: c_int, y: c_int)) {
    // Empty.
}

pub extern fn reshape_callback(width: c_int, height: c_int) {
    unsafe {
        let callback = local_data_get(reshape_callback_tls_key).get();
        (*callback)(width, height);
    }
}

pub fn reshape_func(_window: Window, callback: @fn(x: c_int, y: c_int)) {
    unsafe {
        local_data_set(reshape_callback_tls_key, @callback);
        glutReshapeFunc(reshape_callback);
    }
}

pub fn idle_callback_tls_key(_callback: @@fn()) {
    // Empty.
}

pub extern fn idle_callback() {
    unsafe {
        let callback = local_data_get(idle_callback_tls_key).get();
        (*callback)();
    }
}

pub fn idle_func(callback: @fn()) {
    unsafe {
        local_data_set(idle_callback_tls_key, @callback);
        glutIdleFunc(idle_callback);
    }
}

// Mouse wheel handling.
//
// This is not part of the standard, but it's supported by freeglut and our Mac hack.
pub fn mouse_wheel_callback_tls_key(_callback: @@fn(wheel: c_int,
                                                    direction: c_int,
                                                    x: c_int,
                                                    y: c_int)) {
    // Empty.
}

#[cfg(target_os="linux")]
pub extern fn mouse_wheel_callback(wheel: c_int, direction: c_int, x: c_int, y: c_int) {
    unsafe {
        let callback = local_data_get(mouse_wheel_callback_tls_key).get();
        (*callback)(wheel, direction, x, y)
    }
}

#[cfg(target_os="linux")]
pub fn mouse_wheel_func(callback: @fn(wheel: c_int, direction: c_int, x: c_int, y: c_int)) {
    unsafe {
        local_data_set(mouse_wheel_callback_tls_key, @callback);
        glutMouseWheelFunc(mouse_wheel_callback);
    }
}

#[cfg(target_os="macos")]
pub fn mouse_wheel_func(callback: @fn(wheel: c_int, direction: c_int, x: c_int, y: c_int)) {
    unsafe {
        local_data_set(mouse_wheel_callback_tls_key, @callback);
    }
}

#[cfg(target_os="macos")]
pub fn check_loop() {
    unsafe {
        glutCheckLoop();
    }
}

#[cfg(target_os="linux")]
pub fn check_loop() {
    unsafe {
        glutMainLoopEvent();
    }
}

pub fn init_display_mode(mode: c_uint) {
    unsafe {
        glutInitDisplayMode(mode);
    }
}

pub fn swap_buffers() {
    unsafe {
        glutSwapBuffers();
    }
}

pub fn post_redisplay() {
    unsafe {
        glutPostRedisplay();
    }
}

pub fn get(state: State) -> c_int {
    unsafe {
        let glut_state;
        match state {
            WindowWidth => glut_state = WINDOW_WIDTH,
            WindowHeight => glut_state = WINDOW_HEIGHT
        }
        glutGet(glut_state)
    }
}

pub fn get_modifiers() -> c_int {
    unsafe {
        glutGetModifiers()
    }
}

#[cfg(target_os="macos")]
#[nolink]
#[link_args="-framework GLUT"]
extern {
}

#[cfg(target_os="linux")]
#[link_args="-lglut"]
extern {
}

#[cfg(target_os="macos")]
#[nolink]
extern {
    // Mac GLUT extension.
    fn glutCheckLoop();
}

#[cfg(target_os="linux")]
#[nolink]
extern {
    // freeglut extension.
    fn glutMainLoopEvent();
}

#[nolink]
extern {

pub fn glutInit(argcp: *c_int, argv: **c_char);

pub fn glutInitDisplayMode(mode: c_uint);

pub fn glutInitDisplayString(string: *c_char);

pub fn glutInitWindowPosition(x: c_int, y: c_int);

pub fn glutInitWindowSize(width: c_int, height: c_int);

pub fn glutMainLoop();

pub fn glutCreateWindow(title: *c_char) -> c_int;

pub fn glutCreateSubWindow(win: c_int, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int;

pub fn glutDestroyWindow(win: c_int);

pub fn glutPostRedisplay();

pub fn glutPostWindowRedisplay(win: c_int);

pub fn glutSwapBuffers();

pub fn glutGetWindow() -> c_int;

pub fn glutSetWindow(win: c_int);

pub fn glutSetWindowTitle(title: *c_char);

pub fn glutSetIconTitle(title: *c_char);

pub fn glutPositionWindow(x: c_int, y: c_int);

pub fn glutReshapeWindow(width: c_int, height: c_int);

pub fn glutPopWindow();

pub fn glutPushWindow();

pub fn glutIconifyWindow();

pub fn glutShowWindow();

pub fn glutHideWindow();

pub fn glutFullScreen();

pub fn glutSetCursor(cursor: c_int);

pub fn glutWarpPointer(x: c_int, y: c_int);

pub fn glutEstablishOverlay();

pub fn glutRemoveOverlay();

pub fn glutUseLayer(layer: GLenum);

pub fn glutPostOverlayRedisplay();

pub fn glutPostWindowOverlayRedisplay(win: c_int);

pub fn glutShowOverlay();

pub fn glutHideOverlay();

pub fn glutCreateMenu(arg1: *u8) -> c_int;

pub fn glutDestroyMenu(menu: c_int);

pub fn glutGetMenu() -> c_int;

pub fn glutSetMenu(menu: c_int);

pub fn glutAddMenuEntry(label: *c_char, value: c_int);

pub fn glutAddSubMenu(label: *c_char, submenu: c_int);

pub fn glutChangeToMenuEntry(item: c_int, label: *c_char, value: c_int);

pub fn glutChangeToSubMenu(item: c_int, label: *c_char, submenu: c_int);

pub fn glutRemoveMenuItem(item: c_int);

pub fn glutAttachMenu(button: c_int);

pub fn glutDetachMenu(button: c_int);

pub fn glutDisplayFunc(func: *u8);

pub fn glutReshapeFunc(func: *u8);

pub fn glutKeyboardFunc(func: *u8);

pub fn glutMouseFunc(func: *u8);

#[cfg(target_os="linux")]
pub fn glutMouseWheelFunc(func: *u8);

pub fn glutMotionFunc(func: *u8);

pub fn glutPassiveMotionFunc(func: *u8);

pub fn glutEntryFunc(func: *u8);

pub fn glutVisibilityFunc(func: *u8);

pub fn glutIdleFunc(func: *u8);

pub fn glutTimerFunc(millis: c_uint, func: *u8, value: c_int);

pub fn glutMenuStateFunc(func: *u8);

pub fn glutSpecialFunc(func: *u8);

pub fn glutSpaceballMotionFunc(func: *u8);

pub fn glutSpaceballRotateFunc(func: *u8);

pub fn glutSpaceballButtonFunc(func: *u8);

pub fn glutButtonBoxFunc(func: *u8);

pub fn glutDialsFunc(func: *u8);

pub fn glutTabletMotionFunc(func: *u8);

pub fn glutTabletButtonFunc(func: *u8);

pub fn glutMenuStatusFunc(func: *u8);

pub fn glutOverlayDisplayFunc(func: *u8);

pub fn glutWindowStatusFunc(func: *u8);

pub fn glutKeyboardUpFunc(func: *u8);

pub fn glutSpecialUpFunc(func: *u8);

pub fn glutJoystickFunc(func: *u8, pollInterval: c_int);

pub fn glutSetColor(arg1: c_int, red: GLfloat, green: GLfloat, blue: GLfloat);

pub fn glutGetColor(ndx: c_int, component: c_int) -> GLfloat;

pub fn glutCopyColormap(win: c_int);

pub fn glutGet(_type: GLenum) -> c_int;

pub fn glutDeviceGet(_type: GLenum) -> c_int;

pub fn glutExtensionSupported(name: *c_char) -> c_int;

pub fn glutGetModifiers() -> c_int;

pub fn glutLayerGet(_type: GLenum) -> c_int;

pub fn glutGetProcAddress(procName: *c_char) -> *c_void;

pub fn glutBitmapCharacter(font: *c_void, character: c_int);

pub fn glutBitmapWidth(font: *c_void, character: c_int) -> c_int;

pub fn glutStrokeCharacter(font: *c_void, character: c_int);

pub fn glutStrokeWidth(font: *c_void, character: c_int) -> c_int;

pub fn glutBitmapLength(font: *c_void, string: *c_uchar) -> c_int;

pub fn glutStrokeLength(font: *c_void, string: *c_uchar) -> c_int;

pub fn glutWireSphere(radius: GLdouble, slices: GLint, stacks: GLint);

pub fn glutSolidSphere(radius: GLdouble, slices: GLint, stacks: GLint);

pub fn glutWireCone(base: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);

pub fn glutSolidCone(base: GLdouble, height: GLdouble, slices: GLint, stacks: GLint);

pub fn glutWireCube(size: GLdouble);

pub fn glutSolidCube(size: GLdouble);

pub fn glutWireTorus(innerRadius: GLdouble, outerRadius: GLdouble, sides: GLint, rings: GLint);

pub fn glutSolidTorus(innerRadius: GLdouble, outerRadius: GLdouble, sides: GLint, rings: GLint);

pub fn glutWireDodecahedron();

pub fn glutSolidDodecahedron();

pub fn glutWireTeapot(size: GLdouble);

pub fn glutSolidTeapot(size: GLdouble);

pub fn glutWireOctahedron();

pub fn glutSolidOctahedron();

pub fn glutWireTetrahedron();

pub fn glutSolidTetrahedron();

pub fn glutWireIcosahedron();

pub fn glutSolidIcosahedron();

pub fn glutVideoResizeGet(param: GLenum) -> c_int;

pub fn glutSetupVideoResizing();

pub fn glutStopVideoResizing();

pub fn glutVideoResize(x: c_int, y: c_int, width: c_int, height: c_int);

pub fn glutVideoPan(x: c_int, y: c_int, width: c_int, height: c_int);

pub fn glutReportErrors();

pub fn glutIgnoreKeyRepeat(ignore: c_int);

pub fn glutSetKeyRepeat(repeatMode: c_int);

pub fn glutForceJoystickFunc();

pub fn glutGameModeString(string: *c_char);

pub fn glutEnterGameMode() -> c_int;

pub fn glutLeaveGameMode();

pub fn glutGameModeGet(mode: GLenum) -> c_int;

}
