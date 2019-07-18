pub struct BoundingRectangle {
    pub x: f32,
    pub y: f32,

    pub width: f32,
    pub height: f32,
}

fn is_between(value: f32, minimum: f32, maximum: f32) -> bool {
    return value >= minimum && value <= maximum;
}

impl BoundingRectangle {
    pub fn collides_with(&self, another: &BoundingRectangle) -> bool {
        let x_between = is_between(self.x, another.x, another.x + another.width)
            || is_between(another.x, self.x, self.x + self.width);

        let y_between = is_between(self.y, another.y, another.y + another.height)
            || is_between(another.y, self.y, self.y + self.height);

        return x_between && y_between;
    }
}

#[test]
fn collides_with_returns_true_for_typical_cases() {
    let main = BoundingRectangle {
        x: -5.0,
        y: -10.0,
        width: 10.0,
        height: 20.0,
    };
    let a = BoundingRectangle {
        x: -10.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    let b = BoundingRectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    let c = BoundingRectangle {
        x: -10.0,
        y: -10.0,
        width: 10.0,
        height: 10.0,
    };
    let d = BoundingRectangle {
        x: 0.0,
        y: -10.0,
        width: 10.0,
        height: 10.0,
    };
    let e = BoundingRectangle {
        x: -5.0,
        y: -10.0,
        width: 10.0,
        height: 20.0,
    };

    assert!(main.collides_with(&a));
    assert!(main.collides_with(&b));
    assert!(main.collides_with(&c));
    assert!(main.collides_with(&d));
    assert!(main.collides_with(&e));

    assert!(a.collides_with(&main));
    assert!(b.collides_with(&main));
    assert!(c.collides_with(&main));
    assert!(d.collides_with(&main));
    assert!(e.collides_with(&main));
}

#[test]
fn collides_with_returns_false_for_non_overlapped_rectangles() {
    let a = BoundingRectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 20.0,
    };
    let b = BoundingRectangle {
        x: 50.0,
        y: 50.0,
        width: 30.0,
        height: 40.0,
    };

    assert!(a.collides_with(&b) == false);
    assert!(b.collides_with(&a) == false);
}
