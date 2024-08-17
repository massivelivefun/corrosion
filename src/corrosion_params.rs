use nih_plug::params::{FloatParam, Params};
use nih_plug::prelude::{FloatRange, SmoothingStyle};
use nih_plug::{formatters, util};

#[derive(Params)]
pub struct CorrosionParams {
    #[id = "distortion"]
    pub distortion: FloatParam,
}

impl Default for CorrosionParams {
    fn default() -> Self {
        Self {
            distortion: FloatParam::new(
                "distortion",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0)
                })
                .with_smoother(SmoothingStyle::Logarithmic(30.0))
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db())
        }
    }
}
