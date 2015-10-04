// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
//use glib::translate::ToGlibPtr;

/// MenuBar — The widget used to store menu items
struct_Widget!(MenuBar);

impl MenuBar {
    pub fn new() -> Option<MenuBar> {
        let tmp_pointer = unsafe { ffi::gtk_menu_bar_new() };
        check_pointer!(tmp_pointer, MenuBar)
    }
    
    // we're missing GMenuModel to implement new_from_model
}

impl_drop!(MenuBar);
impl_TraitWidget!(MenuBar);

impl ::ContainerTrait for MenuBar {}
impl ::MenuBarTrait for MenuBar {}
