// #![windows_subsystem = "windows"]

use gio::prelude::*;
use glib::{clone, WeakRef};
use gtk::prelude::*;
use gtk::{Builder, TreeModel};

use gdk::keys::constants::P;
use std::env::args;
use std::rc::Rc;

fn build_ui(application: &gtk::Application) {
    let main_window_glade = include_str!("main_window.glade");
    let builder = Builder::new();
    builder
        .add_from_string(main_window_glade)
        .expect("Couldn't add UI from string.");

    let main_window: gtk::ApplicationWindow = builder
        .get_object("main_window")
        .expect("Unable to get main_window.");
    main_window.set_application(Some(application));

    let cb_machine: gtk::ComboBoxText = builder
        .get_object("cb_machine")
        .expect("Unable to get cb_machine");
    for machine in get_machines() {
        cb_machine.append_text(machine.as_str());
    }

    // cb_machine.append_text("Synergy2");
    // cb_machine.append_text("Synergy3");

    main_window.show_all();
    build_actions(&builder);

    cb_machine.set_active(Some(0));
}

trait selected_text {
    fn get_selected_text(&self) -> String;
}

impl selected_text for gtk::ComboBoxText {
    fn get_selected_text(&self) -> String {
        self.get_active_text()
            .or(Some(glib::GString::from("")))
            .as_ref()
            .unwrap()
            .to_string()
    }
}

fn get_machines() -> Vec<String> {
    vec!["Synergy2".to_string(), "Synergy3".to_string()]
}

fn get_energies(machine: &str) -> Vec<String> {
    if machine == "Synergy2" {
        vec!["4".to_string(), "6".to_string(), "8".to_string()]
    } else if machine == "Synergy3" {
        vec!["10".to_string(), "12".to_string(), "14".to_string()]
    } else {
        vec!["None".to_string()]
    }
}

fn get_applicators(machine: &str, energy: &str) -> Vec<String> {
    if machine == "Synergy2" && get_energies(&machine).contains(&energy.to_string()) {
        vec!["6x6".to_string(), "10x10".to_string()]
    } else if machine == "Synergy3" && get_energies(&machine).contains(&energy.to_string()) {
        vec!["14x14".to_string(), "20x20".to_string()]
    } else {
        vec!["None".to_string()]
    }
}

