use emu_check::EmuError;
use glib::BoolError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unable to launch GTK application: {0}")]
    GuiLaunch(BoolError),
    #[error("Core error: {0}")]
    CoreError(EmuError),
    #[error("GLib error: {0}")]
    GLibError(glib::Error),
    #[error("GTK object not found")]
    GtkObjectNotFound,
}
