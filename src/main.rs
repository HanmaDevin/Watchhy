use gtk::{
    Application, ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::{self},
    prelude::GtkWindowExt,
};

pub const APP_ID: &str = "org.hanmadevin.Watchhy";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Watchhy")
        .build();

    window.present();
}
