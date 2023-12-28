extern crate nannou;
use nannou::{prelude::*, color::Hwb};

trait RectExt {
    fn center(&self) -> Point2;
}

impl RectExt for Rect {
    fn center(&self) -> Point2 {
        pt2(
            (self.left() + self.right()) / 2.0,
            (self.top() + self.bottom()) / 2.0,
        )
    }
}

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let window_rect = app.window_rect();
    let closest_ratio = find_closest_ratio(window_rect.w(), window_rect.h());

    // Calculate the dimensions for the rectangles based on the closest ratio
    let rect_width = window_rect.w() / closest_ratio.0 as f32;
    let rect_height = window_rect.h() / closest_ratio.1 as f32;

    let mut num_rect = 0.0;

    // Draw rectangles
    for row in 0..closest_ratio.1 {
        for col in 0..closest_ratio.0 {
            let rect_x = window_rect.left() + rect_width * (col as f32 + 0.5);
            let rect_y = window_rect.top() - rect_height * (row as f32 + 0.5);

            let hue = 15.0 * num_rect;
            let hue_tile = Rect::from_x_y_w_h(rect_x, rect_y, rect_width, rect_height);
            draw_hue_tile(&draw, hue, hue_tile);

            num_rect += 1.0;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn find_closest_ratio(width: f32, height: f32) -> (u32, u32) {
    let current_ratio = width / height;
    let defined_ratios = [
        (1, 24),
        (2, 12),
        (3, 8),
        (4, 6),
        (6, 4),
        (8, 3),
        (12, 2),
        (24, 1),
    ];

    let closest_ratio = defined_ratios
        .iter()
        .min_by_key(|&&(x, y)| {
            let ratio = x as f32 / y as f32;
            ((ratio - current_ratio).abs() * 1000.0) as u32 // Scale the difference for comparison
        })
        .unwrap();

    *closest_ratio
}

fn draw_hue_tile(draw: &Draw, hue: f32, rect: Rect) {
    let rect_center = rect.center();
    let corner = (
        rect.top_left(),
        rect.top_right(),
        rect.bottom_right(),
        rect.bottom_left(),
    );

    let hwb_colors = [Hwb::new(hue, 0.0, 0.0),
                                Hwb::new(hue, 0.0, 0.5),
                                Hwb::new(hue, 0.25, 0.25),
                                Hwb::new(hue, 0.5, 0.0)];
    print!("Hue:{hue}\n");
    

    // Draw the four triangles with oscillating Hwb color
    draw.tri()
        .points(rect_center, corner.0, corner.1)
        .color(hwb_colors[0]);
    draw.tri()
        .points(rect_center, corner.1, corner.2)
        .color(hwb_colors[1]);
    draw.tri()
        .points(rect_center, corner.2, corner.3)
        .color(hwb_colors[2]);
    draw.tri()
        .points(rect_center, corner.3, corner.0)
        .color(hwb_colors[3]);
}
