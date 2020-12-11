extern crate gtk;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("couldnt init gtk");

    let glade_src = include_str!("builder_basics.glade");
    print!("{}", glade_src);
    let builder = gtk::Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("window1").unwrap();
    window.show_all();

    gtk::main();
}
