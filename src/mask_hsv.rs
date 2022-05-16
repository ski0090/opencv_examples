use opencv::{
    core::Scalar,
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs::IMREAD_UNCHANGED,
    imgproc::COLOR_BGR2HSV,
    prelude::*,
};

const BRIGHT: &'static str = "resources/candies.png";
const DARK: &'static str = "resources/candies2.png";

fn mask_hsv(path: &str) -> opencv::Result<()> {
    let window1 = "src";
    let window2 = "rgb_mask";
    let window3 = "hsv_mask";

    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;

    let src = opencv::imgcodecs::imread(path, IMREAD_UNCHANGED)?;

    // https://en.wikipedia.org/wiki/HSL_and_HSV
    let mut hsv = Mat::default();

    opencv::imgproc::cvt_color(&src, &mut hsv, COLOR_BGR2HSV, 0)?;

    let rgb_mask = in_range(&src, (0, 128, 0), (100, 255, 100));
    let hsv_mask = in_range(&hsv, (50, 150, 0), (80, 255, 255));

    highgui::imshow(window1, &src).unwrap();
    highgui::imshow(window2, &rgb_mask).unwrap();
    highgui::imshow(window3, &hsv_mask).unwrap();

    highgui::wait_key(0)?;

    Ok(())
}

fn in_range(src: &Mat, low: (u32, u32, u32), high: (u32, u32, u32)) -> Mat {
    let mut ret = Mat::default();
    let low = Scalar::from((low.0 as f64, low.1 as f64, low.2 as f64));
    let high = Scalar::from((high.0 as f64, high.1 as f64, high.2 as f64));
    opencv::core::in_range(src, &low, &high, &mut ret).unwrap();
    ret
}

fn main() {
    mask_hsv(BRIGHT).unwrap();
    mask_hsv(DARK).unwrap()
}
