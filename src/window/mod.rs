use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio,
    glib::{self, Object, SignalHandlerId},
    prelude::EditableExt,
};

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn get_search(&self) -> SignalHandlerId {
        self.imp()
            .search_entry
            .connect_search_changed(|e| println!("{}", e.text()))
    }
}
