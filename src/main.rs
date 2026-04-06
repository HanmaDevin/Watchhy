use gtk::{
    ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::{self},
    prelude::GtkWindowExt,
};

use crate::scraper::{get_episode_streams, get_episode_urls};

mod scraper;

pub const APP_ID: &str = "org.hanmadevin.Watchhy";

fn main() {
    // let app = adw::Application::builder().application_id(APP_ID).build();

    // app.connect_activate(build_ui);

    // app.run()

    let urls = get_episode_streams("ReooPAxPMsHM4KPMY", "1");
    println!("{urls:#?}");
}

fn build_ui(app: &adw::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Watchhy")
        .build();

    window.present();
}
