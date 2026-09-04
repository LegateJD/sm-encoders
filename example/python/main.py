from sm_encoders import SgnEncoderX64, XorDynamicEncoderX64

enc = SgnEncoderX64(seed=1234, rng="chacha")

print("Encoding:")

# mov    eax,0x42
# ret
payload = enc.encode(b"\xb8\x42\x00\x00\x00\xc3")

print(f"Payload\n: {payload.hex()}")
