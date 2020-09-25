use core::option::Option::Some;
use gtk::ComboBoxTextExt;

pub trait SelectedText {
    fn get_selected_text(&self) -> String;
}

impl SelectedText for gtk::ComboBoxText {
    fn get_selected_text(&self) -> String {
        self.get_active_text()
            .or(Some(glib::GString::from("")))
            .as_ref()
            .unwrap()
            .to_string()
    }
}
