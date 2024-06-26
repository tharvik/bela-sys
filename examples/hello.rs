//! Very basic program to test bindings of the Bela

use bela_sys::{BelaContext, BelaInitSettings};
use nix::sys::signal;
use std::{mem, ptr, slice, thread, time};

extern "C" fn render(context: *mut BelaContext, _user_data: *mut std::os::raw::c_void) {
    let context = unsafe { &mut *context };
    let n_frames = context.audioFrames;
    let n_channels = context.audioOutChannels;

    let audio_out: &mut [f32] =
        unsafe { slice::from_raw_parts_mut(context.audioOut, (n_frames * n_channels) as usize) };

    let frame_index = context.audioFramesElapsed;

    let len = audio_out.len();
    for (idx, samp) in audio_out.iter_mut().enumerate() {
        *samp = (idx as f32 / len as f32) * (frame_index % 5) as f32;
    }
}

extern "C" fn signal_handler(_signal: std::os::raw::c_int) {
    unsafe {
        bela_sys::Bela_requestStop();
    }
}

fn main() {
    unsafe {
        let mut settings = {
            let mut settings: mem::MaybeUninit<BelaInitSettings> = mem::MaybeUninit::uninit();
            bela_sys::Bela_defaultSettings(settings.as_mut_ptr());
            settings.assume_init()
        };
        bela_sys::Bela_setVerboseLevel(1);
        settings.render = Some(render);
        settings.verbose = 1;
        settings.highPerformanceMode = 1;
        settings.analogOutputsPersist = 0;
        settings.uniformSampleRate = 1;

        if bela_sys::Bela_initAudio(&mut settings, ptr::null_mut()) != 0 {
            println!("settings.render: {:?}", &settings.render);
            panic!("lol fail");
        }

        if bela_sys::Bela_startAudio() != 0 {
            println!("Unable to start real-time audio");
            bela_sys::Bela_stopAudio();
            bela_sys::Bela_cleanupAudio();
            panic!("Aborting");
        }

        let handler = signal::SigHandler::Handler(signal_handler);
        let flags = signal::SaFlags::empty();
        let set = signal::SigSet::empty();
        let sig_action = signal::SigAction::new(handler, flags, set);

        signal::sigaction(signal::SIGINT, &sig_action).unwrap();
        signal::sigaction(signal::SIGTERM, &sig_action).unwrap();

        while bela_sys::Bela_stopRequested() == 0 {
            thread::sleep(time::Duration::new(1, 0));
        }

        bela_sys::Bela_stopAudio();
        bela_sys::Bela_cleanupAudio();
    }
}
