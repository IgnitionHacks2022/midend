use std::{sync::mpsc::Sender, thread, time::Duration};

use anyhow::{anyhow, Result};
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

const DELAY: u64 = 3000;

// translated from https://www.geeksforgeeks.org/webcam-motion-detector-python/
pub fn opencv_test(tx: Sender<Vec<u8>>, device: i32) -> Result<()> {
    highgui::named_window("highgui", highgui::WINDOW_FULLSCREEN)?;

    let mut cam = VideoCapture::new(device, videoio::CAP_ANY)?;
    let mut static_frame = Mat::default();

    let mut i = 0;
    // flag to limit how often a motion event is sent
    let mut sent = false;
    loop {
        i += 1;

        // preprocessing steps
        let mut frame = capture_frame(&mut cam)?;
        let original_frame = frame.clone();
        grayscale(&mut frame)?;
        gaussian_blur(&mut frame)?;

        // save first frame as reference
        if i < 10 {
            static_frame = frame.clone();
            continue;
        }

        // difference between reference frame
        let mut diff = frame_diff(&static_frame, &frame)?;
        threshold(&mut diff);

        // find contours
        let contours = find_contours(&diff)?;

        // debug contour lines
        let mut boxed = original_frame;
        for contour in contours.iter() {
            if imgproc::contour_area(&contour, false)? > 5000. {
                let rect = bounding_rect(&contour)?;
                imgproc::rectangle(&mut boxed, rect, VecN([0., 255., 0., 1.]), 5, LINE_8, 0)?;
            }
        }

        // take picture if motion was detected
        if !contours.is_empty() {
            if sent == false {
                // delay before sending image
                thread::sleep(Duration::from_millis(DELAY));

                let motion_frame = capture_frame(&mut cam)?;
                let mut buf = Vector::default();
                imgcodecs::imencode(".png", &motion_frame, &mut buf, &Vector::default())?;
                let buf = buf.to_vec();

                tx.send(buf)?;
                sent = true;
            }
        } else {
            sent = false;
        }

        highgui::imshow("highgui", &boxed)?;
        // imgcodecs::imwrite("./frame.png", &diff, &Vector::default())?;

        if highgui::wait_key(1)? == 113 {
            break;
        }
    }

    cam.release()?;
    highgui::destroy_all_windows()?;

    Ok(())
}

/// Capture one frame from camera
fn capture_frame(cam: &mut VideoCapture) -> Result<Mat> {
    let mut out = Mat::default();
    cam.read(&mut out)?;
    Ok(out)
}

/// Convert to grayscale
fn grayscale(frame: &mut Mat) -> Result<()> {
    if frame.size()?.width <= 0 {
        return Err(anyhow!("frame size assertion failed"));
    }
    imgproc::cvt_color(&frame.clone(), frame, imgproc::COLOR_BGR2GRAY, 0);
    Ok(())
}

/// Apply gaussian blur effect
fn gaussian_blur(frame: &mut Mat) -> opencv::Result<()> {
    imgproc::gaussian_blur(
        &frame.clone(),
        frame,
        Size {
            width: 21,
            height: 21,
        },
        0.,
        0.,
        BORDER_DEFAULT,
    )
}

/// Find difference between two frames
fn frame_diff(frame_a: &Mat, frame_b: &Mat) -> opencv::Result<Mat> {
    let mut out = Mat::default();
    absdiff(&frame_a, &frame_b, &mut out)?;
    Ok(out)
}

/// Normalize frame using threshold
fn threshold(frame: &mut Mat) -> opencv::Result<()> {
    let mut dummy = Mat::default();
    let thresh_frame = imgproc::threshold(&frame.clone(), &mut dummy, 30., 255., THRESH_BINARY)?;
    imgproc::dilate(
        &dummy,
        frame,
        &Mat::default(),
        Point { x: -1, y: -1 },
        2,
        BORDER_CONSTANT,
        morphology_default_border_value()?,
    )
}

fn find_contours(frame: &Mat) -> opencv::Result<VectorOfMat> {
    let mut contours = VectorOfMat::default();
    imgproc::find_contours(
        frame,
        &mut contours,
        RETR_EXTERNAL,
        CHAIN_APPROX_SIMPLE,
        Point { x: 0, y: 0 },
    )?;
    Ok(contours)
}
