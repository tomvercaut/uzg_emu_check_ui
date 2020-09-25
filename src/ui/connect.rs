use flume::Sender;

pub enum Event {
    MachineChanged(String),
    EnergyChanged(f64),
    ApplicatorChanged(String),
}

pub trait Connect<T> {
    fn connect_events(&self, sender: Sender<T>);
    fn connect_compute(&self, sender: Sender<T>);
}
