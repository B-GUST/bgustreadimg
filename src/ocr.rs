use ort::session::{Session, builder::GraphOptimizationLevel};

pub struct OcrEngine {
    session: Session,
}

impl OcrEngine {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let _ = ort::init().with_name("surya-ocr").commit();

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

    pub fn recognize_text(&self, _image_data: &[u8]) -> Result<String, String> {
        // En producción: decodificar imagen, convertir a tensor, llamar al session.run()
        Ok(String::new())
    }
}