fn get_fitments(machine: &str, energy: &str, applicator: &str) -> Vec<String> {
    if !machine.is_empty() && !energy.is_empty() && !applicator.is_empty() {
        if applicator == "6x6" {
            vec!["6x6".to_string(), "4x6".to_string()]
        } else if applicator == "10x10" {
            vec!["8x8".to_string(), "10x10".to_string()]
        } else if applicator == "14x14" {
            vec![
                "10x12".to_string(),
                "10x14".to_string(),
                "14x14".to_string(),
            ]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

fn update_results(
    machine: &str,
    energy: &str,
    applicator: &str,
    applicator_fitment: &str,
    prescription_dose: &str,
    d2: &str,
    beam_mu: &str,
    ssd: &str,
) -> Result<(String, String, String), String> {
    Ok(("unimplemented".to_string(), "unimplemented".to_string(), "unimplemented".to_string()))
}

fn build_actions(builder: &Builder) {
    let main_window: gtk::ApplicationWindow = builder
        .get_object("main_window")
        .expect("Unable to get main_window.");
    // Exit button
    let btn_exit: gtk::Button = builder
        .get_object("btn_exit")
        .expect("Unable to get btn_exit");
    btn_exit.connect_clicked(clone!(
        @weak main_window => move |_| main_window.close()
    ));

    // Compute button
    let btn_compute: gtk::Button = builder
        .get_object("btn_compute")
        .expect("Unable to get btn_compute");
    btn_compute.connect_clicked(clone!(
    @weak btn_compute, @weak btn_exit => move |_| {
        let lbl1 = btn_compute.get_label().expect("Unable to get label from btn_compute").as_str().to_string();
        let lbl2 = btn_exit.get_label().expect("Unable to get label from btn_exit").as_str().to_string();
    }
    ));

    let cb_machine: gtk::ComboBoxText = builder
        .get_object("cb_machine")
        .expect("Unable to get cb_machine");
    let cb_energy: gtk::ComboBoxText = builder
        .get_object("cb_energy")
        .expect("Unable to get cb_energy");
    let cb_applicator: gtk::ComboBoxText = builder
        .get_object("cb_applicator")
        .expect("Unable to get cb_applicator");
    let cb_applicator_fitment: gtk::ComboBoxText = builder
        .get_object("cb_applicator_fitment")
        .expect("Unable to get cb_applicator_fitment");
    let te_prescription_dose: gtk::Entry = builder
        .get_object("te_prescription_dose")
        .expect("Unable to get te_prescription_dose");
    let te_d2: gtk::Entry = builder.get_object("te_d2").expect("Unable to get te_d2");
    let te_beam_mu: gtk::Entry = builder
        .get_object("te_beam_mu")
        .expect("Unable to get te_beam_mu");
    let te_ssd: gtk::Entry = builder.get_object("te_ssd").expect("Unable to get te_ssd");
    let lbl_output_factor: gtk::Label = builder
        .get_object("lbl_output_factor")
        .expect("Unable to get lbl_output_factor");
    let lbl_applicator_cf: gtk::Label = builder
        .get_object("lbl_applicator_cf")
        .expect("Unable to get lbl_applicator_cf");
    let lbl_expected_mu: gtk::Label = builder
        .get_object("lbl_expected_mu")
        .expect("Unable to get lbl_expected_mu");
    let lbl_error : gtk::Label = builder.get_object("lbl_error").expect("Unable to get lbl_error");

    cb_machine.connect_changed(clone!(
    @weak cb_machine, @weak cb_energy => move |_| {
       callback_cb_machine(&cb_machine, &cb_energy);
    }));
    cb_energy.connect_changed(clone!(
    @weak cb_machine, @weak cb_energy, @weak cb_applicator => move |_| {
       callback_energy(&cb_machine, &cb_energy, &cb_applicator);
    }));
    cb_applicator.connect_changed(clone!(
    @weak cb_machine, @weak cb_energy, @weak cb_applicator, @weak cb_applicator_fitment => move |_| {
        callback_cb_applicator(&cb_machine, &cb_energy, &cb_applicator, &cb_applicator_fitment);
    }));
    cb_applicator_fitment.connect_changed(clone!(
        @weak cb_machine, @weak cb_energy, @weak cb_applicator, @weak cb_applicator_fitment,
        @weak te_prescription_dose, @weak te_d2, @weak te_beam_mu, @weak te_ssd,
        @weak lbl_output_factor, @weak lbl_applicator_cf, @weak lbl_expected_mu,
        @weak lbl_error => move |_| {
           callback_update(&cb_machine, &cb_energy, &cb_applicator, &cb_applicator_fitment,
                           &te_prescription_dose, &te_d2, &te_beam_mu, &te_ssd, &lbl_output_factor,
                           &lbl_applicator_cf, &lbl_expected_mu, &lbl_error);
    }));
}

fn callback_btn_compute(
    cb_machine: &gtk::ComboBoxText,
    cb_energy: &gtk::ComboBoxText,
    cb_applicator: &gtk::ComboBoxText,
) {
    let machine = cb_machine.get_selected_text();
    let energy = cb_energy.get_selected_text();
    let applicator = cb_applicator.get_selected_text();
    println!("machine: {}", machine);
    println!("energy: {}", energy);
    println!("applicator: {}", applicator);
}

fn callback_cb_machine(cb_machine: &gtk::ComboBoxText, cb_energy: &gtk::ComboBoxText) {
    let machine = cb_machine.get_selected_text();
    let energies = get_energies(&machine);
    cb_energy.remove_all();
    let n = energies.len();
    for energy in energies {
        // ComboBoxTextExt::
        cb_energy.append_text(&energy);
    }
    if n > 0 {
        cb_energy.set_active(Some(0));
    } else {
        cb_energy.set_active(None);
    }
    println!("callback_cb_machine");
}

fn callback_energy(
    cb_machine: &gtk::ComboBoxText,
    cb_energy: &gtk::ComboBoxText,
    cb_applicator: &gtk::ComboBoxText,
) {
    let machine = cb_machine.get_selected_text();
    let energy = cb_energy.get_selected_text();
    let apps = get_applicators(&machine, &energy);
    cb_applicator.remove_all();
    let n = apps.len();
    for app in apps {
        cb_applicator.append_text(app.as_str());
    }
    if n > 0 {
        cb_applicator.set_active(Some(0));
    } else {
        cb_applicator.set_active(None);
    }
    println!("callback_cb_energy");
}

fn callback_cb_applicator(
    cb_machine: &gtk::ComboBoxText,
    cb_energy: &gtk::ComboBoxText,
    cb_applicator: &gtk::ComboBoxText,
    cb_applicator_fitment: &gtk::ComboBoxText,
) {
    let machine = cb_machine.get_selected_text();
    let energy = cb_energy.get_selected_text();
    let applicator = cb_applicator.get_selected_text();
    let fitments = get_fitments(&machine, &energy, &applicator);
    let n = fitments.len();
    cb_applicator_fitment.remove_all();
    for fit in fitments {
        cb_applicator_fitment.append_text(fit.as_str());
    }
    if n > 0 {
        cb_applicator_fitment.set_active(Some(0));
    } else {
        cb_applicator_fitment.set_active(None);
    }
    println!("callback_cb_applicator");
}

fn callback_update(
    cb_machine: &gtk::ComboBoxText,
    cb_energy: &gtk::ComboBoxText,
    cb_applicator: &gtk::ComboBoxText,
    cb_applicator_fitment: &gtk::ComboBoxText,
    te_prescription_dose: &gtk::Entry,
    te_d2: &gtk::Entry,
    te_beam_mu: &gtk::Entry,
    te_ssd: &gtk::Entry,
    lbl_output_factor: &gtk::Label,
    lbl_applicator_cf: &gtk::Label,
    lbl_expected_mu: &gtk::Label,
    lbl_error: &gtk::Label,
) {
    let machine = cb_machine.get_selected_text();
    let energy = cb_energy.get_selected_text();
    let applicator = cb_applicator.get_selected_text();
    let fitment = cb_applicator_fitment.get_selected_text();
    let prescription_dose = te_prescription_dose.get_text().to_string();
    let d2 = te_d2.get_text().to_string();
    let beam_mu = te_beam_mu.get_text().to_string();
    let ssd = te_ssd.get_text().to_string();

    let res_update = update_results(
        &machine,
        &energy,
        &applicator,
        &fitment,
        &prescription_dose,
        &d2,
        &beam_mu,
        &ssd,
    );
    match res_update {
        Ok((outpuf_factor, applicator_cf, expected_mu)) => {
            lbl_output_factor.set_text(&outpuf_factor);
            lbl_applicator_cf.set_text(&applicator_cf);
            lbl_expected_mu.set_text(&expected_mu);
        }
        Err(msg) => {
            lbl_error.set_text(&msg);
        }
    }
}

fn main() {
    let application = gtk::Application::new(Some("org.tv.emu_check.ui"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}