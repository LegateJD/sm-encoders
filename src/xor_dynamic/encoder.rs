struct XorDynamicEncoder {}

impl XorDynamicEncoder {
    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        todo!()
    }
}


fn generate_decoder_assembly(payload_size: usize) -> String {
    let decoder_template: String = "JMP _call
POP RBX
PUSH RBX
POP RDI
MOV AL, 'A'
CLD
SCAS AL, BYTE PTR ES:[RDI]
JNE _lp1
PUSH RDI
POP RCX
PUSH RBX
POP RSI
MOV AL, BYTE PTR [RSI]
XOR BYTE PTR [DIR], AL
INC RDI
INC RSI
CMP WORD PTR [RDI], 'BB'
JE  _jmp
CMP BYTE PTR [RSI], 'A'
JNE _lp3
JMP _lp2
JMP  RCX
CALL _ret"
        .into();

    todo!()
}
