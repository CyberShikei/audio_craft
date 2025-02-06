// src/vulkan_renderer.rs

use ash::{extensions::khr::Surface, vk, Entry, Instance};
use ash_window;
use std::{
    error::Error,
    ffi::{CStr, CString},
};
use winit::window::Window;

pub struct VulkanRenderer {
    _entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    surface_loader: Surface,
}

impl VulkanRenderer {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        // Load Vulkan function pointers.
        let entry = unsafe { Entry::load()? };

        // Application Info.
        let app_name = CString::new("Audio Viz Engine")?;
        let engine_name = CString::new("No Engine")?;
        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(vk::API_VERSION_1_0)
            .engine_name(&engine_name)
            .engine_version(vk::API_VERSION_1_0)
            .api_version(vk::API_VERSION_1_0);

        // Get the required extensions from the window.
        let raw_extensions = ash_window::enumerate_required_extensions(window)?;

        // Filter out any null pointers.
        let valid_extensions: Vec<*const i8> = raw_extensions
            .iter()
            .copied()
            .filter(|&ext| {
                if ext.is_null() {
                    eprintln!("Warning: found a null extension pointer and filtering it out");
                    false
                } else {
                    true
                }
            })
            .collect();

        // Check that we have at least one valid extension.
        if valid_extensions.is_empty() {
            return Err("No valid extension names found!".into());
        }

        // Debug print extension names.
        for &ext in &valid_extensions {
            // Safe because we've filtered out null pointers.
            let ext_str = unsafe { CStr::from_ptr(ext) };
            println!("Using extension: {}", ext_str.to_string_lossy());
        }

        // Create the Vulkan instance.
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&valid_extensions);

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        // Create a Vulkan surface from the winit window.
        let surface = unsafe { ash_window::create_surface(&entry, &instance, window, None)? };

        // Create the surface loader.
        let surface_loader = Surface::new(&entry, &instance);

        Ok(Self {
            _entry: entry,
            instance,
            surface,
            surface_loader,
        })
    }

    /// Dummy function to simulate drawing a frame.
    pub fn draw_frame(&mut self) {
        // In a full implementation, this would acquire a swapchain image,
        // record command buffers, submit them, etc.
        println!("Rendering frame...");
    }

    /// Cleanup Vulkan resources.
    pub fn cleanup(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        self.cleanup();
    }
}
