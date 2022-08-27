use anyhow::{anyhow, Result};
use image::RgbImage;
use ndarray::{Array1, ArrayView1, ArrayView3};
use opencv::{core::Vector, highgui, imgcodecs, prelude::*, videoio, videoio::VideoCapture};

pub fn opencv_test(device: i32) -> Result<()> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;

    let mut cam = VideoCapture::new(device, videoio::CAP_ANY)?;

    let mut frame = Mat::default();
    for i in 0..1 {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        imgcodecs::imwrite("./frame.png", &frame, &Vector::default())?;
        let a = frame.try_as_array()?;
        let img = array_to_image(a);
        img.save("./out.png")?;
        // println!("frame {}", a);

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
