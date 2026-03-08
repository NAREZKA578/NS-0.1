//! Math utilities
//! 
//! Contains helper functions for mathematical operations

use glam::{Vec3, Quat, Mat4};

/// Clamp a value between min and max
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Smooth step interpolation
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smoother step interpolation
#[inline]
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Convert degrees to radians
#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convert radians to degrees
#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Calculate distance between two points
#[inline]
pub fn distance(a: Vec3, b: Vec3) -> f32 {
    (b - a).length()
}

/// Calculate squared distance (faster than distance)
#[inline]
pub fn distance_squared(a: Vec3, b: Vec3) -> f32 {
    (b - a).length_squared()
}

/// Get direction from point A to point B
#[inline]
pub fn direction(from: Vec3, to: Vec3) -> Vec3 {
    (to - from).normalize_or_zero()
}

/// Rotate a vector around Y axis
#[inline]
pub fn rotate_y(vector: Vec3, angle: f32) -> Vec3 {
    let cos = angle.cos();
    let sin = angle.sin();
    Vec3::new(
        vector.x * cos - vector.z * sin,
        vector.y,
        vector.x * sin + vector.z * cos,
    )
}

/// Look at rotation quaternion
pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Quat {
    let forward = (target - eye).normalize();
    let right = forward.cross(up).normalize();
    let up = right.cross(forward);
    
    Mat4::from_cols(
        Vec4::new(right.x, right.y, right.z, 0.0),
        Vec4::new(up.x, up.y, up.z, 0.0),
        Vec4::new(-forward.x, -forward.y, -forward.z, 0.0),
        Vec4::new(eye.x, eye.y, eye.z, 1.0),
    ).to_quat()
}

use glam::Vec4;

/// Random point in circle
pub fn random_point_in_circle(radius: f32) -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let angle = rng.gen_range(0.0..=std::f32::consts::PI * 2.0);
    let r = radius * rng.gen_range(0.0..=1.0).sqrt(); // Sqrt for uniform distribution
    
    Vec3::new(
        r * angle.cos(),
        0.0,
        r * angle.sin(),
    )
}

/// Random point in sphere
pub fn random_point_in_sphere(radius: f32) -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Marsaglia method
    let mut x1: f32;
    let mut x2: f32;
    let mut w: f32;
    
    loop {
        x1 = rng.gen_range(-1.0..=1.0);
        x2 = rng.gen_range(-1.0..=1.0);
        w = x1 * x1 + x2 * x2;
        if w < 1.0 {
            break;
        }
    }
    
    let y = (1.0 - w).sqrt();
    let z = rng.gen_range(-1.0..=1.0);
    
    Vec3::new(
        2.0 * x1 * y * radius,
        2.0 * x2 * y * radius,
        (1.0 - 2.0 * z * z) * radius,
    )
}

/// Inverse lerp (get interpolation factor from value)
#[inline]
pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if (b - a).abs() < f32::EPSILON {
        return 0.0;
    }
    ((value - a) / (b - a)).clamp(0.0, 1.0)
}

/// Remap value from one range to another
#[inline]
pub fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = inverse_lerp(from_min, from_max, value);
    lerp(to_min, to_max, t)
}

/// Map value with custom curve
pub fn map_curve(value: f32, curve_power: f32) -> f32 {
    value.powf(curve_power)
}

/// Dampen value towards target
#[inline]
pub fn damp(current: f32, target: f32, lambda: f32, dt: f32) -> f32 {
    let d = -lambda * dt;
    if d.abs() < f32::EPSILON {
        return target;
    }
    let decay = d.exp();
    current + (target - current) * (1.0 - decay)
}

/// Spring function for smooth motion
pub fn spring(current: f32, target: f32, velocity: &mut f32, stiffness: f32, damping: f32, dt: f32) -> f32 {
    let displacement = current - target;
    let spring_force = -stiffness * displacement;
    let damping_force = -damping * *velocity;
    let acceleration = spring_force + damping_force;
    
    *velocity += acceleration * dt;
    *velocity *= 0.99; // Small drag
    
    current + *velocity * dt
}

/// Check if point is inside box
pub fn point_in_box(point: Vec3, box_min: Vec3, box_max: Vec3) -> bool {
    point.x >= box_min.x && point.x <= box_max.x &&
    point.y >= box_min.y && point.y <= box_max.y &&
    point.z >= box_min.z && point.z <= box_max.z
}

/// Find closest point on line segment
pub fn closest_point_on_segment(point: Vec3, segment_start: Vec3, segment_end: Vec3) -> Vec3 {
    let segment = segment_end - segment_start;
    let length_sq = segment.length_squared();
    
    if length_sq < f32::EPSILON {
        return segment_start;
    }
    
    let t = ((point - segment_start).dot(segment) / length_sq).clamp(0.0, 1.0);
    segment_start + segment * t
}

/// Calculate angle between two vectors
pub fn angle_between(a: Vec3, b: Vec3) -> f32 {
    let dot = a.dot(b);
    let cross = a.cross(b).length();
    cross.atan2(dot)
}
