extern crate baseplug;

use serde::{Deserialize, Serialize};
use baseplug::event::*;
use baseplug::*;
use ringbuf::Rb;
use ringbuf::StaticRb;


baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct BaseplugMPETestParams {
        #[model(min = -90.0, max = 3.0)]
        #[parameter(name = "gain", unit = "Decibels",
            gradient = "Power(0.15)")]
        gain: f32
    }
}

impl Default for BaseplugMPETestParams {
    fn default() -> Self {
        Self {
            gain: 0.0
        }
    }
}

struct BaseplugMPETest {
    midi_queue: StaticRb::<Event<BaseplugMPETest>, 32>,
}

impl Plugin for BaseplugMPETest {
    const NAME: &'static str = "Launchpad JI";
    const PRODUCT: &'static str = "Launchpad JI";
    const VENDOR: &'static str = "Fractalysoft";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = BaseplugMPETestParams;

    #[inline]
    fn new(_sample_rate: f32, _model: &BaseplugMPETestParams) -> Self {
        
        Self {
            midi_queue: StaticRb::default(),
        }
    }

    // Do nothing to the audio
    #[inline]
    fn process(&mut self, model: &BaseplugMPETestParamsProcess, ctx: &mut ProcessContext<Self>) {

        // Send all midi events
        let enqueue_midi = &mut ctx.enqueue_event;
        while let Some(event) = self.midi_queue.pop() {
            enqueue_midi(event);
        }

        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..ctx.nframes {
            output[0][i] = input[0][i];
            output[1][i] = input[1][i];
        }
    }
}


impl MidiReceiver for BaseplugMPETest {
    fn midi_input(&mut self, _model: &BaseplugMPETestParamsProcess, msg: [u8; 3]) {
        match msg[0] {
            // note on
            0x90 => {
                let note_on = Event::<BaseplugMPETest> {
                    frame: 0,
                    data: Data::Midi([0x91, msg[1], msg[2]]),
                };
                self.midi_queue.push(note_on);
            },

            // note off
            0x80 => {
                let note_off = Event::<BaseplugMPETest> {
                    frame: 0,
                    data: Data::Midi([0x81, msg[1], msg[2]]),
                };
                self.midi_queue.push(note_off);
            },

            _ => ()
        }
    }
}

baseplug::vst2!(BaseplugMPETest, b"hhhh");