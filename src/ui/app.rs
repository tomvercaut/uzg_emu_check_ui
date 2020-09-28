use crate::ui::connect::{Connect, Event};
use crate::ui::main_layout::MainLayout;
use crate::{parse_f64, SelectedText};
use emu_check::CorrectionDataSet;
use flume::Sender;
use gtk::prelude::*;
use log::trace;

pub struct AppState {
    pub title: String,
    pub data: CorrectionDataSet,
    pub tx: flume::Sender<Event>,
}

pub struct App {
    pub window: gtk::Window,
    pub layout: MainLayout,
    pub state: AppState,
}

impl App {
    pub fn new(state: AppState) -> Self {
        let layout = MainLayout::new();

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title(state.title.as_str());
        // gtk::Window::set_default_icon_name("typecatcher");
        // gtk::Window::set_gravity(&window, gdk::Gravity::Center);
        window.set_position(gtk::WindowPosition::CenterAlways);
        window.add(&layout.center_box);
        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let app = Self {
            window,
            layout,
            state,
        };
        app.initial_load_data();
        app
    }

    // Show the main window and the widgets
    pub fn show(&self) {
        self.window.show_all();
    }

    // Load the correction data into the UI widgets
    fn initial_load_data(&self) {
        let item_none = "None".to_string();
        // Add None to all comboboxes
        self.layout.cb_machine.append_text(item_none.as_str());
        self.layout.cb_energy.append_text(item_none.as_str());
        self.layout.cb_app.append_text(item_none.as_str());
        self.layout.cb_app_fitment.append_text(item_none.as_str());
        // Apppend the available machines
        let machines = self.state.data.get_machines();
        for machine in &machines {
            self.layout.cb_machine.append_text(machine.as_str());
        }
        // Select None in all comboboxes
        self.layout.cb_machine.set_active(Some(0));
        self.layout.cb_energy.set_active(Some(0));
        self.layout.cb_app.set_active(Some(0));
        self.layout.cb_app_fitment.set_active(Some(0));
    }

    pub fn get_machine_selected(&self) -> String {
        self.layout.cb_machine.get_selected_text()
    }

    pub fn get_energy_selected(&self) -> String {
        self.layout.cb_energy.get_selected_text()
    }

    pub fn get_energy_selected_f64(&self) -> Option<f64> {
        parse_f64(self.get_energy_selected())
    }

    pub fn get_applicator_selected(&self) -> String {
        self.layout.cb_app.get_selected_text()
    }

    pub fn get_applicator_fitment_selected(&self) -> String {
        self.layout.cb_app_fitment.get_selected_text()
    }

    // The machine selection changed
    pub fn machine_changed(&self, name: &str) {
        trace!("fn machine_changed");
        self.update_energies(name);
    }

    fn update_energies(&self, machine: &str) {
        trace!("updating energies");
        let mut j = 0;
        let current_energy = self.get_energy_selected();
        let mut energies = vec![];
        if machine != "None" {
            energies = self.state.data.get_energies(machine);
        }
        self.layout.cb_energy.remove_all();
        self.layout.cb_energy.append_text("None");
        for (i, energy) in energies.iter().enumerate() {
            let se = energy.to_string();
            self.layout.cb_energy.append_text(se.as_str());
            if current_energy == se {
                j = i + 1; // if matching, add one because None was already added to the list.
            }
        }
        self.layout.cb_energy.set_active(Some(j as u32));
    }

    // The energy selection changed
    pub fn energy_changed(&self, energy: &str) {
        trace!("fn energy_changed");
        self.update_applicators(self.get_machine_selected().as_str(), energy);
    }

    // Update the applicators
    fn update_applicators(&self, machine: &str, energy: &str) {
        trace!("updating applicators");
        trace!("machine: {}", machine);
        trace!("energy: {}", energy);
        let current_app = self.get_applicator_selected();
        trace!("current applicator: {}", current_app.as_str());
        let mut applicators = vec![];
        if machine != "None" && energy != "None" {
            if let Some(e) = parse_f64(energy) {
                applicators = self.state.data.get_applicators(machine, e);
            }
        }
        self.layout.cb_app.remove_all();
        self.layout.cb_app.append_text("None");
        let mut j = 0;
        for (i, applicator) in applicators.iter().enumerate() {
            self.layout.cb_app.append_text(applicator.as_str());
            if current_app.as_str() == applicator.as_str() {
                j = i + 1; // if matching, add one because None was already added to the list.
            }
        }
        self.layout.cb_app.set_active(Some(j as u32));
    }

    // The applicator selection changed
    pub fn applicator_changed(&self, applicator: &str) {
        trace!("fn applicator_changed");
        self.update_applicator_fitments(
            self.get_machine_selected().as_str(),
            self.get_energy_selected().as_str(),
            applicator,
        );
    }

