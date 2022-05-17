use opencv::{
    core::{no_array, Size, Vector, CV_8U, NORM_MINMAX},
    imgproc::LINE_8,
    prelude::*,
};

fn cbcr_channel() -> Vector<i32> {
    Vector::from_slice(&[1, 2])
}

fn cbcr_range() -> Vector<f32> {
    Vector::from_slice(&[0_f32, 255_f32])
}

pub fn inverse(src: &Mat) -> opencv::Result<Mat> {
    let mut dst = Mat::default();
    opencv::core::bitwise_not(&src, &mut dst, &opencv::core::no_array())?;

    Ok(dst)
}

pub fn cvt_color(src: &Mat, case: i32) -> opencv::Result<Mat> {
    let mut ret = Mat::default();
    opencv::imgproc::cvt_color(src, &mut ret, case, 0)?;
    Ok(ret)
}

pub fn ctoi(c: char) -> i8 {
    (c as u8) as i8
}

pub fn hist_mat_cb_cr(ycbcr: &Mat) -> opencv::Result<Mat> {
    let mut img: Vector<Mat> = Vector::new();
    img.push(ycbcr.clone());

    let hist_size = Vector::from_slice(&[128, 128]);
    let mut hist = unsafe { Mat::new_nd(0, &255, CV_8U)? };
    opencv::imgproc::calc_hist(
        &img,
        &cbcr_channel(),
        &opencv::core::no_array(),
        &mut hist,
        &hist_size,
        &cbcr_range(),
        false,
    )?;
    Ok(hist)
}

pub fn hist_mat_y(ycbcr: &Mat) -> opencv::Result<Mat> {
    let mut img: Vector<Mat> = Vector::new();
    img.push(ycbcr.clone());

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
    )?;
    Ok(hist)
}

pub fn draw_hist_y(hist: &Mat) -> opencv::Result<()> {
    let mut img =
        Mat::new_size_with_default(Size::new(256, 100), CV_8U, opencv::core::Scalar::all(255.0))
            .unwrap();

    for index in 0..255 {
        let mut max = 0_f64;
        opencv::core::min_max_loc(
            hist,
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
    Ok(())
}

pub fn normalize(src: &Mat) -> opencv::Result<Mat> {
    let mut ret = Mat::default();
    opencv::core::normalize(src, &mut ret, 0.0, 255.0, NORM_MINMAX, CV_8U, &no_array())?;
    Ok(ret)
}

pub fn back_proj_cb_cr(rgb: &Mat, hist: &Mat) -> opencv::Result<Mat> {
    let mut ret = Mat::default();
    opencv::imgproc::calc_back_project(rgb, &cbcr_channel(), hist, &mut ret, &cbcr_range(), 1_f64)?;
    Ok(ret)
}

pub fn copy_to(src: &Mat, mask: &Mat) -> opencv::Result<Mat> {
    let mut ret = Mat::default();
    opencv::core::copy_to(src, &mut ret, mask)?;
    Ok(ret)
}
