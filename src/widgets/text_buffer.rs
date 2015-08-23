// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;

use ffi;
use FFIWidget;
use TextIter;

use cast::GTK_TEXT_BUFFER;
/// GtkTextBuffer — Stores attributed text for display in a GtkTextView

struct_Widget!(TextBuffer);

impl TextBuffer {
    pub fn new(text_tag_table: Option<::TextTagTable>) -> Option<TextBuffer> {
        let tmp_pointer = unsafe {
            match text_tag_table {
                Some(ttl) => ffi::gtk_text_buffer_new(ttl.unwrap_pointer()),
                None      => ffi::gtk_text_buffer_new(ptr::null_mut())
            }
        };

        check_pointer!(tmp_pointer, TextBuffer)
    }

   	pub fn get_start_iter(&self) -> Option<TextIter> {
    	let text_iter = TextIter::new();
    	match text_iter {
    		Some(iter) => {
    				unsafe {
					    	ffi::gtk_text_buffer_get_start_iter(
					    		GTK_TEXT_BUFFER(self.unwrap_widget()), 
					    		iter.unwrap_pointer() as *mut ffi::GtkTextIter);
				 	};
	   			    Some(iter)
				},
		    None => text_iter
		}
   	}
}

impl_drop!(TextBuffer);
impl_TraitWidget!(TextBuffer);

impl ::TextBufferTrait for TextBuffer {}
