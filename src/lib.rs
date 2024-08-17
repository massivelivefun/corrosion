mod corrosion;
mod corrosion_params;

#[macro_use]
extern crate nih_plug;

use crate::corrosion::Corrosion;

nih_export_clap!(Corrosion);
nih_export_vst3!(Corrosion);
