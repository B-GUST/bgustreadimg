use ort::session::{Session, builder::GraphOptimizationLevel};

pub struct LayoutAnalyzer {
    session: Session,
}

#[derive(Debug)]
pub struct TableRegion {
    pub xmin: f32,
    pub ymin: f32,
    pub xmax: f32,
    pub ymax: f32,
    pub confidence: f32,
}

impl LayoutAnalyzer {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let _ = ort::init().with_name("table-transformer").commit();

        let session = Session::builder()
            .map_err(|e| format!("Failed to create SessionBuilder: {}", e))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| format!("Optimization configuration failed: {}", e))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread configuration failed: {}", e))?
            .commit_from_file(model_path)
            .map_err(|e| format!("Failed to load ONNX model from {}: {}", model_path, e))?;

        Ok(Self { session })
    }

    pub fn detect_tables(&self, _image_data: &[u8]) -> Result<Vec<TableRegion>, String> {
        // En producción: decodificar imagen y preprocesar (resize, normalize)
        Ok(vec![])
    }
}
