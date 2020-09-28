use crate::ui::helper::set_margin;
use gtk::{Align, BoxExt, ButtonBoxExt, ButtonExt, ContainerExt, EntryExt, GridExt, WidgetExt};

#[derive(Clone)]
pub struct MainLayout {
    pub lbl_machine: gtk::Label,
    pub cb_machine: gtk::ComboBoxText,
    pub lbl_energy: gtk::Label,
    pub cb_energy: gtk::ComboBoxText,
    pub lbl_app: gtk::Label,
    pub cb_app: gtk::ComboBoxText,
    pub lbl_app_fitment: gtk::Label,
    pub cb_app_fitment: gtk::ComboBoxText,
    pub lbl_app_cf: gtk::Label,
    pub lbl_app_cf_result: gtk::Label,
    pub lbl_output_factor: gtk::Label,
    pub lbl_output_factor_result: gtk::Label,
    pub lbl_ssd: gtk::Label,
    pub entry_ssd: gtk::Entry,
    pub lbl_beam_mu: gtk::Label,
    pub entry_beam_mu: gtk::Entry,
    pub lbl_d2: gtk::Label,
    pub entry_d2: gtk::Entry,
    pub lbl_presc_dose: gtk::Label,
    pub entry_presc_dose: gtk::Entry,
    pub lbl_expected_mu: gtk::Label,
    pub lbl_expected_mu_result: gtk::Label,
    pub lbl_error: gtk::Label,
    pub lbl_error_result: gtk::Label,
    pub btn_compute: gtk::Button,
    pub btn_exit: gtk::Button,
    pub grid: gtk::Grid,
    pub center_box: gtk::Box,
    pub btn_box: gtk::ButtonBox,
}

impl Default for MainLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl MainLayout {
    pub fn new() -> Self {
        let grid = gtk::Grid::new();
        grid.set_visible(true);
        grid.set_row_spacing(5);
        grid.set_column_spacing(5);
        grid.set_row_homogeneous(true);
        grid.set_can_focus(false);
        set_margin(&grid, 5, 5, 5, 5);

        let lbl_machine = gtk::Label::new(Some("Machine"));
        lbl_machine.set_halign(Align::End);
        let cb_machine = gtk::ComboBoxText::new();

        let lbl_energy = gtk::Label::new(Some("Energy (MeV)"));
        lbl_energy.set_halign(Align::End);
        let cb_energy = gtk::ComboBoxText::new();

        let lbl_app = gtk::Label::new(Some("Applicator"));
        lbl_app.set_halign(Align::End);
        let cb_app = gtk::ComboBoxText::new();

        let lbl_app_fitment = gtk::Label::new(Some("Fitment"));
        lbl_app_fitment.set_halign(Align::End);
        let cb_app_fitment = gtk::ComboBoxText::new();

        let lbl_app_cf = gtk::Label::new(Some("Applicator correction factor"));
        lbl_app_cf.set_halign(Align::End);
        let lbl_app_cf_result = gtk::Label::new(None);

        let lbl_output_factor = gtk::Label::new(Some("Output factor"));
        lbl_output_factor.set_halign(Align::End);
        let lbl_output_factor_result = gtk::Label::new(None);

        let lbl_ssd = gtk::Label::new(Some("SSD (cm)"));
        lbl_ssd.set_halign(Align::End);
        let entry_ssd = gtk::Entry::new();
        entry_ssd.set_placeholder_text(Some("Source skin distance"));

        let lbl_beam_mu = gtk::Label::new(Some("Beam MU"));
        lbl_beam_mu.set_halign(Align::End);
        let entry_beam_mu = gtk::Entry::new();
        entry_beam_mu.set_placeholder_text(Option::from("MU's from a beam"));

        let lbl_d2 = gtk::Label::new(Some("D2 (cGy)"));
        lbl_d2.set_halign(Align::End);
        let entry_d2 = gtk::Entry::new();
        entry_d2.set_placeholder_text(Some("Dose at 2%"));

        let lbl_presc_dose = gtk::Label::new(Some("Prescription dose (cGy)"));
        lbl_presc_dose.set_halign(Align::End);
        let entry_presc_dose = gtk::Entry::new();
        entry_presc_dose.set_placeholder_text(Some("prescription dose"));

        let lbl_expected_mu = gtk::Label::new(Some("Expected MU"));
        lbl_expected_mu.set_halign(Align::End);
        let lbl_expected_mu_result = gtk::Label::new(None);

        let lbl_error = gtk::Label::new(Some("Error:"));
        lbl_error.set_halign(Align::End);
        lbl_error.set_visible(false);
        let lbl_error_result = gtk::Label::new(None);
        lbl_error_result.set_visible(false);

        let btn_compute = gtk::Button::new();
        btn_compute.set_label("Compute");

        let btn_exit = gtk::Button::new();
        btn_exit.set_label("Exit");

        grid.attach(&lbl_machine, 0, 0, 1, 1);
        grid.attach(&cb_machine, 1, 0, 1, 1);
        grid.attach(&lbl_energy, 0, 1, 1, 1);
        grid.attach(&cb_energy, 1, 1, 1, 1);
        grid.attach(&lbl_app, 0, 2, 1, 1);
        grid.attach(&cb_app, 1, 2, 1, 1);
        grid.attach(&lbl_app_fitment, 0, 3, 1, 1);
        grid.attach(&cb_app_fitment, 1, 3, 1, 1);
        grid.attach(&lbl_ssd, 0, 4, 1, 1);
        grid.attach(&entry_ssd, 1, 4, 1, 1);
        grid.attach(&lbl_beam_mu, 0, 5, 1, 1);
        grid.attach(&entry_beam_mu, 1, 5, 1, 1);
        grid.attach(&lbl_d2, 0, 6, 1, 1);
        grid.attach(&entry_d2, 1, 6, 1, 1);
        grid.attach(&lbl_presc_dose, 0, 7, 1, 1);
        grid.attach(&entry_presc_dose, 1, 7, 1, 1);
        grid.attach(&lbl_output_factor, 0, 8, 1, 1);
        grid.attach(&lbl_output_factor_result, 1, 8, 1, 1);
        grid.attach(&lbl_app_cf, 0, 9, 1, 1);
        grid.attach(&lbl_app_cf_result, 1, 9, 1, 1);
        grid.attach(&lbl_expected_mu, 0, 10, 1, 1);
        grid.attach(&lbl_expected_mu_result, 1, 10, 1, 1);

        let btn_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        btn_box.set_can_focus(false);
        btn_box.set_layout(gtk::ButtonBoxStyle::End);
        btn_box.set_property_expand(false);
        btn_box.add(&btn_compute);
        btn_box.add(&btn_exit);

        let center_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        set_margin(&center_box, 5, 5, 5, 5);
        center_box.set_spacing(5);
        center_box.add(&grid);
        center_box.add(&btn_box);

        Self {
            lbl_machine,
            cb_machine,
            lbl_energy,
            cb_energy,
            lbl_app,
            cb_app,
            lbl_app_fitment,
            cb_app_fitment,
            lbl_app_cf,
            lbl_app_cf_result,
            lbl_output_factor,
            lbl_output_factor_result,
            lbl_ssd,
            entry_ssd,
            lbl_beam_mu,
            entry_beam_mu,
            lbl_d2,
            entry_d2,
            lbl_presc_dose,
            entry_presc_dose,
            lbl_expected_mu,
            lbl_expected_mu_result,
            lbl_error,
            lbl_error_result,
            btn_compute,
            btn_exit,
            grid,
            center_box,
            btn_box,
        }
    }
}