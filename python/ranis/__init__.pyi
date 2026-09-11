"""Type stubs for ranis"""

class SgnEncoderX64ThreadRng:
    """SGN (Shikata Ga Nai) Encoder for x64 architecture, using the OS thread RNG"""
    def __init__(
        self,
        plain_decoder: bool = False,
        encoding_count: int = 1,
        save_registers: bool = False,
        badchars: list[int] = ...,
    ) -> None: ...
    def encode(self, payload: bytes) -> bytes: ...

class SgnEncoderX64ChaCha:
    """SGN (Shikata Ga Nai) Encoder for x64 architecture, using a seeded ChaCha RNG"""
    def __init__(
        self,
        seed: int = 0,
        plain_decoder: bool = False,
        encoding_count: int = 1,
        save_registers: bool = False,
        badchars: list[int] = ...,
    ) -> None: ...
    def encode(self, payload: bytes) -> bytes: ...

class XorDynamicEncoderX64ThreadRng:
    """XOR Dynamic Encoder for x64 architecture, using the OS thread RNG"""
    def __init__(
        self,
        plain_decoder: bool = False,
        encoding_count: int = 1,
        save_registers: bool = False,
        badchars: list[int] = ...,
    ) -> None: ...
    def encode(self, payload: bytes) -> bytes: ...

class XorDynamicEncoderX64ChaCha:
    """XOR Dynamic Encoder for x64 architecture, using a seeded ChaCha RNG"""
    def __init__(
        self,
        seed: int = 0,
        plain_decoder: bool = False,
        encoding_count: int = 1,
        save_registers: bool = False,
        badchars: list[int] = ...,
    ) -> None: ...
    def encode(self, payload: bytes) -> bytes: ...
