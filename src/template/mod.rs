use anyhow::{Result, Context};
use opencv::{
    core::Mat,
    imgcodecs,
    prelude::*,
};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, error};
use crate::api::{TarkovMarketAPI, Item};

pub struct TemplateManager {
    template_dir: PathBuf,
    templates: Arc<RwLock<HashMap<String, Mat>>>,
}

impl TemplateManager {
    pub fn new<P: AsRef<Path>>(template_dir: P) -> Self {
        Self {
            template_dir: template_dir.as_ref().to_path_buf(),
            templates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn load_templates(&self) -> Result<()> {
        let mut templates = self.templates.write().await;
        templates.clear();

        // Create directory if it doesn't exist
        if !self.template_dir.exists() {
            fs::create_dir_all(&self.template_dir)
                .context(format!("Failed to create template directory: {:?}", self.template_dir))?;
        }

        // Walk through the template directory
        let entries = fs::read_dir(&self.template_dir)
            .context(format!("Failed to read template directory: {:?}", self.template_dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Skip non-image files (we only support PNG for templates)
            if path.extension().map_or(false, |ext| ext == "png") {
                let filename = path.file_stem()
                    .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                    .to_string_lossy()
                    .to_string();

                // The filename should be the item UID
                let item_uid = filename;

                // Load the template
                let template = imgcodecs::imread(
                    path.to_str().unwrap(),
                    imgcodecs::IMREAD_GRAYSCALE,
                )?;

                // Store the template
                templates.insert(item_uid, template);
                info!("Loaded template: {}", path.display());
            }
        }

        info!("Loaded {} templates", templates.len());
        Ok(())
    }

    pub async fn download_templates(&self, api: &TarkovMarketAPI) -> Result<usize> {
        // Get all items from the API
        let items = api.get_all_items().await?;
        let mut downloaded_count = 0;

        // Create directory if it doesn't exist
        if !self.template_dir.exists() {
            fs::create_dir_all(&self.template_dir)
                .context(format!("Failed to create template directory: {:?}", self.template_dir))?;
        }

        // Process each item
        for item in items {
            // Only download if the template doesn't already exist
            let template_path = self.template_dir.join(format!("{}.png", item.uid));
            if template_path.exists() {
                continue;
            }

            // Skip items with no icons
            if item.icon.is_empty() && item.img.is_empty() {
                continue;
            }

            // Choose the image URL (prefer icon over img)
            let image_url = if !item.icon.is_empty() {
                &item.icon
            } else {
                &item.img
            };

            // Download the image
            match download_image(image_url, &template_path).await {
                Ok(_) => {
                    info!("Downloaded template for item: {} ({})", item.name, item.uid);
                    downloaded_count += 1;

                    // Load the template into memory
                    match imgcodecs::imread(template_path.to_str().unwrap(), imgcodecs::IMREAD_GRAYSCALE) {
                        Ok(template) => {
                            let mut templates = self.templates.write().await;
                            templates.insert(item.uid, template);
                        },
                        Err(e) => {
                            error!("Failed to load downloaded template: {}", e);
                            // Clean up the failed download
                            if template_path.exists() {
                                let _ = fs::remove_file(&template_path);
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to download template for item {}: {}", item.uid, e);
                }
            }
        }

        info!("Downloaded {} new templates", downloaded_count);
        Ok(downloaded_count)
    }

    pub async fn process_templates(&self) -> Result<()> {
        let mut templates = self.templates.write().await;
        let template_paths: Vec<PathBuf> = fs::read_dir(&self.template_dir)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().map_or(false, |ext| ext == "png"))
            .collect();

        for path in template_paths {
            let item_uid = path.file_stem()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                .to_string_lossy()
                .to_string();

            if templates.contains_key(&item_uid) {
                continue;
            }

            // Load and process the template
            let mut template = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_GRAYSCALE)?;
            
            // Process the template (resize, enhance edges, etc.)
            opencv::imgproc::gaussian_blur(
                &template,
                &mut template,
                opencv::core::Size::new(3, 3),
                0.0,
                0.0,
                opencv::core::BORDER_DEFAULT,
            )?;

            // Store processed template
            templates.insert(item_uid, template);
            info!("Processed template: {}", path.display());
        }

        Ok(())
    }

    pub async fn get_template(&self, item_uid: &str) -> Option<Mat> {
        let templates = self.templates.read().await;
        templates.get(item_uid).cloned()
    }

    pub async fn get_all_template_ids(&self) -> Vec<String> {
        let templates = self.templates.read().await;
        templates.keys().cloned().collect()
    }

    pub async fn add_template(&self, item_uid: &str, template: Mat) -> Result<()> {
        // Save the template to disk
        let template_path = self.template_dir.join(format!("{}.png", item_uid));
        imgcodecs::imwrite(
            template_path.to_str().unwrap(),
            &template,
            &opencv::core::Vector::new(),
        )?;

        // Add to in-memory cache
        let mut templates = self.templates.write().await;
        templates.insert(item_uid.to_string(), template);

        info!("Added template for item: {}", item_uid);
        Ok(())
    }

    pub async fn remove_template(&self, item_uid: &str) -> Result<()> {
        // Remove from in-memory cache
        let mut templates = self.templates.write().await;
        templates.remove(item_uid);

        // Remove from disk
        let template_path = self.template_dir.join(format!("{}.png", item_uid));
        if template_path.exists() {
            fs::remove_file(&template_path)
                .context(format!("Failed to remove template file: {:?}", template_path))?;
        }

        info!("Removed template for item: {}", item_uid);
        Ok(())
    }

    pub async fn get_template_count(&self) -> usize {
        let templates = self.templates.read().await;
        templates.len()
    }
}

async fn download_image(url: &str, output_path: &Path) -> Result<()> {
    // Create a reqwest client
    let client = reqwest::Client::new();
    
    // Fetch the image
    let response = client.get(url)
        .send()
        .await
        .context("Failed to fetch image")?;
    
    // Check status
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to download image: HTTP {}", response.status()));
    }
    
    // Get image bytes
    let bytes = response.bytes()
        .await
        .context("Failed to read image bytes")?;
    
    // Save to file
    fs::write(output_path, &bytes)
        .context("Failed to write image to file")?;
    
    Ok(())
} 