use crate::app_error::AppError;
use crate::{SelectedText, UIEvent};
use async_std::task;
use clap::App;
use emu_check::{correction_data_set_load_data, CorrectionDataSet};
use gio::prelude::*;
use glib::{clone, WeakRef};
use gtk::prelude::*;
use gtk::{Application, Builder, BuilderExt};
use std::cell::RefCell;
use std::rc::Rc;

use std::sync::mpsc::channel;

// let (tx_background, rx_background) = std::sync::mpsc::channel();

#[derive(Debug)]
pub struct AppController {
    // gui_app: Application,
    builder: Builder,
    correction_data_set: CorrectionDataSet,
    data_dir: String,
    tx_bg: std::sync::mpsc::Sender<UIEvent>,
    rx_bg: std::sync::mpsc::Receiver<UIEvent>,
    tx_ui: glib::Sender<UIEvent>,
    rx_ui: glib::Receiver<UIEvent>,
}

impl AppController {
    pub fn new<T: AsRef<str> + ?Sized>(
        data_dir: &T,
        tx_bg: std::sync::mpsc::Sender<UIEvent>,
        rx_bg: std::sync::mpsc::Receiver<UIEvent>,
        tx_ui: glib::Sender<UIEvent>,
        rx_ui: glib::Receiver<UIEvent>,
    ) -> Result<Self, AppError> {
        let ac = AppController {
            builder: Builder::new(),
            correction_data_set: Default::default(),
            data_dir: data_dir.as_ref().to_string(),
            tx_bg,
            rx_bg,
            tx_ui,
            rx_ui,
        };

        Ok(ac)
    }

    pub fn load_correction_data(&mut self) -> Result<(), AppError> {
        let cd = task::block_on(correction_data_set_load_data(&self.data_dir))
            .map_err(|e| AppError::CoreError(e))?;
        self.correction_data_set = cd;
        Ok(())
    }

    pub fn build_ui(&mut self) -> Result<(), AppError> {
        let main_window_glade = include_str!("main_window.glade");
        self.builder
            .add_from_string(main_window_glade)
            .map_err(|e| AppError::GLibError(e))?;
        Ok(())
    }

    pub fn build_actions(&mut self) -> Result<(), AppError> {
        let main_window: gtk::ApplicationWindow = self.get_main_window()?;

        // Buttons
        let btn_exit = self.get_button("btn_exit")?;
        btn_exit.connect_clicked(clone!(
            @weak main_window => move |_| main_window.close()
        ));

        let btn_compute = self.get_button("btn_compute")?;
        btn_compute.connect_clicked(clone!(
            @weak btn_compute => move |_| {
                let label = btn_compute.get_label().expect("unable to get label from btn_compute").as_str().to_string();
                println!("Clicked compute [label: {}]", &label);
            }
        ));

        // ComboBoxText changes
        let cb_machine = self.get_combo_box_text("cb_machine")?;
        let cb_energy = self.get_combo_box_text("cb_energy")?;
        let cb_applicator = self.get_combo_box_text("cb_applicator")?;
        let te_prescription_dose = self.get_entry("te_prescription_dose")?;
        let te_d2 = self.get_entry("te_d2")?;
        let te_beam_mu = self.get_entry("te_beam_mu")?;
        let te_sdd = self.get_entry("te_sdd");
        let lbl_output_factor = self.get_label("lbl_output_factor")?;
        let lbl_applicator_cf = self.get_label("lbl_applicator_cf")?;
        let lbl_expected_mu = self.get_label("lbl_expected_mu")?;
        let lbl_error = self.get_label("lbl_error")?;

        {
            let tx = self.tx_ui.clone();
            let cb = cb_machine.clone();
            cb_machine.connect_changed(move |_| {
                tx.send(UIEvent::MachineChanged(cb.get_selected_text()));
            });
        }
        Ok(())
    }

    pub fn get_object<T: glib::IsA<glib::Object>>(&self, name: &str) -> Result<T, AppError> {
        self.builder
            .get_object(name)
            .ok_or(AppError::GtkObjectNotFound)
    }

    pub fn get_combo_box_text(&self, name: &str) -> Result<gtk::ComboBoxText, AppError> {
        let obj: gtk::ComboBoxText = self.get_object(name)?;
        Ok(obj)
    }

    pub fn get_main_window(&self) -> Result<gtk::ApplicationWindow, AppError> {
        let obj: gtk::ApplicationWindow = self.get_object("main_window")?;
        Ok(obj)
    }

    pub fn get_entry(&self, name: &str) -> Result<gtk::Entry, AppError> {
        let obj: gtk::Entry = self.get_object(name)?;
        Ok(obj)
    }

    pub fn get_label(&self, name: &str) -> Result<gtk::Label, AppError> {
        let obj: gtk::Label = self.get_object(name)?;
        Ok(obj)
    }

    pub fn get_button(&self, name: &str) -> Result<gtk::Button, AppError> {
        let obj: gtk::Button = self.get_object(name)?;
        Ok(obj)
    }

    fn callback_cb_machine(&self, cb_machine: &gtk::ComboBoxText, cb_energy: &gtk::ComboBoxText) {
        let machine = cb_machine.get_selected_text();
        // let cd = self.correction_data_set.as_ref().borrow();

        // let energies = get_energies(&machine);
        // cb_energy.remove_all();
        // let n = energies.len();
        // for energy in energies {
        //     // ComboBoxTextExt::
        //     cb_energy.append_text(&energy);
        // }
        // if n > 0 {
        //     cb_energy.set_active(Some(0));
        // } else {
        //     cb_energy.set_active(None);
        // }
        // println!("callback_cb_machine");
    }
}
