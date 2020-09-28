use gtk::ComboBoxTextExt;

pub trait SelectedText {
    fn get_selected_text(&self) -> String;
}

impl SelectedText for gtk::ComboBoxText {
    fn get_selected_text(&self) -> String {
        self.get_active_text()
            .or_else(|| Some(glib::GString::from("")))
            // .or(Some(glib::GString::from("")))
            .as_ref()
            .unwrap()
            .to_string()
    }
}

// Helper function to convert string types to an optional f64.
pub fn parse_f64<T: AsRef<str>>(value: T) -> Option<f64> {
    let x = value.as_ref().parse::<f64>();
    match x {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}
