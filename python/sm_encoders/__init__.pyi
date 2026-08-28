"""Type stubs for sm_encoders"""

from typing import Literal

RngAlgorithm = Literal["chacha", "thread"]

class PySgnEncoderX64:
    """SGN (Shikata Ga Nai) Encoder for x64 architecture"""
    def __init__(
        self,
        seed: int = 0,
        plain_decoder: bool = False,
        encoding_count: int = 1,
        save_registers: bool = False,
        badchars: list[int] = ...,
        ascii_printable: bool = False,
        rng: RngAlgorithm = "thread",
    ) -> None: ...
    def encode(self, payload: bytes) -> bytes: ...

class PyXorDynamicEncoderX64:
    """XOR Dynamic Encoder for x64 architecture"""
    def __init__(
        self,
        seed: int = 0,
        plain_decoder: bool = False,
        encoding_count: int = 1,
        save_registers: bool = False,
        badchars: list[int] = ...,
        ascii_printable: bool = False,
        rng: RngAlgorithm = "thread",
    ) -> None: ...
    def encode(self, payload: bytes) -> bytes: ...


