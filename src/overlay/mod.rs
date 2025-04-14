use anyhow::{Result, Context};
use obs_sys::{obs_source_t, gs_effect_t};
use opencv::core::{Rect, Scalar};
use crate::api::Item;
use log::{info, warn, error};

pub struct OverlayRenderer {
    highlight_color: [f32; 4],
    tooltip_font_size: f32,
    tooltip_font_color: [f32; 4],
}

impl OverlayRenderer {
    pub fn new(highlight_color: [f32; 4], tooltip_font_size: f32, tooltip_font_color: [f32; 4]) -> Self {
        Self {
            highlight_color,
            tooltip_font_size,
            tooltip_font_color,
        }
    }

    pub fn update_settings(&mut self, highlight_color: [f32; 4], tooltip_font_size: f32, tooltip_font_color: [f32; 4]) {
        self.highlight_color = highlight_color;
        self.tooltip_font_size = tooltip_font_size;
        self.tooltip_font_color = tooltip_font_color;
    }

    pub fn draw_highlight(&self, source: *mut obs_source_t, rect: &Rect) {
        unsafe {
            // Convert our color from [0-1] to OBS GS format
            let color = gs_color_from_rgba(
                (self.highlight_color[0] * 255.0) as u8,
                (self.highlight_color[1] * 255.0) as u8,
                (self.highlight_color[2] * 255.0) as u8,
                (self.highlight_color[3] * 255.0) as u8,
            );
            
            // Draw a rectangle using OBS graphics API
            gs_draw_sprite_rect(
                rect.x as i32,
                rect.y as i32,
                rect.width as u32,
                rect.height as u32,
                color,
                2, // Border thickness
            );
        }
    }

    pub fn draw_tooltip(&self, source: *mut obs_source_t, rect: &Rect, item: &Item) {
        unsafe {
            // Format price with thousand separators
            let price_text = format_price(item.price);
            
            // Format the tooltip text
            let tooltip_text = format!("{}: {}₽", item.name, price_text);
            
            // Convert our color from [0-1] to OBS GS format
            let color = gs_color_from_rgba(
                (self.tooltip_font_color[0] * 255.0) as u8,
                (self.tooltip_font_color[1] * 255.0) as u8,
                (self.tooltip_font_color[2] * 255.0) as u8,
                (self.tooltip_font_color[3] * 255.0) as u8,
            );
            
            // Calculate tooltip position (above the item)
            let x = rect.x as i32;
            let y = rect.y as i32 - 30; // 30 pixels above the item
            
            // Draw tooltip background
            draw_tooltip_background(x, y, tooltip_text.len() as u32 * 10, 30);
            
            // Draw tooltip text
            draw_text(
                &tooltip_text,
                x + 5, // Add some padding
                y + 20, // Center vertically in background
                self.tooltip_font_size,
                color,
            );
        }
    }
}

// Helper function to format price with thousand separators
fn format_price(price: i32) -> String {
    let price_str = price.to_string();
    let mut result = String::with_capacity(price_str.len() + price_str.len() / 3);
    let mut count = 0;
    
    for (i, c) in price_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    
    result.chars().rev().collect()
}

// Wrapper functions for OBS graphics API
unsafe fn gs_color_from_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

unsafe fn gs_draw_sprite_rect(x: i32, y: i32, width: u32, height: u32, color: u32, border_size: i32) {
    // This would need to be implemented with actual OBS graphics API calls
    // Currently just a placeholder
}

unsafe fn draw_tooltip_background(x: i32, y: i32, width: u32, height: u32) {
    // This would need to be implemented with actual OBS graphics API calls
    // Currently just a placeholder
}

unsafe fn draw_text(text: &str, x: i32, y: i32, font_size: f32, color: u32) {
    // This would need to be implemented with actual OBS graphics API calls
    // Currently just a placeholder
} 