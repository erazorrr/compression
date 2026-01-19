use std::collections::HashMap;
use crate::encoder::Encoder;

pub struct Huffman {
    map: HashMap<u8, (u8, u8)>,

    byte: u8,
    offset: u8
}

impl Huffman {
    fn new(frequencies: [u32; 256]) -> Self {
        let mut sorted: [(u8, u32); 256] = [(0, 0); 256];
        for i in 0..=255 {
            sorted[i] = (i as u8, frequencies[i]);
        }
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let mut map: HashMap<u8, (u8, u8)> = HashMap::new();
        let mut curr: u8 = 0b0000_0000;
        let mut curr_length: u8 = 1;
        for i in 0..sorted.len() {
            map.insert(sorted[i].0, (curr, curr_length));
            if i != sorted.len() - 2 {
                curr = curr.rotate_left(1) | 0b0000_0010;
                curr_length = curr_length.wrapping_add(1);
            } else {
                curr = curr | 0b0000_0001;
            }
        }

        Self { map, byte: 0, offset: 0 }
    }

    fn build_frequencies(buffer: &Vec<u8>) -> [u32; 256] {
        let mut counts = [0u32; 256];

        for b in buffer {
            counts[*b as usize] += 1;
        }

        counts
    }
}

impl Encoder for Huffman {
    fn encode(&mut self, buffer: &Vec<u8>) -> Vec<u8> {
        buffer.iter().fold(Vec::new(), |mut acc, item| {
            let (val, bits) = self.map.get(item).unwrap();

            self.byte = self.byte | ((*val << (8 - *bits)) >> self.offset);
            self.offset += *bits;
            if self.offset >= 8 {
                acc.push(self.byte);
                self.offset -= 8;
                self.byte = 0b0000_0000 | ((*val).unbounded_shl((8 - self.offset) as u32));
            }

            acc
        })
    }

    fn flush(&mut self) -> Vec<u8> {
        if self.offset > 0 {
            vec![self.byte]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::encoder::Encoder;
    use crate::huffman::Huffman;

    #[test]
    fn it_compresses500a() {
        let content = fs::read("./src/samples/500a.txt").unwrap();

        let mut encoder = Huffman::new(Huffman::build_frequencies(&content));

        assert_eq!(encoder.encode(&content), vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]);
        assert_eq!(encoder.flush(), vec![0]);
    }

    #[test]
    fn it_compresses100a100b() {
        let content = fs::read("./src/samples/100a100b.txt").unwrap();

        let mut encoder = Huffman::new(Huffman::build_frequencies(&content));

        assert_eq!(encoder.encode(&content), vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 10, 170, 170, 170, 170, 170, 170, 170,
            170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
            170, 170, 170, 170, 170, 170, 170,
        ]);
        assert_eq!(encoder.flush(), vec![160]);
    }

    #[test]
    fn it_compresses100ab() {
        let content = fs::read("./src/samples/100ab.txt").unwrap();

        let mut encoder = Huffman::new(Huffman::build_frequencies(&content));

        // a -> 0
        // b -> 10

        // 0b0_10_0_10_0_1 -> 73
        // 0b0_0_10_0_10_0 -> 36
        // 0b10_0_10_0_10 -> 146
        assert_eq!(encoder.encode(&content), vec![
            73, 36, 146, 73, 36, 146, 73, 36, 146, 73,
            36, 146, 73, 36, 146, 73, 36, 146, 73, 36,
            146, 73, 36, 146, 73, 36, 146, 73, 36, 146,
            73, 36, 146, 73, 36, 146, 73,
        ]);
        assert_eq!(encoder.flush(), vec![32]);
    }
}
