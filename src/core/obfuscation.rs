pub trait GarbageJump {
    fn get_jmp_over(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error>;

    fn generate_garbage_jump(&self) -> Result<Vec<u8>, anyhow::Error>;
}

pub trait CallOver {
    fn add_call_over(payload: Vec<u8>) -> Result<Vec<u8>, anyhow::Error>;
}

pub trait Encode {
    fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error>;
}