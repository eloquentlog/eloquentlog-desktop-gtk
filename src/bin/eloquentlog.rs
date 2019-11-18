extern crate gio;
extern crate gtk;

use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{
    AboutDialog, AccelGroup, Application, ApplicationWindow,
   Label, Menu, MenuBar, MenuItem,  Orientation, WindowPosition,
};

const WEBSITE_URL: &str =
    "https://gitlab.com/eloquentlog/eloquentlog-desktop-gtk";

fn build_ui(app: &Application) {
    let win = ApplicationWindow::new(app);

    win.set_title("Eloquentlog");
    win.set_border_width(10);
    win.set_position(WindowPosition::Center);

    let vbox = gtk::Box::new(Orientation::Vertical, 10);

    let accl_grp = AccelGroup::new();
    win.add_accel_group(&accl_grp);

    // menubar
    let bar = MenuBar::new();

    // file
    let file  = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");

    let menu = Menu::new();
    menu.append(&about);
    menu.append(&quit);
   file.set_submenu(Some(&menu));

    bar.append(&file);

    let weak = win.downgrade();
    quit.connect_activate(move |_| {
        let win = match weak.upgrade() {
            Some(o) => o,
            None => return (),
        };
        win.destroy();
    });

    // main
    let lbl = Label::new(Some("Sample"));
    // https://gtk-rs.org/docs/gtk/trait.BoxExt.html#tymethod.pack_start
    vbox.pack_start(&bar, false, true, 0);
    vbox.pack_start(&lbl, true, true, 0);
    win.add(&vbox);

    win.show_all();

    about.connect_activate(move |_| {
        let dlg = AboutDialog::new();
        dlg.set_title("About");
        dlg.set_website_label(Some("Eloquentlog Desktop for Linux"));
        dlg.set_website(Some(WEBSITE_URL));
        dlg.set_authors(&["Lupine Software LLC"]);
        dlg.set_transient_for(Some(&win));
        dlg.run();
        dlg.destroy();
    });
}

fn main() {
    let application =
        Application::new(Some("com.eloquentlog.desktop"), Default::default())
            .expect("Initialization faild");
    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
