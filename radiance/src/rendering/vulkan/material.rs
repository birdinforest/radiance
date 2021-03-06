use super::{shader::VulkanShader, texture::VulkanTexture};
use crate::rendering::vulkan::adhoc_command_runner::AdhocCommandRunner;
use crate::rendering::{Material, MaterialDef};
use ash::Device;
use std::rc::Rc;

pub struct VulkanMaterial {
    name: String,
    shader: VulkanShader,
    textures: Vec<VulkanTexture>,
}

impl Material for VulkanMaterial {}

impl std::fmt::Debug for VulkanMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("VulkanMaterial: {}", &self.name))
    }
}

impl VulkanMaterial {
    pub fn new(
        def: &MaterialDef,
        device: &Rc<Device>,
        allocator: &Rc<vk_mem::Allocator>,
        command_runner: &Rc<AdhocCommandRunner>,
    ) -> Self {
        let shader = VulkanShader::new(def.shader(), device).unwrap();
        let textures = def
            .textures()
            .iter()
            .map(|t| VulkanTexture::new(t, device, allocator, command_runner).unwrap())
            .collect();
        Self {
            name: def.name().to_string(),
            shader,
            textures,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn shader(&self) -> &VulkanShader {
        &self.shader
    }

    pub fn textures(&self) -> &[VulkanTexture] {
        &self.textures
    }
}
