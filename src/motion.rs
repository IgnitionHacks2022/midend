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
pub fn opencv_test(device: i32) -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;

    let mut cam = VideoCapture::new(device, videoio::CAP_ANY)?;
    let mut static_frame = Mat::default();

    for i in 0..200 {
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

trait AsArray {
    fn try_as_array(&self) -> Result<ArrayView3<u8>>;
}
impl AsArray for Mat {
    fn try_as_array(&self) -> Result<ArrayView3<u8>> {
        if !self.is_continuous() {
            return Err(anyhow!("Mat is not continuous"));
        }
        let bytes = self.data_bytes()?;
        // println!("databytes {:?}", bytes);
        let size = self.size()?;
        // println!("size {:?}", size);
        let a = ArrayView3::from_shape((size.height as usize, size.width as usize, 3), bytes)?;
        Ok(a)
    }
}

fn array_to_image(arr: ArrayView3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());
    let (height, width, _) = arr.dim();
    let raw = arr.to_slice().expect("Failed to extract slice from array");
    RgbImage::from_raw(width as u32, height as u32, raw.to_vec())
        .expect("container should have the right size for the image dimensions")
}
