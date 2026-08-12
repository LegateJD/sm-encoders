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
use rand::{Rng, RngExt, SeedableRng};
use rand::rngs::{ChaCha12Rng, ChaCha20Rng};

use crate::{core::encoder::Encoder, sgn::encoder::{SgnEncoderX64ChaCha12Rng, SgnEncoderX64ChaChaRng, SgnEncoderX64ThreadRng}, xor_dynamic::encoder::XorDynamicEncoderX64ChaCha};
use crate::pipeline::encode::Pipeline;
use crate::schema::encoder::{SchemaEncoderX64, SchemaEncoderX64ChaCha, SchemaEncoderX64Thread};

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

    /// Number of encoding iterations (ignored if --pipeline is specified)
    #[arg(long, default_value_t = 1)]
    encoding_count: u32,

    /// Save and restore registers in decoder stub (ignored if --pipeline is specified)
    #[arg(long, default_value_t = false)]
    save_registers: bool,

    /// Path to pipeline YAML configuration file
    #[arg(long, conflicts_with = "encoder_type")]
    pipeline: Option<String>,

    /// RNG algorithm used by assembler internals (ignored if --pipeline is specified)
    #[arg(long, value_enum, default_value_t = RngAlgorithm::ChaCha12)]
    rng: RngAlgorithm,

    /// Seed for the assembler RNG; random if omitted (ignored for --rng thread)
    #[arg(long)]
    asm_seed: Option<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum EncoderType {
    Sgn,
    Schema,
    XorDynamic,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
enum RngAlgorithm {
    #[default]
    ChaCha12,
    ChaCha20,
    Thread,
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
        input: "input.bin".to_owned(),
        output: "output.bin".to_owned(),
        encoder_type: Some(EncoderType::XorDynamic),
        plain_decoder: false,
        encoding_count: 6,
        save_registers: false,
        pipeline: None,
        rng: RngAlgorithm::ChaCha12,
        asm_seed: None,
    };

    let mut buf = vec![];
    let mut input_file = File::open(&args.input).map_err(|x| x.to_string())?;
    input_file
        .read_to_end(&mut buf)
        .map_err(|e| e.to_string())?;

    let encoded = if let Some(pipeline_path) = args.pipeline {
        // Use pipeline mode
        println!("Using pipeline configuration from: {}", pipeline_path);
        let mut pipeline = Pipeline::from_file(&pipeline_path)?;
        pipeline.run(&buf)?
    } else {
        // Use single encoder mode
        let encoder_type = args.encoder_type
            .ok_or("Either --encoder-type or --pipeline must be specified")?;

        let seed: u8 = rand::rng().random();
        let asm_seed: u64 = args.asm_seed.unwrap_or_else(|| rand::rng().random());
        println!("Using single encoder mode with seed: 0x{:02X}, asm_seed: 0x{:016X}", seed, asm_seed);

        match (encoder_type, args.rng) {
            (EncoderType::Sgn, RngAlgorithm::ChaCha12) => {
                let mut encoder = SgnEncoderX64ChaCha12Rng::builder()
                    .set_seed(seed)
                    .set_plain_decoder(args.plain_decoder)
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .build_with_rng(ChaCha12Rng::seed_from_u64(asm_seed));
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::Sgn, RngAlgorithm::ChaCha20) => {
                let mut encoder = SgnEncoderX64ChaChaRng::builder()
                    .set_seed(seed)
                    .set_plain_decoder(args.plain_decoder)
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .build_with_rng(ChaCha20Rng::seed_from_u64(asm_seed));
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::Sgn, RngAlgorithm::Thread) => {
                let mut encoder = SgnEncoderX64ThreadRng::builder()
                    .set_seed(seed)
                    .set_plain_decoder(args.plain_decoder)
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .build();
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::XorDynamic, _) => {
                let mut encoder = XorDynamicEncoderX64ChaCha::builder()
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .build_with_rng(ChaCha20Rng::seed_from_u64(asm_seed));
                encoder.encode(&buf).map_err(|x: xor_dynamic::encoder::XorDynamicEncoderError| x.to_string())?
            }
            (EncoderType::Schema, RngAlgorithm::ChaCha12) => {
                let mut encoder = SchemaEncoderX64::new_with_rng(ChaCha12Rng::seed_from_u64(asm_seed));
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::Schema, RngAlgorithm::ChaCha20) => {
                let mut encoder = SchemaEncoderX64ChaCha::new_with_rng(ChaCha20Rng::seed_from_u64(asm_seed));
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::Schema, RngAlgorithm::Thread) => {
                let mut encoder = SchemaEncoderX64Thread::new(0);
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
