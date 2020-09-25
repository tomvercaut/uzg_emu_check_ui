use gtk;
use gtk::prelude::*;

pub(crate) fn set_margin<W: gtk::WidgetExt>(widget: &W, t: i32, r: i32, b: i32, l: i32) {
    widget.set_margin_top(t);
    widget.set_margin_end(t);
    widget.set_margin_bottom(t);
    widget.set_margin_start(t);
}
