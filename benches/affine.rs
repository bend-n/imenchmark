//! note that imageproc doesnt support in-place transformations.
use fimg::Image;
use image::{imageops, RgbImage};
const SIZE: u32 = 5424;

// dont have to alloc, but it would be less fair.
pub fn fimg() -> Image<Vec<u8>, 3> {
    Image::build(SIZE, SIZE).buf(iai::black_box(include_bytes!("../data.imgbuf").to_vec()))
}

pub fn ipimg() -> RgbImage {
    RgbImage::from_raw(
        SIZE,
        SIZE,
        iai::black_box(include_bytes!("../data.imgbuf").to_vec()),
    )
    .unwrap()
}

pub fn rot_90_fimg() {
    iai::black_box(unsafe { fimg().cloner().rot_90() });
}

pub fn rot_90_imgproc() {
    iai::black_box(imageops::rotate90(&ipimg()));
}

pub fn rot_180_fimg() {
    iai::black_box(fimg().cloner().rot_180());
}

pub fn rot_180_imgproc() {
    iai::black_box(imageops::rotate180(&ipimg()));
}

pub fn rot_270_fimg() {
    iai::black_box(unsafe { fimg().cloner().rot_270() });
}

pub fn rot_270_imgproc() {
    iai::black_box(imageops::rotate270(&ipimg()));
}

pub fn flip_h_fimg() {
    iai::black_box(fimg().cloner().flip_h());
}

pub fn flip_h_imgproc() {
    iai::black_box(imageops::flip_horizontal(&ipimg()));
}

pub fn flip_v_fimg() {
    iai::black_box(fimg().cloner().flip_h());
}

pub fn flip_v_imgproc() {
    iai::black_box(imageops::flip_vertical(&ipimg()));
}

iai::main!(
    rot_90_fimg,
    rot_90_imgproc,
    rot_180_fimg,
    rot_180_imgproc,
    rot_270_fimg,
    rot_270_imgproc,
    flip_h_fimg,
    flip_h_imgproc,
    flip_v_fimg,
    flip_v_imgproc,
);
