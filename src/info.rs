use opencv::{
    core::{Vec3b, CV_8UC1, CV_8UC3},
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs::{IMREAD_GRAYSCALE, IMREAD_UNCHANGED},
    prelude::*,
};

fn run() -> opencv::Result<()> {
    let window1 = "img1";
    let window2 = "img2";
    highgui::named_window(window1, WINDOW_AUTOSIZE)?;
    highgui::named_window(window2, WINDOW_AUTOSIZE)?;
    let mut img1 = opencv::imgcodecs::imread("cat.bmp", IMREAD_UNCHANGED)?;
    let img2 = opencv::imgcodecs::imread("cat.bmp", IMREAD_GRAYSCALE)?;
    let x = 20;
    let y = 10;

    assert_eq!(img1.typ(), CV_8UC3);
    assert_eq!(img1.elem_size()?, 3);

    dbg!(img1.size())?; // get size

    let pixel = img1.at_2d_mut::<Vec3b>(y, x)?;
    println!("B: {}, G: {}, R: {}", pixel[0], pixel[1], pixel[2]);
    pixel[0] = 255;
    pixel[1] = 0;
    pixel[2] = 0;
    assert_eq!(img2.typ(), CV_8UC1);
    assert_eq!(img2.elem_size()?, 1);

    highgui::imshow(window1, &img1)?;
    highgui::imshow(window2, &img2)?;
    highgui::wait_key(0)?;
    Ok(())
}

fn main() {
    run().unwrap()
}
