use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_bexpand::BexpandPlugin;

fn main() {
    serve_plugin(&BexpandPlugin, MsgPackSerializer)
}
