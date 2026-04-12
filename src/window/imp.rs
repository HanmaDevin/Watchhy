use std::cell::OnceCell;

use gtk::gio::Settings;
use gtk::glib::subclass::InitializingObject;
use gtk::{CompositeTemplate, SearchEntry};

use adw::subclass::prelude::*;
use gtk::ApplicationWindow;
use gtk::glib;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/hanmadevin/watchhy/window.ui")]
pub struct Window {
    #[template_child]
    // variable name has to be the `id` field of object
    pub search_entry: TemplateChild<SearchEntry>,
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "WatchhyWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.get_search();
        obj.setup_settings();
        obj.load_window_size();
    }
}
impl WidgetImpl for Window {}

impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> glib::Propagation {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");
        // Allow to invoke other event handlers
        glib::Propagation::Proceed
    }
}

impl ApplicationWindowImpl for Window {}
