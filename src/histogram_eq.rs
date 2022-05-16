use opencv::{
    core::{Size, Vector, CV_8U},
    highgui,
    imgcodecs::IMREAD_COLOR,
    imgproc::{COLOR_BGR2YCrCb, COLOR_YCrCb2BGR, LINE_8},
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
    let hist_mat = hist_mat(src)?;
    highgui::imshow(window, &hist_mat)?;
    Ok(())
}

fn hist_mat(src: &Mat) -> opencv::Result<Mat> {
    let mut img: Vector<Mat> = Vector::new();
    img.push(src.clone());

    let channels = Vector::from_slice(&[0]);
    let hist_size = Vector::from_slice(&[256]);
    let ranges = Vector::from_slice(&[0_f32, 255_f32]);
    let mut hist = unsafe { Mat::new_nd(0, &255, CV_8U)? };
    opencv::imgproc::calc_hist(
        &img,
        &channels,
        &opencv::core::no_array(),
        &mut hist,
        &hist_size,
        &ranges,
        false,
    )
    .unwrap();

    draw_hist(hist)
}

fn draw_hist(hist: Mat) -> opencv::Result<Mat> {
    let mut img =
        Mat::new_size_with_default(Size::new(256, 100), CV_8U, opencv::core::Scalar::all(255.0))
            .unwrap();

    for index in 0..255 {
        let mut max = 0_f64;
        opencv::core::min_max_loc(
            &hist,
            None,
            Some(&mut max),
            None,
            None,
            &opencv::core::no_array(),
        )?;
        let hist = hist.at::<f32>(index)?;
        opencv::imgproc::line(
            &mut img,
            opencv::core::Point::new(index, 100),
            opencv::core::Point::new(index, (100_f32 - hist * 100_f32 / max as f32) as i32),
            opencv::core::Scalar::all(0.0),
            1,
            LINE_8,
            0,
        )?;
    }
    Ok(img)
}
