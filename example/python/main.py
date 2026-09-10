from ranis import SgnEncoderX64, XorDynamicEncoderX64

# mov    eax,0x42
# ret
SHELLCODE = b"\xb8\x42\x00\x00\x00\xc3"

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
    encoding_count=2
)
sgn_payload = sgn.encode(SHELLCODE)
show("SGN (chacha, deterministic)", SHELLCODE, sgn_payload)

# XOR Dynamic: lighter-weight encoder, here using the default thread RNG
# (non-deterministic, ignores `seed`).
xor = XorDynamicEncoderX64(rng="thread")
xor_payload = xor.encode(SHELLCODE)
show("XOR Dynamic (thread RNG)", SHELLCODE, xor_payload)
