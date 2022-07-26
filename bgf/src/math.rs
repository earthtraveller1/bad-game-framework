#[repr(C)]
pub struct Vector2<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Clone for Vector2<T> {
    fn clone(&self) -> Self {
        return Vector2 {
            x: self.x,
            y: self.y,
        };
    }
}

#[repr(C)]
pub struct Vector4<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vector4<T> {
    pub fn from_scalar(s: T) -> Vector4<T> {
        Vector4 {
            x: s,
            y: s,
            z: s,
            w: s,
        }
    }
}

impl<T: Copy> Clone for Vector4<T> {
    fn clone(&self) -> Self {
        return Vector4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        };
    }
}

#[repr(C)]
pub struct Matrix4<T: Copy> {
    matrix: [T; 16],
}

impl<T: Copy> Matrix4<T> {
    pub fn new(a: T, b: T) -> Matrix4<T> {
        let mut matrix = [a; 16];
        matrix[0 * 4 + 0] = b;
        matrix[1 * 4 + 1] = b;
        matrix[2 * 4 + 2] = b;
        matrix[3 * 4 + 3] = b;

        return Matrix4 { matrix };
    }

    pub fn as_ptr(&self) -> *const T {
        return self.matrix.as_ptr();
    }
}

pub fn orthographic(
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    far: f32,
    near: f32,
) -> Matrix4<f32> {
    let mut matrix = Matrix4::<f32>::new(0.0, 1.0);

    matrix.matrix[0 * 4 + 0] = 2.0 / (right - left);
    matrix.matrix[1 * 4 + 1] = 2.0 / (top - bottom);
    matrix.matrix[2 * 4 + 2] = -2.0 / (far - near);

    matrix.matrix[0 * 4 + 3] = -((right + left) / (right - left));
    matrix.matrix[1 * 4 + 3] = -((top + bottom) / (top - bottom));
    matrix.matrix[2 * 4 + 3] = -((far + near) / (far - near));

    return matrix;
}
