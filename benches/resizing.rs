use fimg::Image;
const SIZE: u32 = 5424;
const TO: u32 = 2712;

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
scale_fimg!(lanczos_fimg => { scale(Lanczos3) });
scale_img!(lanczos_img => { scale(Lanczos3) });
scale_resize!(lanczos_resize => { scale(Lanczos3 )});
scale_fimg!(catmull_fimg => { scale(CatmullRom) });
scale_img!(catmull_img => { scale(CatmullRom) });
scale_resize!(catmull_resize => { scale(Catrom) });

iai::main!(
    nearest_fimg,
    nearest_img,
    lanczos_fimg,
    lanczos_img,
    lanczos_resize,
    catmull_fimg,
    catmull_img,
    catmull_resize
);
