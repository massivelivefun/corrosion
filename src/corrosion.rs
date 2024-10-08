use std::num::NonZeroU32;
use std::sync::Arc;
use nih_plug::audio_setup::{AudioIOLayout, AuxiliaryBuffers, BufferConfig, PortNames};
use nih_plug::buffer::Buffer;
use nih_plug::midi::MidiConfig;
use nih_plug::params::Params;
use nih_plug::plugin::{Plugin, ProcessStatus};
use nih_plug::prelude::{ClapFeature, ClapPlugin, InitContext, ProcessContext, Vst3Plugin, Vst3SubCategory};

use crate::corrosion_params::CorrosionParams;

pub const MAX_BLOCK_SIZE: usize = 64;

pub struct Corrosion {
    params: Arc<CorrosionParams>,
}

impl Default for Corrosion {
    fn default() -> Self {
        Self {
            params: Arc::new(CorrosionParams::default())
        }
    }
}

impl Plugin for Corrosion {
    const NAME: &'static str = "Corrosion";
    const VENDOR: &'static str = "Matthew Lance Fuller";
    const URL: &'static str = "https://massivelive.fun/corrosion";
    const EMAIL: &'static str = "matthewlancefuller@gmail.com";

    const VERSION: &'static str = "0.0.1";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        // self.gain(buffer)
        self.distortion(buffer)
    }

    fn deactivate(&mut self) {}
}

impl Corrosion {
    // fn gain(&mut self, buffer: &mut Buffer) -> ProcessStatus {
    //     for channel_samples in buffer.iter_samples() {
    //         // Smoothing is optionally built into the parameters themselves
    //         let gain = self.params.gain.smoothed.next();
    //
    //         for sample in channel_samples {
    //             *sample *= gain;
    //         }
    //     }
    //
    //     ProcessStatus::Normal
    // }

    fn distortion(&mut self, buffer: &mut Buffer) -> ProcessStatus {
        let mut cloned_buffer: Vec<Vec<f32>> = Vec::with_capacity(buffer.channels());

        for channel in buffer.as_slice_immutable().iter() {
            cloned_buffer.push((**channel).iter().cloned().collect::<Vec<f32>>());
        }

        for (cloned_channel, channel) in cloned_buffer.iter().zip(buffer.iter_samples()) {
            for (cloned_sample, sample) in cloned_channel.iter().zip(channel) {
                let smoothed_threshold = self.params.threshold.smoothed.next();
                if *cloned_sample >= 0.0_f32 {
                    *sample = cloned_sample.min(smoothed_threshold) / smoothed_threshold;
                } else {
                    *sample = cloned_sample.max(-smoothed_threshold) / smoothed_threshold;
                }
            }
        }

        ProcessStatus::Normal
    }

    fn glitch(&mut self, buffer: &mut Buffer) -> ProcessStatus {
        // let num_samples = buffer.samples();
        // let output = buffer.as_slice();

        // let mut next_event = context.next_event();
        // let mut block_start: usize = 0;
        // let mut block_end: usize = MAX_BLOCK_SIZE.min(num_samples);

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Corrosion {
    const CLAP_ID: &'static str = "com.massivelivefun.distortion";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Distorts audio on your track.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for Corrosion {
    const VST3_CLASS_ID: [u8; 16] = *b"CorrosionPlugin!";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}
