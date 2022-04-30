use opencv::{
    core::{Range, Vector},
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs::{IMREAD_COLOR, IMREAD_UNCHANGED},
    prelude::*,
};

fn run() -> opencv::Result<()> {
    let window1 = "src";
    let window2 = "mask";
    let window3 = "with_field";

    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;

    let src = opencv::imgcodecs::imread("opencv-logo-white.png", IMREAD_UNCHANGED)?;
    let mut layers = Vector::<Mat>::new();
    opencv::core::split(&src, &mut layers)?;
    let mask = layers.get(3)?;
    layers.remove(3)?;
    let mut rgb = Mat::default();
    opencv::core::merge(&layers, &mut rgb)?;
    let src = rgb;

    highgui::imshow(window1, &src)?;
    highgui::imshow(window2, &mask)?;

    let dst = opencv::imgcodecs::imread("field.bmp", IMREAD_COLOR)?;
    let size = src.size()?;
    let (y, x) = (10, 10);
    let mut crop = Mat::ranges(
        &dst,
        &Vector::from_iter([
            Range::new(y, size.height + y)?,
            Range::new(x, size.width + x)?,
        ]),
    )?;

    opencv::core::copy_to(&src, &mut crop, &mask)?;
    highgui::imshow(window3, &dst)?;
    highgui::wait_key(0)?;

    Ok(())
}

fn main() {
    run().unwrap()
}
