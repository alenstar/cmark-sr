use std::ops::Drop;
use std::ops::Deref;

extern "C" {
    pub fn cmark_markdown_to_html(text: *const u8, len: u32, options: i32) -> *mut u8;

    // pub fn cmark_get_default_mem_allocator() ->
}
