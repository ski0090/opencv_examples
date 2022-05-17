use cw::{draw_hist_y, hist_mat_y};
use opencv::{
    core::Vector,
    highgui,
    imgcodecs::IMREAD_COLOR,
    imgproc::{COLOR_BGR2YCrCb, COLOR_YCrCb2BGR},
    prelude::*,
};

const SRC: &'static str = "src";
const SRC_HIST: &'static str = "src_hist";
const STRETCH: &'static str = "stretch";
const STRETCH_HIST: &'static str = "stretch_hist";

fn main() {
    run().unwrap()
}

fn run() -> opencv::Result<()> {
    let src = opencv::imgcodecs::imread("resources/field.bmp", IMREAD_COLOR)?;
    let mut src_ycrcb = Mat::default();
    opencv::imgproc::cvt_color(&src, &mut src_ycrcb, COLOR_BGR2YCrCb, 0)?;
    let mut plains: Vector<Mat> = Vector::new();
    opencv::core::split(&src_ycrcb, &mut plains)?;

    let mut dst = Mat::default();
    opencv::imgproc::equalize_hist(&plains.get(0)?, &mut dst)?;
    plains.set(0, dst)?;
    let mut ycbcr = Mat::default();
    opencv::core::merge(&plains, &mut ycbcr)?;
    let mut result = Mat::default();
    opencv::imgproc::cvt_color(&ycbcr, &mut result, COLOR_YCrCb2BGR, 0)?;

    highgui::imshow(SRC, &src)?;
    show_histogram(SRC_HIST, &src).unwrap();
    highgui::imshow(STRETCH, &result)?;
    show_histogram(STRETCH_HIST, &result)?;
    highgui::wait_key(0)?;

    Ok(())
}

fn show_histogram(window: &str, src: &Mat) -> opencv::Result<()> {
    let hist_mat = hist_mat_y(src)?;
    draw_hist_y(&hist_mat)?;
    highgui::imshow(window, &hist_mat)?;
    Ok(())
}
