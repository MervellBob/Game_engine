use png::Decoder;
use png::Reader;
use std::fs::File;
use std::io::BufReader;

use crate::asset::Asset;

pub struct Png {
    reader: Reader<BufReader<File>>,
    pub contents_raw: Vec<u8>,
}

impl Asset for Png {
    fn new(file: File) -> Self {
        let mut decoder: Decoder<BufReader<File>> = Decoder::new(BufReader::new(file));
        decoder.set_ignore_text_chunk(true);

        let reader: Reader<BufReader<File>> = decoder.read_info().unwrap();
        let contents_raw: Vec<u8> = vec![0; reader.output_buffer_size().unwrap()];

        Self {
            reader: reader,
            contents_raw: contents_raw,
        }
    }

    fn read_raw(&mut self) {
        self.reader.next_frame(&mut self.contents_raw).unwrap();
    }
}

#[cfg(test)]
mod tests_png {
    use crate::asset::Asset;
    use crate::png::Png;
    use std::fs::File;

    #[test]
    fn png_file_1() {
        let path_file_1: String = std::env::var("PNG_FILE_1").unwrap_or_default();
        // This magic value comes from `convert icon_Flav20.png rgba:- | wc -c`
        let res: usize = 1313316;

        let file: File = File::open(&path_file_1).expect("Unable to open file");

        let mut png: Png = Png::new(file);
        png.read_raw();

        assert_eq!(
            png.contents_raw.len(),
            res,
            "Expected:{}, got: {}",
            png.contents_raw.len(),
            res
        );
    }
}
