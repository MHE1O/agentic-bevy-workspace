use bevy::prelude::Plugin;
use dev_diagnostics::DevDiagnosticsPlugin;

fn implements_plugin<T: Plugin>() {}

#[test]
fn diagnostics_exposes_a_bevy_plugin() {
    implements_plugin::<DevDiagnosticsPlugin>();
}
