use opencv::{
    core::Range,
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs::IMREAD_UNCHANGED,
    prelude::*,
};

fn run() -> opencv::Result<()> {
    let window1 = "img1";
    let window2 = "img2";
    let window3 = "img3";
    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;
    highgui::named_window(window3, WINDOW_AUTOSIZE)?;
    let img1 = opencv::imgcodecs::imread("cat.bmp", IMREAD_UNCHANGED)?;

    let range_row = Range::new(40, 120)?;
    let range_col = Range::new(30, 150)?;

    // reference
    let mut img2 = Mat::rowscols(&img1, &range_row, &range_col)?;
    // value
    #[allow(clippy::redundant_clone)]
    let mut img3 = Mat::rowscols(&img1, &range_row, &range_col)?.clone();
    img2.set(opencv::core::Scalar::new(0_f64, 255_f64, 0_f64, 0_f64))?;
    img3.set(opencv::core::Scalar::new(255_f64, 0_f64, 0_f64, 0_f64))?;
    highgui::imshow(window1, &img1)?;
    highgui::imshow(window2, &img2)?;
    highgui::imshow(window3, &img3)?;

    highgui::wait_key(0)?;

    Ok(())
}

fn main() {
    run().unwrap()
}
