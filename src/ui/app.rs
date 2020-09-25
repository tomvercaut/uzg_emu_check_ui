use crate::ui::connect::Event;
use gtk;
use gtk::prelude::*;

pub struct AppState {
    pub title: String,
    pub tx: flume::Sender<Event>,
}

pub struct App {
    pub window: gtk::Window,
    pub state: AppState,
}

impl App {
    pub fn new(state: AppState) -> Self {
        // let window = cascade! {
        //     gtk::Window::new(gtk::WindowType::Toplevel);
        //     ..set_title(state.title.as_str());
        //     ..set_default_size(600,800);
        //
        // };

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title(state.title.as_str());
        window.set_default_size(600, 800);
       //TODO continue here
        Self {
            window,
            state
        }
    }
}
