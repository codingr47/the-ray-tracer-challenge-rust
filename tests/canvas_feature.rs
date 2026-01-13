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
