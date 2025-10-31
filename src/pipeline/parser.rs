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

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineConfig {
    pub pipeline: PipelineDefinition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub stages: Vec<StageConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StageType {
    Sgn,
    XorDynamic,
    Schema,
}

impl StageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StageType::Sgn => "sgn",
            StageType::XorDynamic => "xor_dynamic",
            StageType::Schema => "schema",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageConfig {
    #[serde(rename = "type")]
    pub stage_type: StageType,
    pub config: StageConfigData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    X64,
    X32,
    AArch64,
}

impl Architecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Architecture::X64 => "x64",
            Architecture::X32 => "x32",
            Architecture::AArch64 => "aarch64",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageConfigData {
    #[serde(default)]
    pub seed: u8,
    pub architecture: Architecture,
    #[serde(default)]
    pub plain_decoder: bool,
    #[serde(default)]
    pub save_registers: bool,
    #[serde(default)]
    pub encoding_count: u32,
    #[serde(default)]
    pub badchars: Vec<u8>,
    #[serde(default)]
    pub schema_size: Option<usize>,
}

impl PipelineConfig {
    /// Parse pipeline configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read pipeline file: {}", e))?;

        Self::from_yaml(&contents)
    }

    /// Parse pipeline configuration from a YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        serde_yaml::from_str(yaml)
            .map_err(|e| format!("Failed to parse YAML: {}", e))
    }

    /// Validate the pipeline configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.pipeline.name.is_empty() {
            return Err("Pipeline name cannot be empty".to_string());
        }

        if self.pipeline.stages.is_empty() {
            return Err("Pipeline must have at least one stage".to_string());
        }

        for (idx, stage) in self.pipeline.stages.iter().enumerate() {
            self.validate_stage(stage, idx)?;
        }

        Ok(())
    }

    fn validate_stage(&self, stage: &StageConfig, idx: usize) -> Result<(), String> {
        // Validate stage type
        match stage.stage_type {
            StageType::Sgn | StageType::XorDynamic | StageType::Schema => {},
        }

        // Validate SGN-specific parameters
        if stage.stage_type == StageType::Sgn {
            if stage.config.encoding_count == 0 {
                return Err(format!(
                    "Stage {}: SGN encoding_count must be greater than 0 (use at least 1)",
                    idx
                ));
            }

            if stage.config.encoding_count > 10 {
                return Err(format!(
                    "Stage {}: SGN encoding_count should not exceed 10 (got {})",
                    idx, stage.config.encoding_count
                ));
            }
        }

        // Validate schema-specific parameters
        if stage.stage_type == StageType::Schema {
            if let Some(size) = stage.config.schema_size {
                if size == 0 {
                    return Err(format!(
                        "Stage {}: Schema size must be greater than 0",
                        idx
                    ));
                }
            }
        }

        // Validate badchars
        if stage.config.badchars.len() > 256 {
            return Err(format!(
                "Stage {}: Too many badchars specified (max 256)",
                idx
            ));
        }

        Ok(())
    }
}
