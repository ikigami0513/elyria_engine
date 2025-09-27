use cgmath::{Matrix4, Vector3, Euler, Deg, prelude::*};

#[derive(Clone)]
pub struct Transform {
    // Local space information
    position: Vector3<f32>,
    euler_rotation: Euler<Deg<f32>>,
    scale: Vector3<f32>,

    // Global space information concatenate in matrix
    model_matrix: Matrix4<f32>,

    // Dirty flag
    is_dirty: bool
}

#[allow(dead_code)]
impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            euler_rotation: Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)),
            scale: Vector3::new(1.0, 1.0, 1.0),
            model_matrix: Matrix4::identity(),
            is_dirty: true
        }
    }

    fn get_local_model_matrix(&self) -> Matrix4<f32> {
        let rotation_matrix = Matrix4::from(self.euler_rotation);
        Matrix4::from_translation(self.position) * rotation_matrix * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn compute_model_matrix(&mut self) {
        self.model_matrix = self.get_local_model_matrix();
        self.is_dirty = false;
    }

    pub fn compute_model_matrix_with_parent(&mut self, parent_global_model_matrix: &Matrix4<f32>) {
        self.model_matrix = parent_global_model_matrix * self.get_local_model_matrix();
        self.is_dirty = false;
    }

    pub fn set_local_position(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
        self.is_dirty = true;
    }

    pub fn set_local_rotation(&mut self, new_rotation: Vector3<f32>) {
        self.euler_rotation = Euler::new(Deg(new_rotation.x), Deg(new_rotation.y), Deg(new_rotation.z));
        self.is_dirty = true;
    }

    pub fn set_local_scale(&mut self, new_scale: Vector3<f32>) {
        self.scale = new_scale;
        self.is_dirty = true;
    }

    pub fn get_global_position(&self) -> Vector3<f32> {
        self.model_matrix.w.truncate()
    }

    pub fn get_local_position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn get_local_position_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.position
    }

    pub fn get_local_rotation(&self) -> Vector3<f32> {
        Vector3::new(self.euler_rotation.x.0, self.euler_rotation.y.0, self.euler_rotation.z.0)
    }

    pub fn get_local_scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.model_matrix.x.truncate().normalize()
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.model_matrix.y.truncate().normalize()
    }

    pub fn get_backward(&self) -> Vector3<f32> {
        self.model_matrix.z.truncate().normalize()
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        -self.get_backward()
    }

    pub fn get_global_scale(&self) -> Vector3<f32> {
        Vector3::new(self.get_right().magnitude(), self.get_up().magnitude(), self.get_backward().magnitude())
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
}