use raylib::math::Vector2;


pub fn rotate_vector(vector: Vector2, angle_rad: f32) -> Vector2{

    // let dist = (vector.x * vector.x) + (vector.y * vector.y);
    // let angle = (vector.x.abs() / vector.y.abs()).atan();
    // let angle = angle + angle_rad;
    return Vector2 {
        x: (vector.x * angle_rad.cos()) - (vector.y * angle_rad.sin()),
        y: (vector.y * angle_rad.cos()) + (vector.x * angle_rad.sin()),
    };

}