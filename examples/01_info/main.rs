use std::sync::Mutex;

use opencv::{
    core::{Point, Scalar, Vec3b, CV_8UC1, CV_8UC3},
    highgui::{self, set_mouse_callback, WINDOW_AUTOSIZE},
    imgcodecs::IMREAD_UNCHANGED,
    imgproc::{FONT_HERSHEY_PLAIN, LINE_8},
    prelude::*,
};

fn main() -> opencv::Result<()> {
    let window1 = "img1";
    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    let mut img1 = opencv::imgcodecs::imread("resources/cat.bmp", IMREAD_UNCHANGED)?;
    print_info(&mut img1);
    highgui::imshow(window1, &img1).unwrap();

    let img1 = Mutex::new(img1);

    set_mouse_callback(
        window1,
        Some(Box::new(move |_, x, y, _| {
            if let Ok(img1) = img1.lock() {
                if let Ok(pixel) = img1.at_2d::<Vec3b>(x, y) {
                    let info = format!("B: {}, G: {}, R: {}", pixel[0], pixel[1], pixel[2]);
                    let mut img1 = img1.clone();
                    put_text(&mut img1, &info, 3);
                    highgui::imshow(window1, &img1).unwrap();
                }
            }
        })),
    )?;
    highgui::wait_key(0)?;
    Ok(())
}

fn print_info(mat: &mut Mat) {
    let mat_type = match mat.typ() {
        CV_8UC1 => "type: CV_8UC1".to_string(),
        CV_8UC3 => "type: CV_8UC3".to_string(),
        _ => todo!(),
    };
    put_text(mat, &mat_type, 0);

    let size = mat.size().unwrap();
    put_text(mat, &format!("Size {:?}", size), 1);

    let depth = mat.elem_size().unwrap();
    put_text(mat, &format!("Depths: {}", depth), 2);
}

fn put_text(mat: &mut Mat, text: &str, line: i32) {
    const LINE_GAP: i32 = 20;
    opencv::imgproc::put_text(
        mat,
        text,
        Point::new(3, LINE_GAP + (LINE_GAP * line)),
        FONT_HERSHEY_PLAIN,
        1.2,
        Scalar::all(0_f64),
        1,
        LINE_8,
        false,
    )
    .unwrap();
}
