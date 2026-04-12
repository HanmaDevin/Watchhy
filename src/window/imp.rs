use gtk::glib::subclass::InitializingObject;
use gtk::{CompositeTemplate, SearchEntry};

use adw::subclass::prelude::*;
use gtk::ApplicationWindow;
use gtk::glib;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/hanmadevin/watchhy/window.ui")]
pub struct Window {
    pub search_entry: SearchEntry,
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
    }
}
impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
