use sha3::{Digest, Sha3_256};
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel};
use thiserror::Error; // For custom error handling
use opencv::{core, imgproc, types};

/// Custom error type for image recognition
#[derive(Error, Debug)]
pub enum ImageRecognitionError {
    #[error("Failed to load image")]
    ImageError(#[from] image::ImageError),
    #[error("Failed to process image with OpenCV")]
    OpenCvError(#[from] opencv::Error),
}

/// Solves an image recognition puzzle by detecting edges and hashing the result.
pub fn solve(image_data: &[u8]) -> Result<Vec<u8>, ImageRecognitionError> {
    // 1. Load the image
    let img = image::load_from_memory(image_data)?;

    // 2. Convert to grayscale
    let img = img.to_luma8();

    // 3. Detect edges using OpenCV
    let edges = detect_edges(&img)?;

    // 4. Hash the edge data
    let mut hasher = Sha3_256::new();
    hasher.update(edges.as_raw());
    let hash = hasher.finalize().to_vec();

    Ok(hash)
}

/// Detects edges in an image using OpenCV's Canny edge detection.
fn detect_edges(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<core::Mat, opencv::Error> {
    // Convert image::ImageBuffer to OpenCV Mat
    let mat = opencv_image_from_buffer(img)?;

    // Detect edges using Canny edge detection
    let mut edges = core::Mat::default();
    imgproc::canny(&mat, &mut edges, 50.0, 150.0, 3, false)?;

    Ok(edges)
}

/// Converts an image::ImageBuffer to an OpenCV Mat.
fn opencv_image_from_buffer(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<core::Mat, opencv::Error> {
    let (width, height) = img.dimensions();
    let data = img.as_raw().as_slice();

    // Create an OpenCV Mat from the raw image data
    core::Mat::from_slice_rows_cols(data, height as i32, width as i32, core::CV_8UC1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    #[test]
    fn test_solve() {
        // Create a simple grayscale image (2x2 black square)
        let img = GrayImage::from_raw(2, 2, vec![0, 0, 0, 0]).unwrap();

        // Test edge detection and hashing
        let result = solve(&img.into_raw());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // SHA3-256 output is 32 bytes
    }
}