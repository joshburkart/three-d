use crate::renderer::*;

#[derive(Clone)]
pub struct Line {
    model: Model2D,
    context: Context,
    pixel0: (f32, f32),
    pixel1: (f32, f32),
    width: f32,
}

impl Line {
    pub fn new(
        context: &Context,
        pixel0: (f32, f32),
        pixel1: (f32, f32),
        width: f32,
    ) -> Result<Self> {
        let mut mesh = CPUMesh::square();
        mesh.transform(&(Mat4::from_scale(0.5) * Mat4::from_translation(vec3(1.0, 0.0, 0.0))));
        let mut line = Self {
            model: Model2D::new(context, &mesh)?,
            context: context.clone(),
            pixel0,
            pixel1,
            width,
        };
        line.update();
        Ok(line)
    }

    pub fn end_point0(&self) -> (f32, f32) {
        self.pixel0
    }

    pub fn end_point1(&self) -> (f32, f32) {
        self.pixel1
    }

    ///
    /// Change the two end points of the line.
    /// The pixel coordinates must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn set_endpoints(&mut self, pixel0: (f32, f32), pixel1: (f32, f32)) {
        self.pixel0 = pixel0;
        self.pixel1 = pixel1;
        self.update();
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.update();
    }

    fn update(&mut self) {
        let dx = self.pixel1.0 - self.pixel0.0;
        let dy = self.pixel1.1 - self.pixel0.1;
        let length = (dx * dx + dy * dy).sqrt();

        let c = dx / length;
        let s = dy / length;
        let rot = Mat3::new(c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0);
        self.model.set_transformation(
            Mat3::from_translation(vec2(self.pixel0.0, self.pixel0.1))
                * rot
                * Mat3::from_nonuniform_scale(length, self.width),
        );
    }
}

impl std::ops::Deref for Line {
    type Target = Model2D;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}