use fimg::Image;
use image::Rgba;
use imageproc::point::Point;
use umath::generic_float::Constructors;
use umath::FF32;

macro_rules! run_fimg {
    (fn $fn:ident($img:ident) { $($tt:tt)+ }) => {
        fn $fn() {
            let mut $img = iai::black_box(Image::<_, 4>::alloc(1000, 1000).boxed());
            $($tt)+
            iai::black_box($img);
        }
    };
}

macro_rules! run_imgp {
    (fn $fn:ident($img:ident) { $($tt:tt)+ }) => {
        fn $fn() {
            let mut $img = iai::black_box(image::RgbaImage::new(1000, 1000));
            $($tt)+
            iai::black_box($img);
        }
    };
}

run_fimg! {
    fn tri_fimg(img) { unsafe {
        // btw it would be faster if it used points() but future™
        img.tri::<FF32>(
            (FF32::zero(), FF32::zero()),
            (FF32::new(1000.), FF32::new(500.)),
            (FF32::zero(), FF32::new(999.)),
            [255, 255, 255, 255],
        );
    } }
}

run_imgp! {
    fn tri_imgp(img) {
        // fimg L tbh
        imageproc::drawing::draw_polygon_mut(&mut img, &[Point::new(0, 0), Point::new(1000, 500), Point::new(0, 999)], Rgba([255,255,255,255]));
    }
}

run_fimg! { fn circle_filled_fimg(i) {
    i.circle((500,500), 500, [128, 128, 128, 255]);
} }

run_imgp! { fn circle_filled_imgp(i) {
    imageproc::drawing::draw_filled_circle_mut(&mut i, (500,500), 500, Rgba([128, 128, 128, 255]));
} }

run_fimg! { fn border_circle_fimg(i) {
    for radius in (5..500).step_by(5) {
        i.border_circle((500, 500), radius, [128, 128, 128, 255]);
    }
} }

run_imgp! { fn border_circle_imgp(i) {
    for radius in (5..500).step_by(5) {
        imageproc::drawing::draw_hollow_circle_mut(&mut i, (500,500), radius, Rgba([128, 128, 128, 255]));
    }
} }

run_fimg! { fn line_fimg(i) {
    for y in (250..750).step_by(10) {
        i.line((0, y), (2000,2000), [128, 0, 0, 255]);
    }
} }

run_imgp! { fn line_imgp(i) {
    for y in (250..750).step_by(10) {
        imageproc::drawing::draw_line_segment_mut(&mut i, (0.0, y as f32), (2000.0,2000.0), Rgba([128, 0, 0, 255]));
    }
} }

iai::main!(
    tri_fimg,
    tri_imgp,
    circle_filled_fimg,
    circle_filled_imgp,
    border_circle_fimg,
    border_circle_imgp,
    line_fimg,
    line_imgp,
);
