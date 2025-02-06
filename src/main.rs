// src/main.rs

mod osc_listener;
mod visuals;
mod vulkan_renderer;

use std::sync::mpsc;
use std::thread;
// use std::time::Duration;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use osc_listener::start_osc_listener;
use visuals::VisualEngine;
use vulkan_renderer::VulkanRenderer;

fn main() {
    // Initialize logger (if using log and env_logger)
    env_logger::init();

    // Create a winit event loop and window.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Audio Reactive Visuals")
        .build(&event_loop)
        .expect("Failed to create window");

    // Channel for OSC messages (if you want to pass data from the OSC listener to the renderer)
    let (osc_tx, osc_rx) = mpsc::channel();

    // Spawn the OSC listener on a separate thread.
    thread::spawn(move || {
        start_osc_listener(osc_tx);
    });

    // Create and initialize the Vulkan renderer.
    let mut renderer = VulkanRenderer::new(&window).expect("Failed to initialize Vulkan renderer");

    // Create the VisualEngine.
    let mut visual_engine = VisualEngine::new();

    // Use the winit event loop to drive rendering.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                // Process pending OSC messages.
                while let Ok(osc_message) = osc_rx.try_recv() {
                    visual_engine.update(Some(osc_message));
                }
                // Update visuals even if no OSC messages.
                visual_engine.update(None);

                // Render your visuals (update uniforms, animations, etc.).
                visual_engine.render();

                // Render a frame via Vulkan.
                renderer.draw_frame();

                // Request a redraw of the window.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // You can perform additional per-frame tasks here if needed.
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
