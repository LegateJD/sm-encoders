from ranis import SgnEncoderX64, XorDynamicEncoderX64

# mov    eax,0x42
# ret
SHELLCODE = b"\xb8\x42\x00\x00\x00\xc3"

# Bytes the encoded output must never contain (common bad chars in exploits).
BADCHARS = [0x00, 0x0a, 0x0d]


def show(name: str, original: bytes, encoded: bytes) -> None:
    print(f"--- {name} ---")
    print(f"Input  ({len(original)} bytes): {original.hex()}")
    print(f"Output ({len(encoded)} bytes): {encoded.hex()}")
    print()


# Shikata Ga Nai: seeded ChaCha RNG gives fully deterministic output, so
# re-running with the same seed always produces the same encoded payload.
sgn = SgnEncoderX64(
    seed=1234,
    rng="chacha",
    encoding_count=2,
    save_registers=True
)
sgn_payload = sgn.encode(SHELLCODE)
show("SGN (chacha, deterministic)", SHELLCODE, sgn_payload)
assert sgn.encode(SHELLCODE) == sgn_payload, "same seed should reproduce output"

# XOR Dynamic: lighter-weight encoder, here using the default thread RNG
# (non-deterministic, ignores `seed`).
xor = XorDynamicEncoderX64(rng="thread", badchars=BADCHARS)
xor_payload = xor.encode(SHELLCODE)
show("XOR Dynamic (thread RNG)", SHELLCODE, xor_payload)
