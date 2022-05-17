use cw::{back_proj_cb_cr, hist_mat_cb_cr, normalize};
use opencv::{
    core::Range,
    highgui::{select_roi, wait_key},
    imgcodecs::IMREAD_UNCHANGED,
    imgproc::COLOR_BGR2YCrCb,
    prelude::*,
};

fn main() -> opencv::Result<()> {
    let src = opencv::imgcodecs::imread("resources/cropland.png", IMREAD_UNCHANGED)?;
    let crop = select_roi(&src, true, false)?;
    let src_ycrcb = cw::cvt_color(&src, COLOR_BGR2YCrCb)?;
    let y_range = Range::new(crop.y, crop.y + crop.height)?;
    let x_range = Range::new(crop.x, crop.x + crop.width)?;
    let crop = Mat::rowscols(&src_ycrcb, &y_range, &x_range)?.clone();

    let hist = hist_mat_cb_cr(&crop)?;
    let hist_norm = normalize(&hist)?;

    let back_proj = back_proj_cb_cr(&src, &hist)?;
    let dst = cw::copy_to(&src, &back_proj)?;

    opencv::highgui::imshow("backprog", &back_proj)?;
    opencv::highgui::imshow("hist_norm", &hist_norm)?;
    opencv::highgui::imshow("dst", &dst)?;
    wait_key(0)?;

    Ok(())
}
