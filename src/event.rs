use opencv::{
    core::{Size, CV_8U},
    highgui::{destroy_all_windows, imshow, set_mouse_callback, wait_key, EVENT_LBUTTONDOWN},
    prelude::*,
};
const ESCAPE: i32 = 27;

fn main() -> opencv::Result<()> {
    let mat = Mat::ones_size(Size::new(640, 480), CV_8U)?;
    imshow("image", &mat)?;
    set_mouse_callback("image", Some(Box::new(on_mouse)))?;
    while wait_key(0)? != ESCAPE {}
    destroy_all_windows()?;
    Ok(())
}

fn on_mouse(event: i32, x: i32, y: i32, param: i32) {
    if event == EVENT_LBUTTONDOWN {
        dbg!(x, y);
    }
}
