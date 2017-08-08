mod cmark;
fn main() {
    println!("Hello cmark !");
    unsafe {
        let out = cmark::cmark_markdown_to_html("# hello\n## world".as_ptr(), 16, 0);
        let mut i =0;
        loop {
            if *out.offset(i) == 0 {
                break;
            }
            i = i+ 1;
        }
        let out = String::from_raw_parts(out, i as usize, i as usize);
        println!("{}", out);
    }
}
