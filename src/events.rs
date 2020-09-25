#[derive(Clone, Debug)]
pub enum UIEvent {
    MachineChanged(String),
    EnergyChanged(f64),
    ApplicatorChange(String),
}
