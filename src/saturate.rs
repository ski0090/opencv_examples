use opencv::{
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs::IMREAD_UNCHANGED,
    prelude::*,
};

fn main() {
    saturate_scalar().unwrap()
}

fn saturate_scalar() -> opencv::Result<()> {
    let window1 = "img1";
    let window2 = "saturate";
    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;
    let src1 = opencv::imgcodecs::imread("resources/cat.bmp", IMREAD_UNCHANGED)?;
    let src2 = opencv::core::Scalar::all(55_f64);
    let mut dst = Mat::default();
    opencv::core::add(&src1, &src2, &mut dst, &opencv::core::no_array(), -1)?;

    highgui::imshow(window1, &src1)?;
    highgui::imshow(window2, &dst)?;

    highgui::wait_key(0)?;
    Ok(())
}
