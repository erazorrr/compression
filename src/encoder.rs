pub trait Encoder {
    fn encode(&mut self, buffer: &Vec<u8>) -> Vec<u8>;
    fn flush(&mut self) -> Vec<u8>;
}