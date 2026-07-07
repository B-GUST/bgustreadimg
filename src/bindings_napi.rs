use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::{preprocess_image_rs, PreprocessConfigRs};

#[napi(object)]
pub struct PreprocessConfig {
  pub window_size: Option<u32>,
  pub k: Option<f64>,
  pub target_width: Option<u32>,
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
