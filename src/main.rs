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

use crate::sgn::encoder::Encoder;

pub mod asm;
pub mod sgn;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input binary path
    #[arg(short, long)]
    input: String,

    /// Encoded output binary name
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let mut buf = vec![];
    let seed: u8 = rand::rng().random();
    let encoder = Encoder::new(seed);

    let result = File::open(&args.input)
        .map_err(|x| x.to_string())
        .and_then(|mut f| f.read_to_end(&mut buf).map_err(|e| e.to_string()))
        .and_then(|read_bytes| encoder.encode(buf))
        .and_then(|encoded| {
            File::create(&args.output)
                .map_err(|x| x.to_string())
                .map(|x| (x, encoded))
        })
        .and_then(|(mut f, enc)| f.write_all(&enc).map_err(|x| x.to_string()));

    match result {
        Ok(_) => println!("Written payload succesfully"),
        Err(error) => println!("{}", error),
    }
}
