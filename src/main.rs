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

use clap::{arg, Parser};
use rand::Rng;

use crate::sgn::encoder::SgnEncoder;

pub mod asm;
pub mod sgn;
pub mod core;
pub mod xor_dynamic;
pub mod x64_arch;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input binary path
    #[arg(short, long)]
    input: String,

    /// Encoded output binary name
    #[arg(short, long)]
    output: String,

    /// Do not encode the decoder stub
    #[arg(short, long)]
    plain_encoder: bool,
}

fn main() {
    match encode() {
        Ok(_) => println!("Written payload succesfully"),
        Err(error) => println!("{}", error),
    }
}

fn encode() -> Result<(), String> {
    let args = Args::parse();
    let mut buf = vec![];
    let seed: u8 = rand::rng().random();
    let encoder = SgnEncoder::new(seed, args.plain_encoder);

    let mut input_file = File::open(&args.input).map_err(|x| x.to_string())?;
    input_file
        .read_to_end(&mut buf)
        .map_err(|e| e.to_string())?;
    let encoded = encoder.encode(&buf).map_err(|x| x.to_string())?;
    let mut output_file = File::create(&args.output).map_err(|x| x.to_string())?;
    output_file.write_all(&encoded).map_err(|x| x.to_string())?;

    Ok(())
}
