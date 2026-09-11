# Ranis

Ranis is a collection of polymorphic shellcode encoders for **offensive security operations** and **penetration testing**. Encoders obfuscate a raw payload by wrapping it in a self-decoding stub, so its bytes no longer match static signatures used by antivirus/EDR products, while remaining fully functional at runtime. Multiple stages can be chained together, each one re-encoding the previous stage's output, and every encoder can be constrained to avoid a configurable set of bad characters (e.g. `0x00`, `0x0a`, `0x0d`) that would otherwise break delivery through the target's input path.

Encoders are implemented in Rust and exposed as a CLI, a [Python extension module](python/ranis), and a [C library](include/ranis.h) (see [example/python](example/python) and [example/c_lang](example/c_lang)).

* **Shikata Ga Nai (SGN)** — a polymorphic XOR additive-feedback encoder: each decoded byte feeds into the key for the next, and the decoder stub itself is regenerated with randomized instructions and register allocation on every run, so no two encodings look alike even with the same input.
* **XOR Dynamic** — a lighter-weight encoder that XORs the payload against a runtime-generated key, trading SGN's polymorphism for a smaller, faster decoder stub.
* Both encoders support multiple encoding rounds (`encoding_count`), an optional register save/restore prologue (`save_registers`), skipping encoding of the decoder stub itself (`plain_decoder`), a deterministic or OS-sourced RNG for the assembler internals, and a `badchars` list the encoded output is guaranteed to avoid.
* Stages can be chained into a pipeline (see below), where each encoder's output becomes the next one's input.

---

### 🚧 Development Roadmap

* [x] Shikata Ga Nai (x64)
* [ ] Shikata Ga Nai (x32)
* [ ] Shikata Ga Nai (AArch64)
* [x] XOR Dynamic (x64)
* [ ] XOR Dynamic (x32)
* [ ] XOR Dynamic (AArch64)
* [ ] XOR Static
* [x] Encoding pipeline (chain multiple stages via YAML config)

---

### 🔧 Usage

Try `-h` for more information:

```bash
Usage: ranis [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>                Input binary path
  -o, --output <OUTPUT>               Encoded output binary name
  -e, --encoder-type <ENCODER_TYPE>   Encoder type (ignored if --pipeline is specified)
                                       [possible values: shikata-ga-nai, xor-dynamic]
  -p, --plain-decoder                 Do not encode the decoder stub (ignored if --pipeline is specified)
      --encoding-count <ENCODING_COUNT>
                                       Number of encoding iterations (ignored if --pipeline is specified) [default: 1]
      --save-registers                Save and restore registers in decoder stub (ignored if --pipeline is specified)
      --badchars <BADCHARS>...        Bad characters as hex bytes, for example: 0x00 0x0a 0x0d
      --pipeline <PIPELINE>           Path to pipeline YAML configuration file
      --rng <RNG>                     RNG algorithm used by assembler internals (ignored if --pipeline is specified)
                                       [default: thread] [possible values: thread, cha-cha]
      --seed <SEED>                   Seed for the assembler RNG; random if omitted (ignored for --rng thread)
  -h, --help                          Print help
  -V, --version                       Print version
```

---

### 🦀 Rust usage

Ranis can also be used directly as a Rust crate:

```rust
use ranis::core::encoder::Encoder;
use ranis::sgn::encoder::SgnEncoderX64ChaCha;

fn main() {
    // mov eax, 0x42; ret
    let payload = [0xb8, 0x42, 0x00, 0x00, 0x00, 0xc3];

    let mut encoder = SgnEncoderX64ChaCha::builder()
        .set_encoding_count(2)
        .set_badchars([0x00, 0x0a, 0x0d].into_iter().collect())
        .build_with_rng_seed(1234);

    let encoded = encoder.encode(&payload).expect("encoding failed");
    println!("{}", encoded.iter().map(|b| format!("{:02x}", b)).collect::<String>());
}
```

Swap in `SgnEncoderX64ThreadRng` (non-deterministic OS RNG, no seed) or the `xor_dynamic::encoder` equivalents for the other encoder/RNG combinations.

---

### 🧵 Encoding pipeline

Instead of a single encoder, `--pipeline <FILE>` chains multiple encoding stages defined in a YAML file (see [pipeline.yaml](pipeline.yaml)):

```yaml
pipeline:
  name: "multi-layer-obfuscation"
  description: "Apply multiple encoding stages"
  stages:
    - type: "sgn"
      config:
        seed: 97
        rng: "chacha"       # "chacha" (seeded, deterministic) or "thread" (ignores seed, default)
        plain_decoder: false
        architecture: "x64"

    - type: "xor_dynamic"
      config:
        seed: 0x7F
        architecture: "x64"
        rng: "chacha"
        plain_decoder: false
        badchars: [0x00, 0x0a, 0x0d]
```

Each stage under `stages` supports:

| Field             | Default   | Notes                                                        |
|-------------------|-----------|---------------------------------------------------------------|
| `type`            | —         | `sgn` or `xor_dynamic`                                        |
| `architecture`    | —         | Currently only `x64` is wired end-to-end                      |
| `seed`            | `0`       | Ignored when `rng` is `thread`                                 |
| `rng`             | `thread`  | `chacha` (seeded, deterministic) or `thread` (OS RNG)          |
| `plain_decoder`   | `false`   | Skip encoding the decoder stub itself                          |
| `save_registers`  | `false`   | Emit register save/restore prologue/epilogue                   |
| `encoding_count`  | `1`       | Number of encoding rounds (SGN: 1–10)                          |
| `badchars`        | `[]`      | Bytes the encoded output must avoid (max 256)                  |

Stages run in order, each one's output feeding into the next.

---

### 🙏 Acknowledgments

* [egebalci/sgn](https://github.com/egebalci/sgn) — the original Shikata Ga Nai polymorphic XOR additive feedback encoder, which this project's SGN encoder is inspired by.
* [CensoredUsername/dynasm-rs](https://github.com/CensoredUsername/dynasm-rs) — the runtime assembler used to assemble the x64/AArch64 encoder and decoder stubs.
* [rapid7/metasploit-framework](https://github.com/rapid7/metasploit-framework) — whose XOR Dynamic encoder implementation served as a reference.

---

### 📄 License

Licensed under the [Apache License, Version 2.0](LICENSE).


