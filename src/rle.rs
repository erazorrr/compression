use crate::encoder::Encoder;

pub struct RLE {
    last: u8,
    cnt: u8,
}

impl RLE {
    fn new() -> Self {
        Self { last: 0, cnt: 0 }
    }
}

impl Encoder for RLE {
    fn encode(&mut self, buffer: &Vec<u8>) -> Vec<u8> {
        buffer.iter().fold(Vec::new(), |mut acc, item| {
            if self.last == *item && self.cnt < 255 {
                self.cnt += 1;
            } else {
                if self.cnt > 0 {
                    acc.push(self.cnt);
                    acc.push(self.last);
                }
                self.last = *item;
                self.cnt = 1;
            }
            acc
        })
    }

    fn flush(&mut self) -> Vec<u8> {
        if self.cnt > 0 {
            vec![self.cnt, self.last]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::encoder::Encoder;
    use crate::rle::RLE;

    #[test]
    fn it_compresses500a() {
        let content = fs::read("./src/samples/500a.txt").unwrap();

        let mut encoder = RLE::new();

        assert_eq!(encoder.encode(&content), vec![255, 'a' as u8]);
        assert_eq!(encoder.flush(), vec![245, 'a' as u8]);
    }

    #[test]
    fn it_compresses100a100b() {
        let content = fs::read("./src/samples/100a100b.txt").unwrap();

        let mut encoder = RLE::new();

        assert_eq!(encoder.encode(&content), vec![100, 'a' as u8]);
        assert_eq!(encoder.flush(), vec![100, 'b' as u8]);
    }

    #[test]
    fn it_compresses100ab() {
        let content = fs::read("./src/samples/100ab.txt").unwrap();

        let mut encoder = RLE::new();

        let mut expected: Vec<u8> = vec![];
        for i in 0..100 {
            expected.push(1);
            expected.push('a' as u8);
            if i != 99 {
                expected.push(1);
                expected.push('b' as u8);
            }
        }
        assert_eq!(encoder.encode(&content), expected);
        assert_eq!(encoder.flush(), vec![1, 'b' as u8]);
    }
}
