// src/visuals.rs

use rosc::OscPacket;

/// A simple structure representing the state of your visuals.
pub struct VisualEngine {
    pub intensity: f32,
}

impl VisualEngine {
    /// Create a new VisualEngine with default parameters.
    pub fn new() -> Self {
        Self { intensity: 0.5 }
    }

    /// Update the visual state, possibly processing an OSC packet.
    pub fn update(&mut self, osc_packet: Option<OscPacket>) {
        if let Some(packet) = osc_packet {
            println!("VisualEngine received OSC packet: {:?}", packet);
            // Example logic: adjust intensity based on OSC messages.
            self.intensity = (self.intensity + 0.1) % 1.0;
        }
        // Otherwise, update animations, particles, etc.
    }

    /// Render the visuals.
    pub fn render(&self) {
        println!("Rendering visuals with intensity: {:.2}", self.intensity);
    }
}
