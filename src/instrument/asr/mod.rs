use socool_ast;
use instrument::voice::Voice;

#[derive(Clone, Debug, PartialEq)]
pub enum ASR {
    ASR,
    AS,
    S,
    SR,
    R,
    Silence,
}

impl Voice {
    pub fn set_asr(&mut self, silence_next: bool, _decay_length: usize, silence_now: bool) {
        let long = if self.decay_length == 2 { true } else { false };
        if self.silent() && !long {
            self.asr = ASR::Silence;
        } else {
            match self.asr {
                ASR::Silence | ASR::ASR | ASR::SR | ASR::R => {
                    if silence_next && !long {
                        self.asr = ASR::ASR;
                    } else {
                        if silence_now {
                            self.asr = ASR::Silence
                        } else {
                            self.asr = ASR::AS;
                        }
                    }
                }

                ASR::AS | ASR::S => {
                    if self.sound_to_silence() {
                        if long {
                            self.asr = ASR::R
                        } else {
                            self.asr = ASR::Silence
                        }
                    } else if silence_next {
                        if long {
                            self.asr = ASR::S
                        } else {
                            self.asr = ASR::SR
                        }
                    } else {
                        self.asr = ASR::S
                    }
                }
            }
        }
    }

    pub fn calculate_asr_gain(&mut self, buffer_len: usize, index: usize) -> f64 {
        let short = self.is_short(buffer_len);

        match self.asr {
            ASR::Silence => {
                return 0.0;
            }
            ASR::AS => {
                let len = if short { buffer_len } else { self.attack };
                let distance = self.current.gain - self.past.gain;
                if index <= len {
                    return self.calculate_attack(distance, index, len);
                } else {
                    return self.current.gain;
                }
            }
            ASR::S => {
                return self.past.gain
                    + ((self.current.gain - self.past.gain) * index as f64 / buffer_len as f64);
            }
            ASR::ASR => {
                let mut attack_length = self.attack;
                let mut decay_length = self.decay;

                if short {
                    attack_length = buffer_len / 2;
                    decay_length = buffer_len / 2;
                }
                if index <= attack_length {
                    let distance = self.current.gain - self.past.gain;
                    return self.calculate_attack(distance, index, attack_length);
                } else if index > buffer_len - decay_length {
                    let decay_index = buffer_len - (index + 1);
                    self.calculate_decay(self.current.gain, decay_index, decay_length)
                } else {
                    return self.current.gain;
                }
            }
            ASR::SR => {
                if short {
                    let decay_index = buffer_len - (index + 1);
                    return self.calculate_decay(self.past.gain, decay_index, buffer_len);
                };

                let len = buffer_len - self.decay;
                if index < len {
                    let distance = self.current.gain - self.past.gain;
                    let result = self.past.gain + (distance * (index as f64 / len as f64));
                    return result;
                } else {
                    let decay_index = buffer_len - (index + 1);
                    return self.calculate_decay(self.current.gain, decay_index, self.decay);
                };
            }

            ASR::R => {
                if short {
                    let decay_index = buffer_len - (index + 1);
                    return self.calculate_decay(self.past.gain, decay_index, buffer_len);
                };

                if index < self.decay {
                    let decay_index = self.decay - (index + 1);
                    return self.calculate_decay(self.past.gain, decay_index, self.decay);
                } else {
                    return 0.0;
                };
            }
        }
    }
}
