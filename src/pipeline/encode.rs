/*
 * Copyright 2025 Mykyta Zakharov
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::core::encoder::Encoder;
use crate::pipeline::parser::{PipelineConfig, StageConfig};
use crate::sgn::encoder::{SgnEncoderX64, SgnEncoderX32, SgnEncoderAArch64};
use crate::xor_dynamic::encoder::XorDynamicEncoderX64;
use crate::schema::encoder::{SchemaEncoderX64, SchemaEncoderX32, SchemaEncoderAArch64};

/// A processing stage that transforms bytes.
pub trait Stage: Send + Sync {
    /// Process input bytes and return transformed bytes.
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, String>;
}

#[allow(dead_code)]
pub struct Pipeline {
    stages: Vec<Box<dyn Stage>>,
}

#[allow(dead_code)]
impl Pipeline {
    /// Create an empty pipeline.
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    /// Create a pipeline from a YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        let config = PipelineConfig::from_yaml(yaml)?;
        Self::from_config(&config)
    }

    /// Create a pipeline from a YAML file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, String> {
        let config = PipelineConfig::from_file(path)?;
        Self::from_config(&config)
    }

    /// Create a pipeline from a YAML configuration
    pub fn from_config(config: &PipelineConfig) -> Result<Self, String> {
        config.validate()?;

        let mut pipeline = Self::new();

        for stage_config in &config.pipeline.stages {
            let stage = create_stage_from_config(stage_config)?;
            pipeline = pipeline.with_stage(stage);
        }

        Ok(pipeline)
    }

    /// Add a stage to the pipeline (builder style).
    pub fn with_stage(mut self, stage: Box<dyn Stage>) -> Self {
        self.stages.push(stage);
        self
    }

    /// Execute the pipeline over the provided input.
    pub fn run(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        let mut data = input.to_vec();
        for stage in &self.stages {
            data = stage.process(&data)?;
        }
        Ok(data)
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a stage from configuration
fn create_stage_from_config(config: &StageConfig) -> Result<Box<dyn Stage>, String> {
    match config.stage_type.as_str() {
        "sgn" => create_sgn_stage(config),
        "xor_dynamic" => create_xor_dynamic_stage(config),
        "schema" => create_schema_stage(config),
        _ => Err(format!("Unknown stage type: {}", config.stage_type)),
    }
}

fn create_sgn_stage(config: &StageConfig) -> Result<Box<dyn Stage>, String> {
    match config.config.architecture.as_str() {
        "x64" => {
            let encoder = SgnEncoderX64::new(config.config.seed, config.config.plain_decoder);
            Ok(Box::new(EncoderStage { encoder }))
        }
        /*"x32" => {
            let encoder = SgnEncoderX32::new(config.config.seed, config.config.plain_decoder);
            Ok(Box::new(EncoderStage { encoder }))
        }
        "aarch64" => {
            let encoder = SgnEncoderAArch64::new(config.config.seed, config.config.plain_decoder);
            Ok(Box::new(EncoderStage { encoder }))
        }*/
        _ => Err(format!("Unsupported architecture for SGN: {}", config.config.architecture)),
    }
}

fn create_xor_dynamic_stage(config: &StageConfig) -> Result<Box<dyn Stage>, String> {
    match config.config.architecture.as_str() {
        "x64" => {
            let encoder = XorDynamicEncoderX64::new(config.config.seed);
            Ok(Box::new(EncoderStage { encoder }))
        }
        _ => Err(format!("Unsupported architecture for XorDynamic: {}", config.config.architecture)),
    }
}

fn create_schema_stage(config: &StageConfig) -> Result<Box<dyn Stage>, String> {
    match config.config.architecture.as_str() {
        "x64" => {
            let encoder = SchemaEncoderX64::new(config.config.seed);
            Ok(Box::new(EncoderStage { encoder }))
        }
        /*"x32" => {
            let encoder = SchemaEncoderX32::new(config.config.seed);
            Ok(Box::new(EncoderStage { encoder }))
        }
        "aarch64" => {
            let encoder = SchemaEncoderAArch64::new(config.config.seed);
            Ok(Box::new(EncoderStage { encoder }))
        }*/
        _ => Err(format!("Unsupported architecture for Schema: {}", config.config.architecture)),
    }
}

/// Wrapper to adapt an Encoder to a Stage
struct EncoderStage<E> {
    encoder: E,
}

impl<E> Stage for EncoderStage<E>
where
    E: Encoder + Send + Sync,
    E::Error: std::fmt::Display,
{
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        self.encoder.encode(data).map_err(|e| e.to_string())
    }
}