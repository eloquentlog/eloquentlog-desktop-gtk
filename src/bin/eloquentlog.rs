extern crate gio;
extern crate gtk;

use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{
    AboutDialog, AccelGroup, Application, ApplicationWindow, Builder, Label,
    HeaderBar, Menu, MenuBar, MenuItem, Orientation, StackBuilder,
    StackSidebarBuilder,
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

    // menu_bar
    let mbar_src = include_str!("../ui/menu_bar.xml");
    let mbar: MenuBar = Builder::from_string(mbar_src)
        .get_object("menu_bar")
        .expect("couldn't get menu_bar");
    mbar.append(&file);

    // header_bar
    let hbar_src = include_str!("../ui/header_bar.xml");
    let hbar: HeaderBar = Builder::from_string(hbar_src)
        .get_object("header_bar")
        .expect("couldn't get header_bar");
    hbar.add(&mbar);
    awin.set_titlebar(Some(&hbar));

    // sidebar
    let stack = StackBuilder::new().expand(true).visible(true).build();
    let pref_lbl = Label::new(Some("Preferences"));
    stack.add_titled(&pref_lbl, "preferences", "Preferences");
    let sbar = StackSidebarBuilder::default().build();
    sbar.set_stack(&stack);

    let lbl = Label::new(Some("Start"));

    // https://gtk-rs.org/docs/gtk/trait.BoxExt.html#tymethod.pack_start
    let vbox = gtk::Box::new(Orientation::Vertical, 10);
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
