// #![windows_subsystem = "windows"]

// use async_std::prelude::*;
// use gio::prelude::*;
// use gtk::prelude::*;

use clap::{crate_authors, crate_description, crate_version, Arg};
use emu_check::correction_data_set_load_data;
use emu_check_ui::ui::{App, AppState, Connect, Event};
use log::{error, trace, LevelFilter};
use simple_logger::SimpleLogger;
use std::process::exit;

#[async_std::main]
async fn main() {
    // simple_logger::init_with_level(Level::Trace).unwrap();
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();
    println!("EMU check UI");
    println!("------------");
    let opt_dir_default = dirs::data_local_dir();
    if opt_dir_default.is_none() {
        error!("Unable to determine the local data directory for the current user.");
        exit(1);
    }
    let mut pb_dir_default = opt_dir_default.unwrap();
    pb_dir_default.push("emu_check_ui");
    let opt_str_dir_default = pb_dir_default.to_str();
    let matches = clap::App::new("emu_check_ui")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("dir")
                .help(
                    "Directory containing the outputfactors and \
                field defining apertures per energy. \
                Each applicator has a seperate csv file for the \
                output factors and field defining apertures.",
                )
                .index(1)
                .required(false)
                .default_value(opt_str_dir_default.unwrap()),
        )
        .get_matches();
    let dirname = matches.value_of("dir").unwrap().to_string();
    trace!("dirname: {}", dirname.as_str());

    // Load the correction data set in while constructing the rest of the UI
    let fut_cds = correction_data_set_load_data(dirname.as_str());

    // New approach
    let app_title = "Electron MU Check".to_string();
    glib::set_program_name(Some(app_title.as_str()));
    glib::set_application_name(app_title.as_str());

    // Init GTK befor proceeding
    if let Err(e) = gtk::init() {
        error!("Failed to initialise GTK application:\n{}", e);
        exit(1);
    }

    // Create channels on which events will be send.
    let (tx, rx) = flume::unbounded();

    // Wait for the correction data set to be loaded.
    let res_cds = fut_cds.await;
    if let Err(e) = res_cds {
        error!("Failed to initialise the application:\n{}", e);
        exit(1);
    }

    // Initialise the GTK application and widgets
    let app = App::new(AppState {
        title: app_title,
        data: res_cds.unwrap(),
        tx: tx.clone(),
    });

    // Async event loop for handling all application events
    let event_handler = async move {
        app.connect_events(tx);
        app.show();

        while let Ok(event) = rx.recv_async().await {
            match event {
                Event::MachineChanged(name) => {
                    app.machine_changed(name.as_str());
                }
                Event::EnergyChanged(energy) => {
                    app.energy_changed(energy.as_str());
                }
                Event::ApplicatorChanged(applicator) => {
                    app.applicator_changed(applicator.as_str());
                }
                Event::ApplicatorFitmentChanged(fitment) => {
                    app.applicator_fitment_changed(fitment.as_str());
                }
                Event::SSDChanged(ssd) => {
                    app.ssd_changed(ssd.as_str());
                }
                Event::BeamMUChanged(mu) => {
                    app.beam_mu_changed(mu.as_str());
                }
                Event::D2Changed(d2) => {
                    app.d2_changed(d2.as_str());
                }
                Event::PrescriptionDoseChanged(presc_dose) => {
                    app.prescription_dose_changed(presc_dose.as_str());
                }
                Event::AppQuit => {
                    app.quit();
                }
            }
        }
    };

    glib::MainContext::default().spawn_local(event_handler);

    // Start the main event loop of GTK, which will display the UI
    // and handle the actions that were connected.
    gtk::main();
}
