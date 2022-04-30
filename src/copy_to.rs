use opencv::{
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs::{IMREAD_COLOR, IMREAD_GRAYSCALE},
};

fn run() -> opencv::Result<()> {
    let window1 = "src";
    let window2 = "dst";
    let window3 = "mask";
    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;
    let src = opencv::imgcodecs::imread("airplane.bmp", IMREAD_COLOR)?;
    let mut dst = opencv::imgcodecs::imread("field.bmp", IMREAD_COLOR)?;
    let mask = opencv::imgcodecs::imread("mask_plane.bmp", IMREAD_GRAYSCALE)?;

    // src, mask, dst is same range.
    // src, dst is same type.
    // mask is grayscale.
    opencv::core::copy_to(&src, &mut dst, &mask)?;

    highgui::imshow(window1, &src)?;
    highgui::imshow(window2, &dst)?;
    highgui::imshow(window3, &mask)?;

    highgui::wait_key(0)?;
    Ok(())
}

fn main() {
    run().unwrap()
}
