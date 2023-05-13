use macroquad::prelude::Rect;

pub enum Side {
    TOP,
    LEFT,
    RIGHT,
    BOTTOM,
    NONE,
}

/**
 * Gives the side of the rectA which collides with rectB
 */
pub fn collides(rect_a: &Rect, rect_b: &Rect) -> Side {
    let top = Rect::new(rect_a.x, rect_a.y, rect_a.w, 10.0);
    let right = Rect::new(rect_a.x + rect_a.w, rect_a.y, 10.0, rect_a.h);
    let bottom = Rect::new(rect_a.x, rect_a.y + rect_a.h, rect_a.w, 10.0);
    let left = Rect::new(rect_a.x, rect_a.y, 10.0, rect_a.h);

    if top.overlaps(rect_b) {
        println!("top");
        return Side::TOP;
    } else if right.overlaps(rect_b) {
        println!("RIGHT");
        return Side::RIGHT;
    } else if bottom.overlaps(rect_b) {
        println!("BOTTOM");
        return Side::BOTTOM;
    } else if left.overlaps(rect_b) {
        println!("LEFT");
        return Side::LEFT;
    } else {
        return Side::NONE;
    }
}
