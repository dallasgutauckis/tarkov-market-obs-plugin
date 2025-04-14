use anyhow::{Result, Context};
use opencv::{
    core::{Mat, Point, Rect, Scalar, Vector},
    imgproc,
    prelude::*,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, error};
use crate::template::TemplateManager;

pub struct FrameCapture {
    width: i32,
    height: i32,
    last_frame: Arc<RwLock<Option<Mat>>>,
    preprocessed_frame: Arc<RwLock<Option<Mat>>>,
}

impl FrameCapture {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            last_frame: Arc::new(RwLock::new(None)),
            preprocessed_frame: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn capture_frame(&self, frame: &Mat) -> Result<()> {
        // Store the raw frame
        let mut last_frame = self.last_frame.write().await;
        *last_frame = Some(frame.clone());

        // Preprocess the frame for better detection
        let mut preprocessed = frame.clone();
        self.preprocess_frame(&mut preprocessed)?;

        // Store the preprocessed frame
        let mut preprocessed_frame = self.preprocessed_frame.write().await;
        *preprocessed_frame = Some(preprocessed);

        Ok(())
    }

    fn preprocess_frame(&self, frame: &mut Mat) -> Result<()> {
        // Convert to grayscale
        imgproc::cvt_color(frame, frame, imgproc::COLOR_BGR2GRAY, 0)?;

        // Apply Gaussian blur to reduce noise
        imgproc::gaussian_blur(
            frame,
            frame,
            opencv::core::Size::new(5, 5),
            0.0,
            0.0,
            opencv::core::BORDER_DEFAULT,
        )?;

        // Apply adaptive thresholding
        imgproc::adaptive_threshold(
            frame,
            frame,
            255.0,
            imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
            imgproc::THRESH_BINARY,
            11,
            2.0,
        )?;

        Ok(())
    }

    pub async fn get_last_frame(&self) -> Option<Mat> {
        self.last_frame.read().await.clone()
    }

    pub async fn get_preprocessed_frame(&self) -> Option<Mat> {
        self.preprocessed_frame.read().await.clone()
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

pub struct TemplateMatcher {
    templates: Arc<RwLock<Option<TemplateManager>>>,
    threshold: f64,
}

impl TemplateMatcher {
    pub fn new(threshold: f64) -> Self {
        Self {
            templates: Arc::new(RwLock::new(None)),
            threshold,
        }
    }

    pub fn set_template_manager(&mut self, template_manager: Arc<TemplateManager>) {
        let mut templates = self.templates.blocking_write();
        *templates = Some(template_manager.clone());
    }

    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold;
    }

    pub async fn match_templates(&self, frame: &Mat) -> Result<Vec<(Rect, String)>> {
        let mut matches = Vec::new();
        
        // Get the template manager
        let templates = self.templates.read().await;
        if templates.is_none() {
            return Ok(matches);
        }
        
        let template_manager = templates.as_ref().unwrap();
        
        // Get all template IDs
        let template_ids = template_manager.get_all_template_ids().await;
        
        // Match each template
        for item_uid in template_ids {
            // Get the template
            if let Some(template) = template_manager.get_template(&item_uid).await {
                // Match the template
                let mut result = Mat::default();
                imgproc::match_template(
                    frame,
                    &template,
                    &mut result,
                    imgproc::TM_CCOEFF_NORMED,
                    &Mat::default(),
                )?;
                
                // Find the best match
                let mut max_val = 0.0;
                let mut max_loc = Point::new(0, 0);
                opencv::core::min_max_loc(
                    &result,
                    None,
                    Some(&mut max_val),
                    None,
                    Some(&mut max_loc),
                    &Mat::default(),
                )?;
                
                // If the match is good enough, add it to the matches
                if max_val >= self.threshold {
                    let rect = Rect::new(
                        max_loc.x,
                        max_loc.y,
                        template.cols(),
                        template.rows(),
                    );
                    matches.push((rect, item_uid.clone()));
                }
            }
        }
        
        Ok(matches)
    }
} 