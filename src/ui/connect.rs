use flume::Sender;

pub enum Event {
    MachineChanged(String),
    EnergyChanged(String),
    ApplicatorChanged(String),
    ApplicatorFitmentChanged(String),
    SSDChanged(String),
    BeamMUChanged(String),
    D2Changed(String),
    PrescriptionDoseChanged(String),
    AppQuit,
}

pub trait Connect<T> {
    fn connect_events(&self, sender: Sender<T>);
    fn connect_machine_changed(&self, sender: Sender<T>);
    fn connect_energy_changed(&self, sender: Sender<T>);
    fn connect_applicator_changed(&self, sender: Sender<T>);
    fn connect_applicator_fitment_changed(&self, sender: Sender<T>);
    fn connect_ssd_changed(&self, sender: Sender<T>);
    fn connect_beam_mu_changed(&self, sender: Sender<T>);
    fn connect_d2_changed(&self, sender: Sender<T>);
    fn connect_prescription_dose_changed(&self, sender: Sender<T>);
    fn connect_quit(&self, sender: Sender<T>);
}
