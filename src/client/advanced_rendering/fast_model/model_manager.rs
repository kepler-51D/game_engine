use wgpu::{Device, RenderPass};

use crate::advanced_rendering::{extendable_buffer::BufferVec, fast_model::gltf::{self, DrawModelGltf}};

#[derive(Default)]
pub struct ModelManager {
    pub models: Vec<(gltf::Model,BufferVec)>,
}


pub trait RenderModelManager<'a> {
    fn render_gltf_models(&mut self,device: &Device,model_manager: &'a ModelManager);
}
impl<'a,'b> RenderModelManager<'b> for RenderPass<'a> 
where 
    'b: 'a
{
    /// requires camera bind group and light bind group to already be set
    fn render_gltf_models(&mut self,device: &Device ,model_manager: &'b ModelManager) {
        for (model, instance_buffer) in &model_manager.models {
            self.set_vertex_buffer(1, instance_buffer.buffer.slice(..));
            self.draw_model_gltf_instanced(device, model, 0..(instance_buffer.len() as u32));
        }
    }
}