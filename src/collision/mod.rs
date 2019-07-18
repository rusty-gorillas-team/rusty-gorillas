pub struct BoundingRectangle {
    x: f32,
    y: f32,

    width: f32,
    height: f32,
}

fn is_between(value: f32, minimum: f32, maximum: f32) -> bool {
    return value >= minimum && value <= maximum;
}

impl BoundingRectangle {
    pub fn collides_with(&self, another_rectangle: &BoundingRectangle) -> bool {
        let x_between = is_between(
            self.x,
            another_rectangle.x,
            &another_rectangle.x + &another_rectangle.width,
        ) || is_between(
            another_rectangle.x,
            self.x,
            self.x + &another_rectangle.width,
        );

        let y_between = is_between(
            self.y,
            another_rectangle.y,
            &another_rectangle.y + &another_rectangle.height,
        ) || is_between(
            another_rectangle.y,
            self.y,
            self.y + &another_rectangle.height,
        );

        return x_between && y_between;
    }
}

#[test]
fn collides_with_each_other() {
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
fn collides_with_false_for_non_colliding() {
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
