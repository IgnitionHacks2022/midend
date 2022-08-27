use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};
use image::RgbImage;
use ndarray::{Array1, ArrayView1, ArrayView3};
use opencv::{
    core::{
        absdiff, Param_UNSIGNED_INT, Point, Rect, Scalar, Size, VecN, Vector, BORDER_CONSTANT,
        BORDER_DEFAULT,
    },
    highgui, imgcodecs,
    imgproc::{
        self, bounding_rect, morphology_default_border_value, CHAIN_APPROX_SIMPLE, LINE_8,
        RETR_EXTERNAL, THRESH_BINARY,
    },
    prelude::*,
    types::VectorOfMat,
    videoio,
    videoio::VideoCapture,
};

// translated from https://www.geeksforgeeks.org/webcam-motion-detector-python/
pub fn opencv_test(tx: Sender<String>, device: i32) -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;

    let mut cam = VideoCapture::new(device, videoio::CAP_ANY)?;
    let mut static_frame = Mat::default();

    let mut i = 0;
    loop {
        i += 1;
        let mut frame = Mat::default();
        cam.read(&mut frame)?;

        // convert to grayscale
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        // apply gaussian blur
        let mut blur = Mat::default();
        imgproc::gaussian_blur(
            &gray,
            &mut blur,
            Size {
                width: 21,
                height: 21,
            },
            0.,
            0.,
            BORDER_DEFAULT,
        )?;

        // save first frame as reference
        if i < 10 {
            static_frame = blur;
            continue;
        }

        // difference between reference frame
        let mut diff = Mat::default();
        absdiff(&static_frame, &blur, &mut diff)?;

        // apply a threshold
        let mut dummy = Mat::default();
        let thresh_frame = imgproc::threshold(&diff, &mut dummy, 30., 255., THRESH_BINARY)?;
        let mut thresh = Mat::default();
        imgproc::dilate(
            &dummy,
            &mut thresh,
            &Mat::default(),
            Point { x: -1, y: -1 },
            2,
            BORDER_CONSTANT,
            morphology_default_border_value()?,
        )?;

        // find contours
        let mut contours = VectorOfMat::default();
        imgproc::find_contours(
            &thresh,
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_SIMPLE,
            Point { x: 0, y: 0 },
        )?;

        let mut boxed = frame;
        for contour in contours.iter() {
            if imgproc::contour_area(&contour, false)? < 5000. {
                continue;
            }
            // motion detected, send message
            tx.send("Motion!".to_owned())?;

            let rect = bounding_rect(&contour)?;
            imgproc::rectangle(&mut boxed, rect, VecN([0., 255., 0., 1.]), 5, LINE_8, 0)?;
        }

        highgui::imshow("window", &boxed)?;
        // imgcodecs::imwrite("./frame.png", &diff, &Vector::default())?;

        if highgui::wait_key(1)? == 113 {
            break;
        }
    }

    cam.release()?;
    highgui::destroy_all_windows()?;

    Ok(())
}
