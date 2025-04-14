use opencv::{
    core::{Mat, Point, Rect, Scalar, Vector},
    imgproc,
    prelude::*,
};
use crate::api::Item;

pub struct ItemDetector {
    templates: Vec<(Mat, Item)>,
    threshold: f64,
}

impl ItemDetector {
    pub fn new(threshold: f64) -> Self {
        Self {
            templates: Vec::new(),
            threshold,
        }
    }

    pub fn add_template(&mut self, template: Mat, item: Item) {
        self.templates.push((template, item));
    }

    pub fn detect_items(&self, frame: &Mat) -> Vec<(Rect, &Item)> {
        let mut detections = Vec::new();
        
        for (template, item) in &self.templates {
            if let Ok(result) = self.match_template(frame, template) {
                let mut max_val = 0.0;
                let mut max_loc = Point::new(0, 0);
                
                if let Ok(()) = opencv::core::min_max_loc(
                    &result,
                    None,
                    Some(&mut max_val),
                    None,
                    Some(&mut max_loc),
                    &Mat::default(),
                ) {
                    if max_val >= self.threshold {
                        let rect = Rect::new(
                            max_loc.x,
                            max_loc.y,
                            template.cols(),
                            template.rows(),
                        );
                        detections.push((rect, item));
                    }
                }
            }
        }
        
        detections
    }

    fn match_template(&self, frame: &Mat, template: &Mat) -> opencv::Result<Mat> {
        let mut result = Mat::default();
        imgproc::match_template(
            frame,
            template,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &Mat::default(),
        )?;
        Ok(result)
    }

    pub fn draw_detections(&self, frame: &mut Mat, detections: &[(Rect, &Item)]) -> opencv::Result<()> {
        for (rect, item) in detections {
            // Draw rectangle around detected item
            imgproc::rectangle(
                frame,
                *rect,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0,
            )?;

            // Draw item name and price
            let text = format!("{}: {}₽", item.name, item.price);
            imgproc::put_text(
                frame,
                &text,
                Point::new(rect.x, rect.y - 10),
                imgproc::FONT_HERSHEY_SIMPLEX,
                0.5,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                1,
                imgproc::LINE_AA,
                false,
            )?;
        }
        
        Ok(())
    }
} 