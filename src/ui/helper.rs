pub(crate) fn set_margin<W: gtk::WidgetExt>(widget: &W, t: i32, r: i32, b: i32, l: i32) {
    widget.set_margin_top(t);
    widget.set_margin_end(r);
    widget.set_margin_bottom(b);
    widget.set_margin_start(l);
}
