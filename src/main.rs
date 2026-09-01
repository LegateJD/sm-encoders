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
    io::{Read, Write},
};

use clap::{Parser, ValueEnum};
use rand::RngExt;

use crate::pipeline::encode::Pipeline;
use crate::{
    core::encoder::Encoder,
    sgn::encoder::{SgnEncoderX64ChaCha, SgnEncoderX64ThreadRng},
    xor_dynamic::encoder::XorDynamicEncoderX64ChaCha,
};

pub mod arm64;
pub mod core;
pub mod obfuscation;
pub mod pipeline;
pub mod schema;
pub mod sgn;
pub mod utils;
pub mod x64_arch;
pub mod xor_dynamic;

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

    /// Bad characters as hex bytes, for example: 0x00 0x0a 0x0d
    #[arg(long, value_parser = parse_hex_byte, num_args = 0..)]
    badchars: Vec<u8>,

    /// Require printable ASCII output (ignored if --pipeline is specified)
    //#[arg(long, default_value_t = false)]
    //ascii_printable: bool,

    /// Path to pipeline YAML configuration file
    #[arg(long, conflicts_with = "encoder_type")]
    pipeline: Option<String>,

    /// RNG algorithm used by assembler internals (ignored if --pipeline is specified)
    #[arg(long, value_enum, default_value_t = RngAlgorithm::ChaCha)]
    rng: RngAlgorithm,

    /// Seed for the assembler RNG; random if omitted (ignored for --rng thread)
    #[arg(long)]
    seed: Option<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum EncoderType {
    ShikataGaNai,
    XorDynamic,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum RngAlgorithm {
    ChaCha,
    #[default]
    Thread
}

fn main() {
    match encode() {
        Ok(_) => println!("Written payload succesfully"),
        Err(error) => println!("Main error: {}", error),
    }
}

fn encode() -> Result<(), String> {
    //let args = Args::parse();
    let args = Args {
        input: "input.bin".to_owned(),
        output: "output.bin".to_owned(),
        encoder_type: Some(EncoderType::XorDynamic),
        plain_decoder: false,
        encoding_count: 1,
        save_registers: false,
        badchars: vec![],
        //ascii_printable: false,
        pipeline: Some("pipeline.yaml".to_owned()),
        rng: RngAlgorithm::ChaCha,
        seed: Some(4234),
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
        let encoder_type = args
            .encoder_type
            .ok_or("Either --encoder-type or --pipeline must be specified")?;

        let seed: u64 = args.seed.unwrap_or_else(|| rand::rng().random());
        println!(
            "Using single encoder mode with asm_seed: 0x{:016X}",
            seed
        );

        match (encoder_type, args.rng) {
            (EncoderType::ShikataGaNai, RngAlgorithm::ChaCha) => {
                let mut encoder = SgnEncoderX64ChaCha::builder()
                    .set_plain_decoder(args.plain_decoder)
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .set_badchars(args.badchars.iter().copied().collect())
                    //.set_ascii_printable(args.ascii_printable)
                    .build_with_rng_seed(seed);
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::ShikataGaNai, RngAlgorithm::Thread) => {
                let mut encoder = SgnEncoderX64ThreadRng::builder()
                    .set_plain_decoder(args.plain_decoder)
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .set_badchars(args.badchars.iter().copied().collect())
                    //.set_ascii_printable(args.ascii_printable)
                    .build();
                encoder.encode(&buf).map_err(|x| x.to_string())?
            }
            (EncoderType::XorDynamic, _) => {
                let mut encoder = XorDynamicEncoderX64ChaCha::builder()
                    .set_plain_decoder(args.plain_decoder)
                    .set_encoding_count(args.encoding_count)
                    .set_save_registers(args.save_registers)
                    .set_badchars(args.badchars.iter().copied().collect())
                    //.set_ascii_printable(args.ascii_printable)
                    .build_with_rng_seed(seed);
                encoder
                    .encode(&buf)
                    .map_err(|x: xor_dynamic::encoder::XorDynamicEncoderError| x.to_string())?
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

fn parse_hex_byte(value: &str) -> Result<u8, String> {
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);

    u8::from_str_radix(value, 16).map_err(|_| format!("Invalid hex byte: {}", value))
}
