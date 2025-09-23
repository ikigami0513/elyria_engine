use crate::c_str;
use crate::core::frame_context::FrameContext;
use crate::glutils::shader::Shader;
use crate::graphics::cuboid_renderer::PrimitiveRenderComponent;
use crate::graphics::model::ModelRenderComponent;
use crate::world::components::{Parent, TransformComponent};
use crate::world::entity::Entity;

#[allow(unused_variables)]
pub trait System {
    fn update(&mut self, ctx: &mut FrameContext) {}
    fn render(&mut self, ctx: &mut FrameContext, shader: &Shader) {}
}

pub struct TransformSystem;

impl System for TransformSystem {
    fn update(&mut self, ctx: &mut FrameContext) {
        let parent_data: Vec<(Entity, Entity)> = {
            let parents_pool = ctx.world.get_components::<Parent>()
                .expect("ParentComponent pool not found");
            let transforms_pool = ctx.world.get_components::<TransformComponent>()
                .expect("TransformComponent pool not found");

            parents_pool.iter()
                .filter_map(|(&child_id, parent_comp)| {
                    if let Some(child_transform) = transforms_pool.get(&child_id) {
                        if child_transform.transform.is_dirty() {
                            return Some((child_id, parent_comp.0));
                        }
                    }
                    None
                })
                .collect()
        };

        let mut transforms_to_update: Vec<Entity> = Vec::new();

        {
            let transforms_pool = ctx.world.get_components::<TransformComponent>()
                .expect("TransformComponent pool not found");
            let parents_pool = ctx.world.get_components::<Parent>()
                .expect("ParentComponent pool not found");
            
            for (entity_id, transform_comp) in transforms_pool.iter() {
                if transform_comp.transform.is_dirty() && parents_pool.get(entity_id).is_none() {
                    transforms_to_update.push(*entity_id);
                }
            }
        }
        
        {
            let transforms_pool = ctx.world.get_components_mut::<TransformComponent>()
                .expect("TransformComponent pool not found");
            
            for entity_id in transforms_to_update {
                if let Some(transform_comp) = transforms_pool.get_mut(&entity_id) {
                    transform_comp.transform.compute_model_matrix();
                }
            }
        }

        let transforms_pool = ctx.world.get_components_mut::<TransformComponent>()
            .expect("TransformComponent pool not found");
        
        for (child_id, parent_id) in parent_data {
            if let Some(parent_transform) = transforms_pool.get(&parent_id).cloned() {
                if let Some(child_transform) = transforms_pool.get_mut(&child_id) {
                     child_transform.transform.compute_model_matrix_with_parent(
                         &parent_transform.transform.get_model_matrix()
                     );
                }
            }
        }
    }
}

pub struct ModelRenderSystem;

impl System for ModelRenderSystem {
    fn render(&mut self, ctx: &mut FrameContext, shader: &Shader) {
        let transform_pool = ctx.world.get_components::<TransformComponent>()
            .expect("TransformComponent pool not found");
        let models_pool = ctx.world.get_components::<ModelRenderComponent>()
            .expect("ModelRenderComponent pool not found");

        for (entity_id, model) in models_pool.iter() {
            if let Some(transform_comp) = transform_pool.get(entity_id) {
                unsafe {
                    shader.use_program();
                    shader.set_mat4(c_str!("model"), &transform_comp.transform.get_model_matrix());
                }
                for mesh in &model.meshes {
                    unsafe { mesh.render(shader); }
                }
            }
        }
    }
}

pub struct PrimitiveRenderSystem;

impl System for PrimitiveRenderSystem {
    fn render(&mut self, ctx: &mut FrameContext, shader: &Shader) {
        let transform_pool = ctx.world.get_components::<TransformComponent>()
            .expect("TransformComponent pool not found");
        let primitives_pool = ctx.world.get_components::<PrimitiveRenderComponent>()
            .expect("PrimitiveRenderComponent pool not found");

        for (entity_id, primitive) in primitives_pool.iter() {
            if let Some(transform_comp) = transform_pool.get(entity_id) {
                unsafe {
                    shader.use_program();

                    primitive.texture.active(0);
                    primitive.texture.bind();
                    shader.set_int(c_str!("texture_diffuse1"), 0);

                    shader.set_mat4(c_str!("model"), &transform_comp.transform.get_model_matrix());

                    primitive.vao.bind();
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }

            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }            
        }
    }
}
