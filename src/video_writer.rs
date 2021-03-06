use cw::*;
use opencv::{
    core::Size,
    highgui::{destroy_all_windows, imshow, wait_key},
    prelude::*,
    videoio::{
        VideoCapture, VideoWriter, CAP_PROP_FPS, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH,
    },
};
const ESCAPE: i32 = 27;
const SRC_PATH: &'static str = "resources/video1.mp4";
const OUT_PATH: &'static str = "resources/out.avi";

fn main() -> opencv::Result<()> {
    let mut capture = VideoCapture::from_file(SRC_PATH, 0)?;
    if !capture.is_opened()? {
        println!("camera open failed");
        return Ok(());
    }

    let w = capture.get(CAP_PROP_FRAME_WIDTH)?.round() as i32;
    let h = capture.get(CAP_PROP_FRAME_HEIGHT)?.round() as i32;
    let fps = capture.get(CAP_PROP_FPS)?.round();

    // reference https://www.fourcc.org/divx/
    let fourcc = VideoWriter::fourcc(ctoi('D'), ctoi('I'), ctoi('V'), ctoi('X'))?;
    let delay = 1000_f64 / fps.round();

    let mut out = VideoWriter::new(OUT_PATH, fourcc, 30_f64, Size::new(w, h), true)?;

    if !out.is_opened()? {
        capture.release()?;
        println!("out.avi open failed!");
        return Ok(());
    }

    loop {
        let mut image = Mat::default();
        if !capture.read(&mut image)? {
            break;
        }
        let inv = inverse(&image)?;
        out.write(&image)?;
        imshow("frame", &image)?;
        imshow("inv", &inv)?;

        if wait_key(delay as i32)? == ESCAPE {
            break;
        }
    }
    capture.release()?;
    destroy_all_windows()?;
    Ok(())
}
