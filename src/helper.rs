use glib::{ToValue, Type};
use gtk::prelude::ComboBoxExtManual;
use gtk::{ComboBoxExt, ComboBoxText, ComboBoxTextExt, TreeModel, TreeModelExt};
use log::trace;

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

pub fn get_treemodel_index_match(model: &TreeModel, to_match: String) -> Option<u32> {
    let n = model.iter_n_children(None);
    for i in 0..n {
        let opt_iter = model.iter_nth_child(None, i);
        if let Some(iter) = opt_iter {
            let value = iter.to_value();
            let t = value.type_();
            match t {
                Type::String => {
                    if let Ok(opt_s) = value.get::<String>() {
                        if opt_s.is_some() {
                            let s = opt_s.unwrap().to_string();
                            if s == to_match {
                                return Some(i as u32);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

pub fn combobox_set_value(cb: &ComboBoxText, value: String) -> bool {
    let opt_model = cb.get_model();
    if opt_model.is_none() {
        return false;
    }
    let model = opt_model.unwrap();
    let opt = get_treemodel_index_match(&model, value.clone());
    trace!("value: {}", value.clone());
    if opt.is_some() {
        trace!("index to select: {}", opt.clone().unwrap());
        cb.set_active(opt);
        return true;
    }
    false
}

pub fn combobox_set_value_or_none(cb: &ComboBoxText, value: String) {
    if !combobox_set_value(cb, value) {
        cb.set_active(Some(0));
    }
}
