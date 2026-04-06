use gtk::{
    ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::{self},
    prelude::GtkWindowExt,
};

mod models;
mod scraper;

pub const APP_ID: &str = "org.hanmadevin.Watchhy";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Watchhy")
        .build();

    window.present();
}
