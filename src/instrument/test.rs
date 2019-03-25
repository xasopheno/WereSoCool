pub mod tests {
    extern crate num_rational;
    extern crate socool_ast;
    use instrument::{
        asr::ASR,
        loudness::loudness_normalization,
        oscillator::{Basis, Oscillator},
        stereo_waveform::StereoWaveform,
        voice::{Voice, VoiceState, VoiceUpdate},
    };
    use num_rational::Rational64;
    use settings::get_test_settings;
    use socool_ast::{ast::OscType, operations::PointOp};
    pub mod voice {
        use super::*;
        #[test]
        fn test_voice_init() {
            let index = 1;
            let voice = Voice::init(index);

            let result = Voice {
                index,
                past: VoiceState {
                    frequency: 0.0,
                    gain: 0.0,
                },
                current: VoiceState {
                    frequency: 0.0,
                    gain: 0.0,
                },
                phase: 0.0,
                osc_type: OscType::Sine,
                asr: ASR::Silence,
                attack: 44100,
                decay: 44100,
                decay_length: 2,
            };

            assert_eq!(voice, result);
        }

        #[test]
        fn test_deltas() {
            let index = 1;
            let mut voice = Voice::init(index);
            let vu = VoiceUpdate {
                frequency: 200.0,
                gain: 1.0,
                osc_type: OscType::Sine,
                silence_next: true,
                attack: 44100.0,
                decay: 44100.0,
                decay_type: 2,
            };

            voice.update(vu);
            let gain = voice.calculate_asr_gain(10, 2);
            let p_delta = voice.calculate_portamento_delta(10);

            assert_eq!(gain, 0.13176251600253114);
            assert_eq!(p_delta, 20.0);
        }

        #[test]
        fn test_generate_waveform() {
            let index = 1;
            let mut buffer = vec![0.0; 3];
            let mut voice = Voice::init(index);
            let vu = VoiceUpdate {
                frequency: 100.0,
                gain: 1.0,
                osc_type: OscType::Sine,
                silence_next: true,
                attack: 44100.0,
                decay: 44100.0,
                decay_type: 2,
            };
            voice.update(vu);
            voice.generate_waveform(&mut buffer, 3, 2048.0 / 44_100.0);
            assert_eq!(buffer, [0.0, -0.33255392170798287, 0.09091323479014923]);
        }

        #[test]
        fn test_sound_silence() {
            let mut voice = Voice::init(1);
            let vu1 = VoiceUpdate {
                frequency: 100.0,
                gain: 1.0,
                osc_type: OscType::Sine,
                silence_next: true,
                attack: 44100.0,
                decay: 44100.0,
                decay_type: 2,
            };
            let vu2 = VoiceUpdate {
                frequency: 100.0,
                gain: 1.0,
                osc_type: OscType::Sine,
                silence_next: true,
                attack: 44100.0,
                decay: 44100.0,
                decay_type: 2,
            };
            voice.update(vu1);
            let silence_to_sound = voice.silence_to_sound();

            voice.update(vu2);
            let sound_to_silence = voice.sound_to_silence();

            assert_eq!(silence_to_sound, true);
            assert_eq!(sound_to_silence, false);
        }
    }

    pub mod oscillator {
        use super::*;
        #[test]
        fn oscillator_init_test() {
            let osc = Oscillator::init(&get_test_settings());
            let expected = Oscillator {
                portamento_length: 10,
                sample_phase: 0.0,
                settings: get_test_settings(),
                voices: (
                    Voice {
                        index: 0,
                        phase: 0.0,
                        past: VoiceState {
                            frequency: 0.0,
                            gain: 0.0,
                        },
                        current: VoiceState {
                            frequency: 0.0,
                            gain: 0.0,
                        },
                        osc_type: OscType::Sine,
                        asr: ASR::Silence,
                        attack: 44100,
                        decay: 44100,
                        decay_length: 2,
                    },
                    Voice {
                        index: 1,
                        phase: 0.0,
                        past: VoiceState {
                            frequency: 0.0,
                            gain: 0.0,
                        },
                        current: VoiceState {
                            frequency: 0.0,
                            gain: 0.0,
                        },
                        osc_type: OscType::Sine,
                        asr: ASR::Silence,
                        attack: 44100,
                        decay: 44100,
                        decay_length: 2,
                    },
                ),
            };
            assert_eq!(osc, expected);
        }

        #[test]
        fn oscillator_update_test() {
            let mut osc = Oscillator::init(&get_test_settings());

            let origin = Basis {
                f: 100.0,
                g: 1.0,
                l: 1.0,
                p: 0.0,
                a: 1.0,
                d: 1.0,
            };

            let mut point_op = PointOp::init();
            point_op.pa = Rational64::new(1, 2);

            osc.update(origin, &point_op, Some(PointOp::init()));

            assert_eq!(osc.voices.0.past.frequency, 0.0);
            assert_eq!(osc.voices.0.past.gain, 0.0);
            assert_eq!(osc.voices.0.current.frequency, 100.0);
            assert_eq!(osc.voices.0.current.gain, 0.75);
            assert_eq!(osc.voices.0.osc_type, OscType::Sine);
            //
            assert_eq!(osc.voices.1.past.frequency, 0.0);
            assert_eq!(osc.voices.1.past.gain, 0.0);
            assert_eq!(osc.voices.1.current.frequency, 100.0);
            assert_eq!(osc.voices.1.current.gain, 0.25);
            assert_eq!(osc.voices.1.osc_type, OscType::Sine);
        }
        #[test]
        fn oscillator_generate_sine_test() {
            let mut osc = Oscillator::init(&get_test_settings());

            let origin = Basis {
                f: 300.0,
                g: 1.0,
                l: 1.0,
                p: 0.0,
                a: 1.0,
                d: 1.0,
            };

            let mut point_op = PointOp::init();
            point_op.pa = Rational64::new(1, 2);

            osc.update(origin, &point_op, Some(PointOp::init()));

            let expected = StereoWaveform {
                l_buffer: vec![0.0, 0.01654001625028226, 0.033049819429038306],
                r_buffer: vec![0.0, 0.005513338750094087, 0.011016606476346103],
            };
            assert_eq!(osc.generate(3.0), expected);
        }
    }

    pub mod loudness {
        use super::*;
        #[test]
        fn test_loudness_normalization() {
            let expected = loudness_normalization(0.0);
            let result = 0.0;
            assert_eq!(expected, result);

            let expected = loudness_normalization(10.0);
            let result = 0.0;
            assert_eq!(expected, result);

            let expected = loudness_normalization(100.0);
            let result = 1.0;
            assert_eq!(expected, result);

            let expected = loudness_normalization(250.0);
            let result = 0.5759917219028009;
            assert_eq!(expected, result);

            let expected = loudness_normalization(500.0);
            let result = 0.3794705923727163;
            assert_eq!(expected, result);

            let expected = loudness_normalization(1000.0);
            let result = 0.25;
            assert_eq!(expected, result);

            let expected = loudness_normalization(1500.0);
            let result = 0.19584951787253815;
            assert_eq!(expected, result);
        }
    }
}
