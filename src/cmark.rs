#[warn(dead_code)]
use std::ops::Drop;
use std::ops::Deref;

//#[marco_use]
//mod macros;

extern "C" {
    pub fn cmark_markdown_to_html(text: *const u8, len: usize, options: i32) -> *mut u8;
    pub fn free(text:*mut u8);
// pub fn cmark_get_default_mem_allocator() ->
}


pub struct HtmlBody {
    raw_body: Option<*mut u8>,
    body: String,
}
impl HtmlBody {
    pub fn new() -> HtmlBody {
        HtmlBody {
            raw_body: None,
            body: String::new(),
        }
    }

    pub fn new_from_markdown(markdown: &str) -> HtmlBody {

        let html = HtmlBody::_load_markdown(markdown);
        match html {
            Some(x) => unsafe {
                HtmlBody {
                    raw_body: Some(x.0),
                    body: String::from_raw_parts(x.0, x.1, x.1),
                }
            },
            _ => HtmlBody::new(),
        }
    }

    fn _load_markdown(text: &str) -> Option<(*mut u8, usize)> {
        unsafe {
            let s:*mut u8 = cmark_markdown_to_html(text.as_ptr(), text.len(), 0);
            let i = cstr_lenght!(s);
            // let out = String::from_raw_parts(out, i as usize, i as usize);
            if i != 0 {
                Some((s, i as usize))
            } else {
                None
            }
        }
    }

    fn markdown_to_html(text:&str) -> HtmlBody {
        unsafe {
            let s:*mut u8 = cmark_markdown_to_html(text.as_ptr(), text.len(), 0);
            let i = cstr_lenght!(s) as usize;
            if i != 0 {
                HtmlBody {
                    raw_body: Some(s),
                    body: String::from_raw_parts(s, i, i),
                }
            } else {
                HtmlBody::new()  
            }
        }
    }

    pub fn load_markdown(&mut self, markdown: &str) -> bool {
        let html = HtmlBody::_load_markdown(markdown);
        match html {
            Some(x) => {
                self.raw_body = Some(x.0);
                unsafe {
                    self.body = String::from_raw_parts(x.0, x.1, x.1);
                }
                true
            }
            _ => false,
        }
    }

    pub fn as_string(&self) -> &String {
        &self.body
    }
}
impl Drop for HtmlBody {
    fn drop(&mut self) {
        match self.raw_body {
            Some(x) => unsafe {
                free(x);
            },
            None => {}
        }
    }
}
impl Deref for HtmlBody {
    type Target = String ; // 目标类型
    fn deref<'a>(&'a self) -> &'a String{
        &self.body // 返回String类型的引用
    }
}

