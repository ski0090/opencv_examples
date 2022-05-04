use opencv::{
    highgui::{destroy_all_windows, imshow, wait_key},
    imgproc::canny,
    prelude::*,
    videoio::VideoCapture,
};
const ESCAPE: i32 = 27;
const SRC_PATH: &'static str = "video1.mp4";

fn main() -> opencv::Result<()> {
    let mut capture = VideoCapture::from_file(SRC_PATH, 0)?;
    if !capture.is_opened()? {
        println!("camera open failed");
        return Ok(());
    }

    loop {
        let mut image = Mat::default();
        if !capture.read(&mut image)? {
            break;
        }
        let mut edge = Mat::default();
        canny(&image, &mut edge, 50_f64, 150_f64, 3, false)?;

        imshow("frame", &image)?;
        imshow("edge", &edge)?;

        if wait_key(20)? == ESCAPE {
            break;
        }
    }
    capture.release()?;
    destroy_all_windows()?;
    Ok(())
}
