use cgmath::Matrix4;

use crate::c_str;
use crate::core::frame_context::FrameContext;
use crate::glutils::shader::Shader;
use crate::graphics::animation::AnimationComponent;
use crate::graphics::sprite::SpriteRendererComponent;
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

pub struct SpriteRenderSystem;

impl System for SpriteRenderSystem {
    fn render(&mut self, ctx: &mut FrameContext, shader: &Shader) {
        let transform_pool = ctx.world.get_components::<TransformComponent>()
            .expect("TransformComponent pool not found");
        let sprites_pool = ctx.world.get_components::<SpriteRendererComponent>()
            .expect("SpriteRendererComponent pool not found");

        for (entity_id, sprite) in sprites_pool.iter() {
            if let Some(transform_comp) = transform_pool.get(entity_id) {
                unsafe {
                    shader.use_program();
                    sprite.texture.active(0);
                    sprite.texture.bind();
                    shader.set_int(c_str!("texture_diffuse1"), 0);

                    let transform_matrix = transform_comp.transform.get_model_matrix();

                    // 2. On crée une matrice pour la taille de base de la texture
                    let base_size_matrix = Matrix4::from_nonuniform_scale(
                        sprite.width as f32, 
                        sprite.height as f32, 
                        1.0
                    );

                    let final_model_matrix = transform_matrix * base_size_matrix;
                    
                    shader.set_mat4(c_str!("model"), &final_model_matrix);

                    sprite.vao.bind();
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                    sprite.vao.unbind();
                }
            }
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

pub struct AnimationSystem;

impl System for AnimationSystem {
    fn update(&mut self, ctx: &mut FrameContext) {
        let animation_manager = &ctx.animation_manager;
        let spritesheet_manager = &ctx.spritesheet_manager;

        let entities_to_update: Vec<Entity> = {
            ctx.world.get_components::<AnimationComponent>()
                .map(|pool| pool.keys().copied().collect())
                .unwrap_or_else(Vec::new)
        };

        for entity_id in entities_to_update {
            let Some((anim_comp, sprite_comp)) = ctx.world.get_components_mut_pair::<AnimationComponent, SpriteRendererComponent>(entity_id) else {
                continue;
            };

            if !anim_comp.is_playing || anim_comp.current_animation.is_none() {
                continue;
            }

            let anim_name = anim_comp.current_animation.as_ref().unwrap();
            let animation = match animation_manager.get(anim_name) {
                Some(anim) => anim,
                None => continue,
            };

            anim_comp.timer += ctx.time.delta_time();

            if anim_comp.timer >= animation.frame_duration {
                anim_comp.timer -= animation.frame_duration;
                anim_comp.current_frame_index += 1;

                if anim_comp.current_frame_index >= animation.frames.len() {
                    if animation.loops {
                        anim_comp.current_frame_index = 0;
                    } else {
                        anim_comp.current_frame_index = animation.frames.len() - 1;
                        anim_comp.is_playing = false;
                    }
                }

                let spritesheet = spritesheet_manager.get(&animation.spritesheet_name).unwrap();
                let sprite_name = &animation.frames[anim_comp.current_frame_index];

                if let Some(sprite_data) = spritesheet.get_sprite(sprite_name) {
                    let positions: [f32; 12] = [
                        -0.5,  0.5, -0.5, -0.5,  0.5, -0.5,
                        -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,
                    ];

                    let mut tex_coords = sprite_data.tex_coords;
                    if animation.flipped {
                        let u_val1 = tex_coords[0]; 
                        let u_val2 = *tex_coords.iter().step_by(2).find(|&&u| u != u_val1).unwrap_or(&u_val1);

                        for i in 0..6 {
                            if tex_coords[i * 2] == u_val1 {
                                tex_coords[i * 2] = u_val2;
                            } else {
                                tex_coords[i * 2] = u_val1;
                            }
                        }
                    }

                    let mut new_vertices = [0.0f32; 24];
                    for i in 0..6 {
                        new_vertices[i * 4]     = positions[i * 2];
                        new_vertices[i * 4 + 1] = positions[i * 2 + 1];
                        new_vertices[i * 4 + 2] = tex_coords[i * 2];
                        new_vertices[i * 4 + 3] = tex_coords[i * 2 + 1];
                    }

                    sprite_comp.vbo.bind();
                    sprite_comp.vbo.set_data(&new_vertices);
                    sprite_comp.vbo.unbind();

                    sprite_comp.width = sprite_data.width;
                    sprite_comp.height = sprite_data.height;
                }
            }
        }
    }
}
