use super::{
    adhoc_command_runner::AdhocCommandRunner, descriptor_managers::DescriptorManager,
    material::VulkanMaterial, render_object::VulkanRenderObject, shader::VulkanShader,
    texture::VulkanTexture, uniform_buffers::DynamicUniformBufferManager,
};
use crate::rendering::{
    factory::ComponentFactory, texture::TextureDef, Material, MaterialDef, RenderObject, Shader,
    ShaderDef, Texture, VertexBuffer,
};
use ash::Device;
use std::rc::Rc;
use std::sync::Arc;

pub struct VulkanComponentFactory {
    device: Rc<Device>,
    allocator: Rc<vk_mem::Allocator>,
    descriptor_manager: Rc<DescriptorManager>,
    dub_manager: Arc<DynamicUniformBufferManager>,
    command_runner: Rc<AdhocCommandRunner>,
}

impl ComponentFactory for VulkanComponentFactory {
    fn create_texture(&self, texture_def: &TextureDef) -> Box<dyn Texture> {
        Box::new(
            VulkanTexture::new(
                texture_def,
                &self.device,
                &self.allocator,
                &self.command_runner,
            )
            .unwrap(),
        )
    }

    fn create_shader(&self, shader_def: &ShaderDef) -> Box<dyn Shader> {
        Box::new(VulkanShader::new(shader_def, &self.device).unwrap())
    }

    fn create_material(&self, material_def: &MaterialDef) -> Box<dyn Material> {
        Box::new(VulkanMaterial::new(
            material_def,
            &self.device,
            &self.allocator,
            &self.command_runner,
        ))
    }

    fn create_render_object(
        &self,
        vertices: VertexBuffer,
        indices: Vec<u32>,
        material_def: &MaterialDef,
        host_dynamic: bool,
    ) -> Box<dyn RenderObject> {
        let material = self.create_material(material_def);
        Box::new(
            VulkanRenderObject::new(
                vertices,
                indices,
                material,
                host_dynamic,
                &self.device,
                &self.allocator,
                &self.command_runner,
                &self.dub_manager,
                &self.descriptor_manager,
            )
            .unwrap(),
        )
    }
}

impl VulkanComponentFactory {
    pub fn new(
        device: &Rc<Device>,
        allocator: &Rc<vk_mem::Allocator>,
        descriptor_manager: &Rc<DescriptorManager>,
        dub_manager: &Arc<DynamicUniformBufferManager>,
        command_runner: &Rc<AdhocCommandRunner>,
    ) -> Self {
        Self {
            device: device.clone(),
            allocator: allocator.clone(),
            descriptor_manager: descriptor_manager.clone(),
            dub_manager: dub_manager.clone(),
            command_runner: command_runner.clone(),
        }
    }

    pub fn as_component_factory(self: &Rc<Self>) -> Rc<dyn ComponentFactory> {
        self.clone()
    }
}
