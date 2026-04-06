use gtk::{
    ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::{self},
    prelude::GtkWindowExt,
};

use crate::{
    models::Mode,
    scraper::{get_episode_streams, get_video_url_with_quality, search_anime},
};

mod models;
mod scraper;

pub const APP_ID: &str = "org.hanmadevin.Watchhy";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // test code
    let mode = Mode::Sub;
    let search = search_anime("liar game", mode.clone());
    println!("{search:#?}");
    let streams = get_episode_streams(search[0].0.as_str(), mode, "1");
    println!("{streams:#?}");
    let url = get_video_url_with_quality(models::Quality::HD1080, streams[0].1.as_str());
    println!("{url}");

    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Watchhy")
        .build();

    window.present();
}
