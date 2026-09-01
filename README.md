# SM Encoders

This repository provides a collection of custom encoders designed for **offensive security operations** and **penetration testing**.

---

### 🚧 Development Roadmap

* [x] Shikata Ga Nai (x64)
* [ ] Shikata Ga Nai (x32)
* [x] XOR Dynamic (x64)
* [ ] XOR Dynamic (x32 / AArch64)
* [ ] XOR Static
* [x] Encoding pipeline (chain multiple stages via YAML config)

---

### 🔧 Usage

Try `-h` for more information:

```bash
Usage: sm-encoders [OPTIONS] --input <INPUT> --output <OUTPUT>

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

