#![feature(array_chunks)]
//! note that image doesnt support in-place transformations.
use fimg::Image;
use image::{imageops, RgbImage};
use opencv::{core::CV_8UC3, prelude::*};
const SIZE: u32 = 5424;

// dont have to alloc, but it would be less fair.
pub fn fimg() -> Image<Vec<u8>, 3> {
    Image::build(SIZE, SIZE).buf(iai::black_box(include_bytes!("../data.imgbuf").to_vec()))
}

pub fn opencv() -> Mat {
    opencv::core::set_num_threads(1).unwrap();
    let mut data = iai::black_box(include_bytes!("../data.imgbuf").to_vec())
        .array_chunks::<3>()
        .flat_map(|&[r, g, b]| [b, g, r])
        .collect::<Vec<_>>();
    let mat = unsafe {
        Mat::new_size_with_data_def(
            opencv::core::Size_ {
                width: 5424,
                height: 5424,
            },
            CV_8UC3,
            data.as_mut_ptr() as *mut core::ffi::c_void,
        )
        .unwrap()
    };
    std::mem::forget(data);
    mat
}

pub fn o_opencv() -> Mat {
    unsafe { Mat::new_rows_cols(SIZE as i32, SIZE as i32, CV_8UC3).unwrap() }
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

pub fn rot_90_opencv() {
    let mut o = o_opencv();
    opencv::core::rotate(&opencv(), &mut o, 0).unwrap();
    iai::black_box(o);
}

pub fn rot_180_fimg() {
    iai::black_box(fimg().cloner().rot_180());
}

pub fn rot_180_imgproc() {
    iai::black_box(imageops::rotate180(&ipimg()));
}

pub fn rot_180_opencv() {
    let mut o = o_opencv();
    opencv::core::rotate(&opencv(), &mut o, 1).unwrap();
    iai::black_box(o);
}

pub fn rot_270_fimg() {
    iai::black_box(unsafe { fimg().cloner().rot_270() });
}

pub fn rot_270_imgproc() {
    iai::black_box(imageops::rotate270(&ipimg()));
}

pub fn rot_270_opencv() {
    let mut o = o_opencv();
    opencv::core::rotate(&opencv(), &mut o, -1).unwrap();
    iai::black_box(o);
}

pub fn flip_h_fimg() {
    iai::black_box(fimg().cloner().flip_h());
}

pub fn flip_h_imgproc() {
    iai::black_box(imageops::flip_horizontal(&ipimg()));
}

pub fn flip_h_opencv() {
    let mut o = o_opencv();
    opencv::core::flip(&opencv(), &mut o, 1).unwrap();
    iai::black_box(o);
}

pub fn flip_v_fimg() {
    iai::black_box(fimg().cloner().flip_h());
}

pub fn flip_v_imgproc() {
    iai::black_box(imageops::flip_vertical(&ipimg()));
}

pub fn flip_v_opencv() {
    let mut o = o_opencv();
    opencv::core::flip(&opencv(), &mut o, 0).unwrap();
    iai::black_box(o);
}

iai::main!(
    rot_90_fimg,
    rot_90_imgproc,
    rot_90_opencv,
    rot_180_fimg,
    rot_180_imgproc,
    rot_180_opencv,
    rot_270_fimg,
    rot_270_imgproc,
    rot_270_opencv,
    flip_h_fimg,
    flip_h_imgproc,
    flip_h_opencv,
    flip_v_fimg,
    flip_v_imgproc,
    flip_v_opencv,
);
