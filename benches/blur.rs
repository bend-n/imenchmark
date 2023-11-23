#![feature(array_chunks)]
use fimg::Image;
use image::RgbImage;
use opencv::{core::CV_8UC3, prelude::*};

const SIZE: u32 = 1356;

pub fn opencv() {
    opencv::core::set_num_threads(1).unwrap();
    let mut data = iai::black_box(include_bytes!("../small_data.imgbuf").to_vec())
        .array_chunks::<3>()
        .flat_map(|&[r, g, b]| [b, g, r])
        .collect::<Vec<_>>();
    let mut o = unsafe { Mat::new_rows_cols(SIZE as i32, SIZE as i32, CV_8UC3).unwrap() };
    opencv::imgproc::gaussian_blur_def(
        &unsafe {
            Mat::new_size_with_data_def(
                opencv::core::Size_ {
                    width: SIZE as i32,
                    height: SIZE as i32,
                },
                CV_8UC3,
                data.as_mut_ptr() as *mut core::ffi::c_void,
            )
            .unwrap()
        },
        &mut o,
        opencv::core::Size_ {
            width: 0,
            height: 0,
        },
        15.0,
    )
    .unwrap();
    iai::black_box(&o);
}

pub fn imageproc() {
    iai::black_box(&imageproc::filter::gaussian_blur_f32(
        &RgbImage::from_raw(
            SIZE,
            SIZE,
            iai::black_box(include_bytes!("../small_data.imgbuf").to_vec()),
        )
        .unwrap(),
        15.0,
    ));
}

pub fn image() {
    iai::black_box(&image::imageops::blur(
        &RgbImage::from_raw(
            SIZE,
            SIZE,
            iai::black_box(include_bytes!("../small_data.imgbuf").to_vec()),
        )
        .unwrap(),
        15.0,
    ));
}

pub fn fastblur() {
    let mut data = iai::black_box(
        include_bytes!("../small_data.imgbuf")
            .array_chunks::<3>()
            .copied()
            .collect::<Vec<_>>(),
    );
    fastblur::gaussian_blur(&mut data, SIZE as usize, SIZE as usize, 15.0);
    iai::black_box(&data);
}

pub fn blurslice() {
    let mut data = iai::black_box(include_bytes!("../small_data.imgbuf").to_vec());
    blurslice::gaussian_blur_bytes::<3>(&mut data, SIZE as usize, SIZE as usize, 15.0).unwrap();
    iai::black_box(&data);
}

pub fn blud() {
    let mut i = Image::<_, 3>::build(SIZE, SIZE).buf(iai::black_box(
        include_bytes!("../small_data.imgbuf")
            .to_vec()
            .into_boxed_slice(),
    ));
    blud::blur(&mut i, unsafe { umath::FF32::new(15.0) });
    iai::black_box(&i);
}

pub fn fimg() {
    let mut i: Image<Box<[u32]>, 1> = Image::<_, 3>::build(SIZE, SIZE)
        .buf(iai::black_box(&include_bytes!("../small_data.imgbuf")[..]))
        .into();
    i.blur_argb(30);
    iai::black_box(&i);
}

pub fn stackblur() {
    let i: Image<Box<[u32]>, 1> = Image::<_, 3>::build(SIZE, SIZE)
        .buf(iai::black_box(&include_bytes!("../small_data.imgbuf")[..]))
        .into();
    let mut i = i.take_buffer().to_vec();
    stackblur_iter::simd_blur_argb::<8>(
        &mut stackblur_iter::imgref::ImgRefMut::new(&mut i, SIZE as usize, SIZE as usize),
        30,
    );
    iai::black_box(&i);
}

iai::main!(blud, imageproc, fimg, image, fastblur, opencv, stackblur);
