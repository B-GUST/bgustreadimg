#![deny(clippy::all)]

use image::{ImageFormat, io::Reader, GrayImage};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::io::Cursor;
use sysinfo::{System, SystemExt};

pub mod layout;
pub mod ocr;

#[derive(Clone)]
pub struct PreprocessConfigRs {
  pub window_size: Option<u32>,
  pub k: Option<f64>,
  pub target_width: Option<u32>,
}

#[napi(object)]
pub struct PreprocessConfig {
  pub window_size: Option<u32>,
  pub k: Option<f64>,
  pub target_width: Option<u32>,
}

pub fn sauvola_threshold(img: &GrayImage, window_size: u32, k: f32) -> GrayImage {
  let width = img.width() as i32;
  let height = img.height() as i32;
  let mut out_img = img.clone();

  // Asegurar que el tamaño de ventana sea impar y >= 3
  let w = (window_size as i32).max(3);
  let half = w / 2;

  let stride = width as usize + 1;
  let mut sat = vec![0u64; (width as usize + 1) * (height as usize + 1)];
  let mut sat_sq = vec![0u64; (width as usize + 1) * (height as usize + 1)];

  // 1. Construir las imágenes integrales (SAT y SAT_SQ) en O(N)
  for y in 0..height {
    let mut row_sum = 0u64;
    let mut row_sum_sq = 0u64;
    for x in 0..width {
      let val = img.get_pixel(x as u32, y as u32).0[0] as u64;
      row_sum += val;
      row_sum_sq += val * val;

      let idx = (y as usize + 1) * stride + (x as usize + 1);
      let prev_row_idx = (y as usize) * stride + (x as usize + 1);

      sat[idx] = sat[prev_row_idx] + row_sum;
      sat_sq[idx] = sat_sq[prev_row_idx] + row_sum_sq;
    }
  }

  // 2. Calcular umbral adaptativo local para cada píxel en O(1) usando las SAT
  for y in 0..height {
    for x in 0..width {
      let x1 = (x - half).max(0);
      let x2 = (x + half).min(width - 1);
      let y1 = (y - half).max(0);
      let y2 = (y + half).min(height - 1);

      let count = ((x2 - x1 + 1) * (y2 - y1 + 1)) as f64;

      let idx_a = (y1 as usize) * stride + (x1 as usize);
      let idx_b = (y1 as usize) * stride + (x2 as usize + 1);
      let idx_c = (y2 as usize + 1) * stride + (x1 as usize);
      let idx_d = (y2 as usize + 1) * stride + (x2 as usize + 1);

      let sum = sat[idx_d] as f64 - sat[idx_b] as f64 - sat[idx_c] as f64 + sat[idx_a] as f64;
      let sum_sq = sat_sq[idx_d] as f64 - sat_sq[idx_b] as f64 - sat_sq[idx_c] as f64 + sat_sq[idx_a] as f64;

      let mean = sum / count;
      let variance = (sum_sq / count) - (mean * mean);
      let std_dev = variance.max(0.0).sqrt();

      // Fórmula de Sauvola: T = mean * (1 + k * (std_dev / 128.0 - 1.0))
      let threshold = mean * (1.0 + k as f64 * (std_dev / 128.0 - 1.0));

      let pixel_val = img.get_pixel(x as u32, y as u32).0[0] as f64;
      let out_pixel = if pixel_val < threshold { 0 } else { 255 };

      out_img.put_pixel(x as u32, y as u32, image::Luma([out_pixel]));
    }
  }

  out_img
}

pub async fn preprocess_image_rs(data: Vec<u8>, config: Option<PreprocessConfigRs>) -> std::result::Result<Vec<u8>, String> {
  // Obtener parámetros de configuración con valores por defecto
  let (window_size, k_param, user_target_width) = match config {
    Some(cfg) => (
      cfg.window_size.unwrap_or(25),
      cfg.k.unwrap_or(0.2) as f32,
      cfg.target_width,
    ),
    None => (25, 0.2f32, None),
  };

  let result = tokio::task::spawn_blocking(move || {
    // 1. Verificar memoria RAM libre si no hay ancho objetivo especificado
    let target_width = match user_target_width {
      Some(w) => w,
      None => {
        let mut system = System::new_all();
        system.refresh_memory();
        let free_ram_kb = system.available_memory();
        let free_ram_mb = free_ram_kb / 1024;
        if free_ram_mb < 400 {
          1280
        } else {
          1920
        }
      }
    };

    // 2. Cargar metadatos rápida sin decodificar toda la imagen en RAM
    let reader = Reader::new(Cursor::new(&data))
      .with_guessed_format()
      .map_err(|e| format!("Formato no soportado: {}", e))?;

    let format = reader.format().ok_or_else(|| {
      "No se pudo determinar el formato de imagen".to_string()
    })?;

    let dimensions = reader.into_dimensions().map_err(|e| {
      format!("No se pudieron leer las dimensiones de la imagen: {}", e)
    })?;

    let width = dimensions.0;
    let height = dimensions.1;

    // 3. Decodificar la imagen real
    let img = image::load_from_memory_with_format(&data, format).map_err(|e| {
      format!("Error al decodificar la imagen: {}", e)
    })?;

    // 4. Redimensionar si supera la resolución objetivo
    let resized_img = if width > target_width {
      let filter = image::imageops::FilterType::Lanczos3; // Conservar alta nitidez
      img.resize(target_width, ((target_width as f32 / width as f32) * height as f32) as u32, filter)
    } else {
      img
    };

    // 5. Convertir a escala de grises (Luma8)
    let gray_img = resized_img.to_luma8();

    // 6. Binarización adaptativa local de Sauvola (remueve sombras y arrugas)
    let contrast_img = sauvola_threshold(&gray_img, window_size, k_param);

    // 7. Escribir búfer de salida como PNG sin pérdidas
    let mut output_buf = Vec::new();
    contrast_img.write_to(&mut Cursor::new(&mut output_buf), ImageFormat::Png).map_err(|e| {
      format!("Error al codificar imagen de salida: {}", e)
    })?;

    Ok(output_buf)
  }).await.map_err(|e| format!("Fallo del hilo de trabajo: {}", e))?;

  result
}

#[napi]
pub async fn preprocess_image(input_buf: Buffer, config: Option<PreprocessConfig>) -> Result<Buffer> {
  let data = input_buf.to_vec();
  let rs_config = config.map(|c| PreprocessConfigRs {
      window_size: c.window_size,
      k: c.k,
      target_width: c.target_width,
  });

  match preprocess_image_rs(data, rs_config).await {
      Ok(buf) => Ok(Buffer::from(buf)),
      Err(e) => Err(Error::new(Status::GenericFailure, e)),
  }
}