    // Update the applicator fitments
    fn update_applicator_fitments(&self, machine: &str, energy: &str, applicator: &str) {
        trace!("updating applicators");
        trace!("machine: {}", machine);
        trace!("energy: {}", energy);
        trace!("applicator: {}", applicator);
        let current_fitment = self.get_applicator_fitment_selected();
        trace!("current applicator fitment: {}", current_fitment.as_str());
        let mut fitments = vec![];
        if machine != "None" && energy != "None" && applicator != "None" {
            if let Some(e) = parse_f64(energy) {
                fitments = self
                    .state
                    .data
                    .get_applicator_fitments(machine, e, applicator);
            }
        }
        self.layout.cb_app_fitment.remove_all();
        self.layout.cb_app_fitment.append_text("None");
        let mut j = 0;
        for (i, fitment) in fitments.iter().enumerate() {
            self.layout.cb_app_fitment.append_text(fitment.as_str());
            if current_fitment.as_str() == fitment.as_str() {
                j = i + 1;
            }
        }
        self.layout.cb_app_fitment.set_active(Some(j as u32));
    }

    pub fn applicator_fitment_changed(&self, _fitment: &str) {
        trace!("fn applicator_fitment_changed")
    }

    pub fn ssd_changed(&self, _ssd: &str) {
        trace!("fn ssd_changed");
    }

    pub fn beam_mu_changed(&self, _beam_mu: &str) {
        trace!("fn beam_mu_changed");
    }

    pub fn d2_changed(&self, _d2: &str) {
        trace!("d2_changed");
    }

    pub fn prescription_dose_changed(&self, _presc_dose: &str) {
        trace!("prescription_dose_changed");
    }

    // Quit the application
    pub fn quit(&self) {
        gtk::main_quit();
    }
}

impl Connect<Event> for App {
    fn connect_events(&self, sender: Sender<Event>) {
        self.connect_machine_changed(sender.clone());
        self.connect_energy_changed(sender.clone());
        self.connect_applicator_changed(sender.clone());
        self.connect_applicator_fitment_changed(sender.clone());
        self.connect_ssd_changed(sender.clone());
        self.connect_beam_mu_changed(sender.clone());
        self.connect_d2_changed(sender.clone());
        self.connect_prescription_dose_changed(sender.clone());
        self.connect_quit(sender);
    }

    fn connect_machine_changed(&self, sender: Sender<Event>) {
        // connect the change event from the combobox to the internal event
        self.layout.cb_machine.connect_changed(move |cb| {
            if let Err(e) = sender.send(Event::MachineChanged(cb.get_selected_text())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_energy_changed(&self, sender: Sender<Event>) {
        self.layout.cb_energy.connect_changed(move |cb| {
            if let Err(e) = sender.send(Event::EnergyChanged(cb.get_selected_text())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_applicator_changed(&self, sender: Sender<Event>) {
        self.layout.cb_app.connect_changed(move |cb| {
            if let Err(e) = sender.send(Event::ApplicatorChanged(cb.get_selected_text())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_applicator_fitment_changed(&self, sender: Sender<Event>) {
        self.layout.cb_app_fitment.connect_changed(move |cb| {
            if let Err(e) = sender.send(Event::ApplicatorFitmentChanged(cb.get_selected_text())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_ssd_changed(&self, sender: Sender<Event>) {
        self.layout.entry_ssd.connect_changed(move |entry| {
            if let Err(e) = sender.send(Event::SSDChanged(entry.get_text().to_string())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_beam_mu_changed(&self, sender: Sender<Event>) {
        self.layout.entry_beam_mu.connect_changed(move |entry| {
            if let Err(e) = sender.send(Event::BeamMUChanged(entry.get_text().to_string())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_d2_changed(&self, sender: Sender<Event>) {
        self.layout.entry_d2.connect_changed(move |entry| {
            if let Err(e) = sender.send(Event::D2Changed(entry.get_text().to_string())) {
                panic!("{}", e);
            }
        });
    }

    fn connect_prescription_dose_changed(&self, sender: Sender<Event>) {
        self.layout.entry_presc_dose.connect_changed(move |entry| {
            if let Err(e) =
                sender.send(Event::PrescriptionDoseChanged(entry.get_text().to_string()))
            {
                panic!("{}", e);
            }
        });
    }

    fn connect_quit(&self, sender: Sender<Event>) {
        self.layout.btn_exit.connect_clicked(move |_| {
            if let Err(e) = sender.send(Event::AppQuit) {
                panic!("{}", e);
            }
        });
    }
}
