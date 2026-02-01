use gtk4 as gtk;
use gtk::prelude::*;
use thunder_core::{ThunderConfig, ThunderCore};

fn main() {
    env_logger::init();

    let app = gtk::Application::builder()
        .application_id("io.thunder.browser")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let core = ThunderCore::new(ThunderConfig::default());
    core.init().expect("core init failed");

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Thunder Browser")
        .default_width(1200)
        .default_height(800)
        .build();

    let header = gtk::HeaderBar::builder()
        .show_title_buttons(true)
        .build();

    let url_entry = gtk::Entry::builder()
        .hexpand(true)
        .placeholder_text("URL gir (https://...)")
        .text(core.homepage())
        .build();

    let core_nav = core;
    url_entry.connect_activate(move |e| {
        let url = e.text().to_string();
        let _ = core_nav.navigate(&url);
    });

    header.pack_start(&url_entry);
    window.set_titlebar(Some(&header));

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let label = gtk::Label::new(Some(
        "Gecko WebView henüz bağlı değil.\nBu ekran stub.",
    ));
    label.set_margin_all(24);

    content.append(&label);
    window.set_child(Some(&content));
    window.present();
}
