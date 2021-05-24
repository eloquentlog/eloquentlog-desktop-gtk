extern crate gio;
extern crate gtk;

use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{
    AboutDialog, AccelGroup, Application, ApplicationWindow, Builder, Label,
    Menu, MenuBar, MenuItem, Orientation, StackBuilder, StackSidebarBuilder,
};

fn build_ui(app: &Application) {
    let awin_src = include_str!("../ui/window.xml");
    let awin: ApplicationWindow = Builder::from_string(awin_src)
        .get_object("window")
        .expect("couldn't get window");
    awin.set_application(Some(app));

    let accl_grp = AccelGroup::new();
    awin.add_accel_group(&accl_grp);

    // menu
    let file = MenuItem::with_label("File");
    let about = MenuItem::with_label("About");
    let quit = MenuItem::with_label("Quit");

    let menu = Menu::new();
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));

    // menubar
    let mbar_src = include_str!("../ui/menubar.xml");
    let mbar: MenuBar = Builder::from_string(mbar_src)
        .get_object("menubar")
        .expect("couldn't get menubar");
    mbar.append(&file);

    // sidebar
    let stack = StackBuilder::new().expand(true).visible(true).build();
    let pref_lbl = Label::new(Some("Preferences"));
    stack.add_titled(&pref_lbl, "preferences", "Preferences");
    let sbar = StackSidebarBuilder::default().build();
    sbar.set_stack(&stack);

    // main
    let lbl = Label::new(Some("Sample"));

    // https://gtk-rs.org/docs/gtk/trait.BoxExt.html#tymethod.pack_start
    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    vbox.pack_start(&mbar, false, true, 0);
    vbox.pack_start(&sbar, false, true, 0);
    vbox.pack_start(&lbl, true, true, 0);
    awin.add(&vbox);

    awin.show_all();

    let weak = awin.downgrade();
    quit.connect_activate(move |_| {
        let win = match weak.upgrade() {
            Some(o) => o,
            None => return,
        };
        win.close();
    });

    about.connect_activate(move |_| {
        let dlog_src = include_str!("../ui/about_dialog.xml");
        let dlog: AboutDialog = Builder::from_string(dlog_src)
            .get_object("about_dialog")
            .expect("couldn't get about_dialog");
        dlog.set_transient_for(Some(&awin));
        dlog.run();
        dlog.close();
    });
}

fn main() {
    let application =
        Application::new(Some("com.eloquentlog.desktop"), Default::default())
            .expect("couldn't get initialized");
    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
