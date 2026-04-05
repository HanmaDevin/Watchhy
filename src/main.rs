use gtk::{
    ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::{self},
    prelude::GtkWindowExt,
};

use crate::scraper::{episode_list, get_episode_urls, search_anime};

mod scraper;

pub const APP_ID: &str = "org.hanmadevin.Watchhy";

fn main() {
    // let app = adw::Application::builder().application_id(APP_ID).build();

    // app.connect_activate(build_ui);

    // app.run()

    search_anime("One Piece");
}

fn build_ui(app: &adw::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Watchhy")
        .build();

    window.present();
}
