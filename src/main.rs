#[warn(unused_imports)]
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate webkit2gtk;
extern crate sourceview;

use glib::object::*;
use gtk::prelude::*;
use gtk::{Builder, Button, MessageDialog, ContainerExt, Inhibit, WidgetExt, WindowType, Window,
          Scrollable, ScrolledWindow, TextView, BoxExt, MenuItem, MenuItemExt, FileChooserDialog,
          FileChooserAction, FileChooserExt, FileChooser, FileFilter, FileChooserButton};
// use sourceview::{View, ViewExt, Buffer, BufferExt};

//#[cfg(feature="v2_4")]
//use glib::ToVariant;

use webkit2gtk::{WebContext, WebView, WebViewExtManual};
#[cfg(feature = "v2_6")]
use webkit2gtk::UserContentManager;


use std::ops::Drop;
use std::ops::Deref;
// use std::borrow::Borrow;

use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::error::Error;

#[macro_use]
mod macros;
mod cmark;

enum ViewMode {
    EditOnly,
    HtmlOnly(bool),
    Preview,
}

struct App {
    builder: Builder,
    window: Window,
    scrolled_edit: ScrolledWindow,
    // scrolled_html: ScrolledWindow,
    // textview_html: TextView,
    main_box: gtk::Box,
    webcontext: WebContext,
    webview: WebView,
    htmltext: String, //Rc<Cell<String>>,
    filename: String, //Rc<Cell<String>>,
    viewmode: ViewMode,
    file_new: MenuItem,
    file_open: MenuItem,
    file_save: MenuItem,
    file_save_as: MenuItem,
    file_quit: MenuItem,
    sourceview: TextView,
    view_html: MenuItem,
    view_render: MenuItem,
    view_perview: MenuItem,
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
        let window: Window = builder.get_object("window").unwrap();

        let file_new = builder.get_object("file_new").unwrap();
        let file_open = builder.get_object("file_open").unwrap();
        let file_save = builder.get_object("file_save").unwrap();
        let file_save_as = builder.get_object("file_save_as").unwrap();
        let file_quit = builder.get_object("file_quit").unwrap();

        let view_html = builder.get_object("view_html").unwrap();
        let view_render = builder.get_object("view_render").unwrap();
        let view_perview = builder.get_object("view_perview").unwrap();

        let scrolled_edit: ScrolledWindow = builder.get_object("scrolled_edit").unwrap();
        // let scrolled_html = builder.get_object("scrolled_html").unwrap();
        //let textview_html = builder.get_object("textview_html").unwrap();
        let main_box: gtk::Box = builder.get_object("main_box").unwrap();

        let context = WebContext::get_default().unwrap();
    #[cfg(not(feature="v2_6"))]
        let webview = WebView::new_with_context(&context);
        main_box.pack_end(&webview, true, true, 0);

        let mut v: TextView = TextView::new();
        //v.set_show_line_numbers(true);
        //v.set_insert_spaces_instead_of_tabs(true);
        scrolled_edit.add(&v);

        App {
            builder: builder,
            window: window,
            scrolled_edit: scrolled_edit,
            //scrolled_html: scrolled_html,
            // textview_html: textview_html,
            main_box: main_box,
            webcontext: context,
            webview: webview,
            htmltext: String::new(), // Rc::new(Cell::new(String::new())),
            filename: String::new(), // Rc::new(Cell::new(String::new())),
            viewmode: ViewMode::HtmlOnly(true),
            file_new: file_new,
            file_open: file_open,
            file_save: file_save,
            file_save_as: file_save_as,
            file_quit: file_quit,
            sourceview: v,
            view_html: view_html,
            view_render: view_render,
            view_perview: view_perview,
        }
    }

    fn open_file(&mut self, filename: &str) {

        let mut f = File::open(filename).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );
        self.filename = filename.to_string();

        let base_css = include_str!("../static/base.css");
        let base_html = include_str!("../static/base.html");
        let html = cmark::HtmlBody::from_markdown(&contents);
        if !html.as_string().is_empty() {
            let result = str::replace(base_html, "{%style%}", base_css);
            let result = str::replace(&result, "{%body%}", &html);
            self.htmltext = result;
            match self.viewmode {
                ViewMode::HtmlOnly(x) => {
                    if !x {
                        self.webview.load_plain_text(&self.htmltext);
                    } else {
                        self.webview.load_html(&self.htmltext, Some(filename));
                    }
                    let buf = self.sourceview.get_buffer().unwrap();
                    buf.set_text(&contents);
                }
                ViewMode::EditOnly => {
                    // self.sourceview.set_show_line_numbers(true);
                    let buf = self.sourceview.get_buffer().unwrap();
                    buf.set_text(&contents);
                }
                ViewMode::Preview => {
                    self.webview.load_html(&self.htmltext, Some(filename));
                    let buf = self.sourceview.get_buffer().unwrap();
                    buf.set_text(&contents);
                }
            }
        } else {
            println!("markdown convert to html failed");
        }
    }

    fn change_view(&mut self, mode: ViewMode) {
        match mode {
            ViewMode::EditOnly => {
                self.webview.hide();
                self.scrolled_edit.show();
            }
            ViewMode::HtmlOnly(_) => {
                self.webview.show();
                self.scrolled_edit.hide();
            }
            ViewMode::Preview => {
                self.webview.show();
                self.scrolled_edit.show();
            }
        }

    }

    fn save_as(&self) {}
    fn save(&self) {}
}

