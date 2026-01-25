use rstest::rstest;
use the_raytracer_challenge_rust::primitives::{canvas::Canvas, color::Color};

#[rstest] 
fn when_creating_a_canvas_width_height_and_pixels_initialized () {
    let cv = Canvas::new(10, 20);
    
    assert_eq!(cv.width, 10);
    assert_eq!(cv.height, 20);

    for x in 0..10 {
        for y in 0..20 {
            assert_eq!(cv.pixel_at(x, y), Some(Color::new(0.0, 0.0, 0.0)).as_ref())
        }
    }
}

#[rstest] 
fn when_creating_a_canvas_and_setting_a_color_it_is_set () {
    let mut cv = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    let red_clone = red.clone();
    cv.write_pixel(2, 3, red);
    assert_eq!(cv.pixel_at(2, 3), Some(&red_clone));
    
}

#[rstest]
fn when_creating_an_empty_canvas_ppm_header_is_correct() {
    let cv = Canvas::new(5, 3);
    let ppm = cv.to_ppm();
    let parts: Vec<&str> = ppm.split('\n').collect();
    let first_three = &parts[..3];
    let value = first_three.join("\n");
    assert_eq!(value, "P3\n5 3\n255");
    
}

#[rstest]
fn when_creating_canvas_with_pixel_data_ppm_consts_of_pixel_data() {
    let mut cv = Canvas::new(5, 3);
    let C1 = Color::new(1.5, 0.0, 0.0);
    let C2 = Color::new(0.0, 0.5, 0.0);
    let C3 = Color::new(-0.5, 0.0, 1.0);
    cv.write_pixel(0, 0, C1);
    cv.write_pixel(2, 1, C2);
    cv.write_pixel(4, 2, C3);
    let ppm = cv.to_ppm();
    assert_eq!(ppm, "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
}