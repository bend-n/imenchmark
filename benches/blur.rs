use fimg::Image;
use image::RgbImage;

const SIZE: u32 = 1356;

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
    let mut i = Image::<_, 3>::build(SIZE, SIZE).buf(iai::black_box(
        include_bytes!("../small_data.imgbuf")
            .to_vec()
            .into_boxed_slice(),
    ));
    i.blur(30);
    iai::black_box(&i);
}

iai::main!(blud, imageproc, fimg, image);
