use dust::model;
use glm;

#[derive(Debug)]
pub enum Error {
    Model(model::Error)
}

impl From<model::Error> for Error {
    fn from(other: model::Error) -> Self {
        Error::Model(other)
    }
}

pub struct Scene {
    models: Vec<model::Model>
}


impl Scene
{
    pub fn create() -> Result<Scene, Error>
    {
        Ok(Scene { models: Vec::new() })
    }

    pub fn add_model(&mut self, model: model::Model)
    {
        &self.models.push(model);
    }

    pub fn draw(&self, screen_width: &u32, screen_height: &u32, camera_position: &glm::Vec3, view: &glm::Matrix4<f32>, projection: &glm::Matrix4<f32>) -> Result<(), Error>
    {
        for model in &self.models {
            model.draw(screen_width, screen_height, camera_position, view, projection)?;
        }
        Ok(())
    }
}
