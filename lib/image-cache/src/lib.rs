use image::{DynamicImage, ImageReader};
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Clone)]
pub struct ImageCache {
    cache_dir: PathBuf,
}

impl ImageCache {
    /// Creates a new ImageCache instance with the default cache directory
    /// (~/.cache/pokedex/images/)
    pub fn new() -> std::io::Result<Self> {
        let cache_dir = Self::default_cache_dir()?;
        Ok(Self { cache_dir })
    }

    /// Returns the default cache directory path
    fn default_cache_dir() -> std::io::Result<PathBuf> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not determine home directory",
                )
            })?;

        Ok(PathBuf::from(home)
            .join(".cache")
            .join("pokedex")
            .join("images"))
    }

    /// Ensures the cache directory exists, creating it if necessary
    async fn ensure_cache_dir(&self) -> std::io::Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir).await?;
        }
        Ok(())
    }

    /// Returns the cached image path for a given Pokemon ID
    fn cache_path(&self, pokemon_id: u32) -> PathBuf {
        self.cache_dir.join(format!("{pokemon_id}_scaled.png"))
    }

    /// Gets an image from cache or loads it from the original path
    ///
    /// If the image is in cache, it's loaded from there (fast).
    /// If not, it's loaded from the original path, saved to cache, and returned.
    pub async fn get_or_load(
        &self,
        pokemon_id: u32,
        original_path: &Path,
    ) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_cache_dir().await?;

        let cache_path = self.cache_path(pokemon_id);

        // Try to load from cache first
        if cache_path.exists() {
            // Load from cache asynchronously
            let image = tokio::task::spawn_blocking(move || {
                ImageReader::open(&cache_path)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                    .decode()
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            })
            .await??;

            return Ok(image);
        }

        // Load original image (blocking operation in a separate task)
        let original_path = original_path.to_path_buf();
        let image = tokio::task::spawn_blocking(move || {
            ImageReader::open(&original_path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                .decode()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
        .await??;

        // Save to cache asynchronously (fire and forget - don't block on save)
        let cache_path_clone = cache_path.clone();
        let image_clone = image.clone();
        tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || image_clone.save(&cache_path_clone)).await;
        });

        Ok(image)
    }
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new().expect("Failed to create image cache")
    }
}
