use gtk::{
    ApplicationWindow, Window,
    gio::{
        self,
        prelude::{ApplicationExt, ApplicationExtManual},
    },
    glib::{self},
    prelude::GtkWindowExt,
};

use crate::{
    config::app_id,
    models::Mode,
    scraper::{get_episode_streams, get_video_url_with_quality, search_anime},
};

mod config;
mod models;
mod scraper;
mod window;

fn main() -> glib::ExitCode {
    let res = gio::Resource::load("../resources/resources.gresource.xml")
        .expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = adw::Application::builder().application_id(app_id()).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);

    window.present();
}