impl_deref!(App, Builder, builder);


fn main() {
    //let html = cmark::HtmlBody::from_markdown("# hello\n## world");
    //println!("{}", html.as_string());

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("../static/markedit.glade");
    let app: Rc<RefCell<App>> = Rc::new(RefCell::new(App::new(glade_src)));

    app.borrow().window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    let a = app.clone();
    app.borrow().file_open.connect_activate(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Open File"),
            Some(&(a.borrow().window)),
            FileChooserAction::Open,
        );
        //dialog.add_button(gtk::Stock::CANCEL, gtk::RESPONSE_CANCEL);
        // dialog.add_button(gtk::Stock::OPEN, gtk::RESPONSE_OK);
        dialog.add_button("Ok", 1);
        dialog.add_button("Cancel", 0);
        let filter = FileFilter::new();
        filter.set_name("markdown files");
        filter.add_pattern("*.md");
        dialog.add_filter(&filter);

        let f = dialog.run();
        dialog.close();
        if f == 1 {
            // #[cfg(feature = "v3_22")]
            match dialog.get_filename() {
                Some(x) => {
                    match x.to_str() {
                        Some(f) => {
                            a.borrow_mut().open_file(f);
                        }
                        _ => {
                            println!("not found file");
                        }
                    }
                }
                _ => {
                    println!("not found file");
                }
            }
        }
    });

    let a = app.clone();
    app.borrow().view_perview.connect_activate(move |_| {});
    let a = app.clone();
    app.borrow().file_save.connect_activate(move |_| {
        let filename = &a.borrow().filename;
        let buf = a.borrow().sourceview.get_buffer().unwrap();
        let start_iter = buf.get_start_iter();
        let end_iter = buf.get_end_iter();
        let text = buf.get_text(&start_iter, &end_iter, false).unwrap();
        let mut options = OpenOptions::new();
        let mut file = options
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)
            .unwrap();
        match file.write_all(text.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", filename, why.description());
            }
            Ok(_) => println!("successfully wrote to {}", filename),
        }
    });

    let a = app.clone();
    app.borrow().file_save_as.connect_activate(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Save File"),
            Some(&(a.borrow().window)),
            FileChooserAction::Save,
        );
        //dialog.add_button(gtk::Stock::CANCEL, gtk::RESPONSE_CANCEL);
        // dialog.add_button(gtk::Stock::OPEN, gtk::RESPONSE_OK);
        dialog.add_button("Ok", 1);
        dialog.add_button("Cancel", 0);
        let filter = FileFilter::new();
        filter.set_name("markdown files");
        filter.add_pattern("*.md");
        dialog.add_filter(&filter);

        let code = dialog.run();
        dialog.close();
        if code == 1 {
            // #[cfg(feature = "v3_22")]
            match dialog.get_filename() {
                Some(x) => {
                    match x.to_str() {
                        Some(filename) => {
                            let buf = a.borrow().sourceview.get_buffer().unwrap();
                            let start_iter = buf.get_start_iter();
                            let end_iter = buf.get_end_iter();
                            let text = buf.get_text(&start_iter, &end_iter, false).unwrap();
                            // let mut file = File::open(filename).expect("file not found");
                            let mut options = OpenOptions::new();
                            let mut file = options
                                .create_new(true)
                                .write(true)
                                .truncate(true)
                                .open(filename)
                                .unwrap();
                            match file.write_all(text.as_bytes()) {
                                Err(why) => {
                                    panic!(
                                        "couldn't write to {}: {} {}",
                                        filename,
                                        why.description(),
                                        text
                                    )
                                }
                                Ok(_) => {
                                    println!("successfully wrote to {}", filename);
                                    a.borrow_mut().open_file(filename);
                                }
                            }
                        }
                        _ => {
                            println!("not found file");
                        }
                    }
                }
                _ => {
                    println!("not found file");
                }
            }
        }


    });
    let a = app.clone();
    app.borrow().file_quit.connect_activate(move |_| {
        gtk::main_quit();
        Inhibit(false);
    });

    app.borrow().window.show_all();

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        app.borrow_mut().change_view(ViewMode::Preview);
        app.borrow_mut().open_file(&args[1]);
    } else {
        app.borrow_mut().change_view(ViewMode::Preview);
    }

    gtk::main();
}
