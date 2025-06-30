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

use std::collections::HashSet;

use anyhow::anyhow;
use thiserror::Error;

struct XorDynamicEncoder {
    pub name: String,
    pub stub: Vec<u8>,
    pub stub_key_term: Vec<u8>,
    pub stub_payload_term: Vec<u8>,
    pub badchars: HashSet<u8>
}

#[derive(Error, Debug)]
pub enum EncoderError {
    #[error("data store disconnected")]
    BadCharacters
}

pub trait XorDynamicStub {
    fn get_decoder_stub(&self, payload_size: usize) -> Result<Vec<u8>, anyhow::Error>;
}

pub fn find_key(
    buf: &[u8],
    badchars: &HashSet<u8>,
    key_chars: &[u8],
) -> Result<Vec<u8>, anyhow::Error> {
    let buf_len = buf.len();

    let mut min_len = {
        let mut val = (buf_len as f64 / 100.0 * (0.2 + 0.05 * badchars.len() as f64)) as usize;
        if val < 1 {
            val = 1;
        }
        val
    };

    let mut max_len = buf_len;
    if min_len > max_len || min_len == usize::MAX {
        min_len = max_len;
    }

    let mut key_increment = {
        let mut val = (buf_len as f64 / 100.0 * (0.01 + 0.001 * badchars.len() as f64)) as usize;
        if val < 1 {
            val = 1;
        }
        val
    };

    let mut key_len = min_len;
    while key_len <= max_len + key_increment {
        if key_len > max_len {
            key_len = max_len;
        }

        let mut my_key: Vec<u8> = Vec::new();

        for x in 0..key_len {
            let mut found_char = None;

            for &j in key_chars {
                let mut ok = true;

                let mut i = 0;
                while i + x < buf_len {
                    let b = buf[i + x] ^ j;
                    if badchars.contains(&b) {
                        ok = false;
                        break;
                    }
                    i += key_len;
                }

                if ok {
                    found_char = Some(j);
                    break;
                }
            }

            if let Some(c) = found_char {
                my_key.push(c);
            } else {
                break;
            }
        }

        if my_key.len() == key_len {
            return Ok(my_key);
        }

        key_len += key_increment;
    }

    todo!()
}

impl XorDynamicEncoder {
    pub fn encode(&self, buf: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        let mut badchars = self.badchars.clone();

        let stub_without_terms = self
            .stub
            .windows(self.stub_key_term.len())
            .filter(|w| *w != self.stub_key_term.as_slice())
            .collect::<Vec<_>>()
            .concat();

        let stub_cleaned = stub_without_terms
            .windows(self.stub_payload_term.len())
            .filter(|w| *w != self.stub_payload_term.as_slice())
            .collect::<Vec<_>>()
            .concat();

        if has_badchars(&stub_cleaned, &badchars) {
            return Err(EncoderError::BadCharacters.into());
        }

        let key_chars: Vec<u8> = (1u8..=255).filter(|c| !badchars.contains(c)).collect();

        let key = find_key(buf, &badchars, &key_chars)?;

        let mut key_term = None;
        let mut rng = rand::thread_rng();
        use rand::seq::SliceRandom;

        let mut shuffled = key_chars.clone();
        shuffled.shuffle(&mut rng);

        for &i in &shuffled {
            if !key.contains(&i) {
                key_term = Some(i);
                break;
            }
        }

        let key_term = key_term.ok_or_else(|| {
            anyhow!(
                "Key terminator could not be found for the {} encoder.",
                self.name
            )
        })?;

        let mut encoded: Vec<u8> = Vec::new();
        for (pos, &b) in buf.iter().enumerate() {
            encoded.push(b ^ key[pos % key.len()]);
        }

        let mut payload_term = None;

        let mut outer = shuffled.clone();
        outer.shuffle(&mut rng);

        'outer: for &i in &outer {
            let mut inner = shuffled.clone();
            inner.shuffle(&mut rng);

            for &j in &inner {
                let pair = vec![i, j];
                if !find_subsequence(&encoded, &pair) {
                    payload_term = Some(pair);
                    break 'outer;
                }
            }
        }

        let payload_term = payload_term.ok_or_else(|| {
            anyhow!(
                "Payload terminator could not be found for the {} encoder.",
                self.name
            )
        })?;

        let mut final_payload = Vec::new();

        let mut stub_replaced = self.stub.clone();
        stub_replaced = replace_subsequence(&stub_replaced, &self.stub_key_term, &[key_term]);
        stub_replaced = replace_subsequence(&stub_replaced, &self.stub_payload_term, &payload_term);

        final_payload.extend_from_slice(&stub_replaced);
        final_payload.extend_from_slice(&key);
        final_payload.push(key_term);
        final_payload.extend_from_slice(&encoded);
        final_payload.extend_from_slice(&payload_term);

        if has_badchars(&final_payload, &badchars) {
            return Err(EncoderError::BadCharacters.into());
        }

        Ok(final_payload)
    }
}

fn has_badchars(buf: &[u8], badchars: &HashSet<u8>) -> bool {
    buf.iter().any(|b| badchars.contains(b))
}

fn replace_subsequence(haystack: &[u8], needle: &[u8], replacement: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i <= haystack.len() - needle.len() {
        if &haystack[i..i + needle.len()] == needle {
            result.extend_from_slice(replacement);
            i += needle.len();
        } else {
            result.push(haystack[i]);
            i += 1;
        }
    }

    result.extend_from_slice(&haystack[i..]);
    result
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
