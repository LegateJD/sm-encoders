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

use std::{
    fs::File,
    io::{Read, Write}
};

use clap::{arg, Parser, ValueEnum};
use rand::Rng;

use crate::{core::encoder::Encoder, sgn::encoder::SgnEncoderX64, xor_dynamic::encoder::XorDynamicEncoderX64};
use crate::pipeline::encode::Pipeline;
use crate::schema::encoder::SchemaEncoderX64;

pub mod sgn;
pub mod core;
pub mod xor_dynamic;
pub mod x64_arch;
pub mod schema;
pub mod arm64;
pub mod obfuscation;
pub mod utils;
pub mod pipeline;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input binary path
    #[arg(short, long)]
    input: String,

    /// Encoded output binary name
    #[arg(short, long)]
    output: String,

    /// Encoder type (ignored if --pipeline is specified)
    #[arg(short, long, value_enum)]
    encoder_type: Option<EncoderType>,

    /// Do not encode the decoder stub (ignored if --pipeline is specified)
    #[arg(short, long, default_value_t = false)]
    plain_decoder: bool,

    /// Path to pipeline YAML configuration file
    #[arg(long, conflicts_with = "encoder_type")]
    pipeline: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum EncoderType {
    Sgn,
    Schema,
    XorDynamic,
}

fn main() {
    match encode() {
        Ok(_) => println!("Written payload succesfully"),
        Err(error) => println!("{}", error),
    }
}

fn encode() -> Result<(), String> {
    //let args = Args::parse();
    let args = Args {
        input: String::from("input.bin"),
        output: String::from("output.bin"),
        encoder_type: None,
        plain_decoder: false,
        pipeline: Some(String::from("pipeline.yaml")),
    };

    let mut buf = vec![];
    let mut input_file = File::open(&args.input).map_err(|x| x.to_string())?;
    input_file
        .read_to_end(&mut buf)
        .map_err(|e| e.to_string())?;

    let encoded = if let Some(pipeline_path) = args.pipeline {
        // Use pipeline mode
        println!("Using pipeline configuration from: {}", pipeline_path);
        let pipeline = Pipeline::from_file(&pipeline_path)?;
        pipeline.run(&buf)?
    } else {
        // Use single encoder mode
        let encoder_type = args.encoder_type
            .ok_or("Either --encoder-type or --pipeline must be specified")?;

        let seed: u8 = rand::rng().random();
        println!("Using single encoder mode with seed: 0x{:02X}", seed);

        match encoder_type {
            EncoderType::Sgn => {
                let encoder = SgnEncoderX64::new(seed, args.plain_decoder);
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            EncoderType::XorDynamic => {
                let encoder = XorDynamicEncoderX64::new(seed);
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            EncoderType::Schema => {
                let encoder = SchemaEncoderX64::new(seed);
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
        }
    };

    println!("Encoded payload ({} bytes):", encoded.len());
    for byte in &encoded {
        print!("0x{:02x}, ", byte);
    }

    println!();

    let mut output_file = File::create(&args.output).map_err(|x| x.to_string())?;
    output_file.write_all(&encoded).map_err(|x| x.to_string())?;

    Ok(())
}
