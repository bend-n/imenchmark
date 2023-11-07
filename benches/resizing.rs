#![feature(array_chunks)]
use fimg::Image;
use opencv::{core::CV_8UC3, prelude::*};

const SIZE: u32 = 5424;
const TO: u32 = 2712;

macro_rules! scale_opencv {
    ($name:ident => { scale($alg:literal) }) => {
        fn $name() {
            opencv::core::set_num_threads(1).unwrap();
            let mut data = iai::black_box(include_bytes!("../data.imgbuf").to_vec())
                .array_chunks::<3>()
                .flat_map(|&[r, g, b]| [b, g, r])
                .collect::<Vec<_>>();
            let mut o = unsafe { Mat::new_rows_cols(SIZE as i32, SIZE as i32, CV_8UC3).unwrap() };
            opencv::imgproc::resize(
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
                    width: TO as i32,
                    height: TO as i32,
                },
                0.0,
                0.0,
                $alg,
            )
            .unwrap();
            iai::black_box(&o);
        }
    };
}

macro_rules! scale_fimg {
    ($name:ident => { scale($alg: ident) }) => {
        fn $name() {
            #[allow(unused_mut)]
            let mut img = Image::<_, 3>::build(SIZE, SIZE)
                .buf(iai::black_box(include_bytes!("../data.imgbuf").to_vec()));
            img.scale::<fimg::scale::$alg>(TO, TO);
            iai::black_box(img);
        }
    };
}

macro_rules! scale_resize {
    ($name:ident => { scale($alg: ident) }) => {
        fn $name() {
            use rgb::FromSlice;
            let src = iai::black_box(include_bytes!("../data.imgbuf").to_vec());
            let mut dst = vec![0; TO as usize * TO as usize * 3];
            resize::new(
                SIZE as usize,
                SIZE as usize,
                TO as usize,
                TO as usize,
                resize::Pixel::RGB8,
                resize::Type::$alg,
            )
            .unwrap()
            .resize(src.as_rgb(), dst.as_rgb_mut())
            .unwrap();
        }
    };
}

macro_rules! scale_img {
    ($name:ident => { scale($alg: ident) }) => {
        fn $name() {
            let img = image::RgbImage::from_raw(
                SIZE,
                SIZE,
                iai::black_box(include_bytes!("../data.imgbuf").to_vec()),
            )
            .unwrap();
            iai::black_box(image::imageops::resize(
                &img,
                TO,
                TO,
                image::imageops::FilterType::$alg,
            ));
        }
    };
}

scale_fimg!(nearest_fimg => { scale(Nearest) });
scale_img!(nearest_img => { scale(Nearest) });
scale_opencv!(nearest_opencv => { scale(6) });
scale_fimg!(lanczos_fimg => { scale(Lanczos3) });
scale_img!(lanczos_img => { scale(Lanczos3) });
scale_opencv!(lanczos_opencv => { scale(4) });
scale_resize!(lanczos_resize => { scale(Lanczos3 )});
scale_fimg!(bicubic_fimg => { scale(CatmullRom) });
scale_img!(bicubic_img => { scale(CatmullRom) });
scale_resize!(bicubic_resize => { scale(Catrom) });
scale_opencv!(bicubic_opencv => { scale(2) });

iai::main!(
    nearest_fimg,
    nearest_img,
    nearest_opencv,
    lanczos_fimg,
    lanczos_img,
    lanczos_resize,
    lanczos_opencv,
    bicubic_fimg,
    bicubic_img,
    bicubic_resize,
    bicubic_opencv
);
