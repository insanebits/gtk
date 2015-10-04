// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! The widget used for item in menus

use ffi;

/// Menu — The widget used to hold list of menu items, for example it may be used
/// to hold "File" menu item children like open, close and etc.

struct_Widget!(Menu);

impl Menu {
    pub fn new() -> Option<Menu> {
        let tmp_pointer = unsafe { ffi::gtk_menu_new() };
        check_pointer!(tmp_pointer, Menu)
    }
}

impl_drop!(Menu);
impl_TraitWidget!(Menu);

impl ::MenuTrait for Menu {}
impl ::MenuShellTrait for Menu {}
impl ::ContainerTrait for Menu {}
