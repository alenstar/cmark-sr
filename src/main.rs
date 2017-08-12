#[warn(unused_imports)]
extern crate glib;
extern crate gtk;
extern crate webkit2gtk;
extern crate sourceview;

use gtk::prelude::*;
use gtk::{Builder, Button, MessageDialog, Window};
use sourceview::{View, ViewExt};

use std::ops::Drop;
use std::ops::Deref;


#[macro_use]
mod macros;
mod cmark;

struct App {
    builder: Builder,
}

impl App {
    fn new(glade: &str) -> App {
        let builder = Builder::new();
        match builder.add_from_string(glade) {
            Err(x) => {
                println!("{:}", x);
            }
            Ok(_) => {}
        }
        App { builder: builder }
    }
}

impl_deref!(App, Builder, builder);


// #[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("Hello world !");
    unsafe {
        let out = cmark::cmark_markdown_to_html("# hello\n## world".as_ptr(), 16, 0);

        let i = cstr_lenght!(out);
        let out = String::from_raw_parts(out, i as usize, i as usize);
        println!("{}", out);
    }

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("../static/markedit.glade");
    let app = App::new(glade_src);
    let window: Window = app.get_object("window").unwrap();
    //let bigbutton: Button = app.get_object("button1").unwrap();
    //let dialog: MessageDialog = app.get_object("messagedialog1").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //bigbutton.connect_clicked(move |_| {
    //    dialog.run();
    //    dialog.hide();
    //});

    window.show_all();
    gtk::main();
}
