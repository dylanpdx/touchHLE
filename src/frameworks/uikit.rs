//! The UIKit framework.
//!
//! For the time being the focus of this project is on running games, which are
//! likely to use UIKit in very simple and limited ways, so this implementation
//! will probably take a lot of shortcuts.

use crate::dyld::FunctionExports;
use crate::export_c_func;

pub mod ui_accelerometer;
pub mod ui_application;
pub mod ui_device;
pub mod ui_nib;
pub mod ui_responder;
pub mod ui_screen;
pub mod ui_view;
pub mod ui_window;

#[derive(Default)]
pub struct State {
    ui_accelerometer: ui_accelerometer::State,
    ui_application: ui_application::State,
    ui_screen: ui_screen::State,
}

pub const FUNCTIONS: FunctionExports = {
    use ui_application::UIApplicationMain;
    &[export_c_func!(UIApplicationMain(_, _, _, _))]
};
