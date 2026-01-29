use std::fs::File;
use std::io::Read;

use crate::asset::Asset;

//
// Text asset
//
pub struct Text {
    file: File,
    pub contents_raw: Vec<u8>,
}

impl Asset for Text {
    fn new(file: File) -> Self {
        Self {
            file: file,
            contents_raw: Vec::new(),
        }
    }

    fn read_raw(&mut self) {
        let _ = self.file.read_to_end(&mut self.contents_raw);
    }
}

#[cfg(test)]
mod tests_text {

    use std::fs::File;
    use crate::text::Text;
    use crate::asset::Asset;

    #[test]
    fn text_file_1() {
        let path_file_1: String = std::env::var("TEXT_FILE_1").unwrap_or_default();
        let res_content: &str = "L’orchestre militaire, au milieu du jardin, Balance ses schakos dans la Valse des fifres : Autour, aux premiers rangs, parade le gandin ; Le notaire pend à ses breloques à chiffres.
";
        let file: File = File::open(&path_file_1).expect("Unable to open file");
        let mut text: Text = Text::new(file);

        text.read_raw();

        let content: String = String::from_utf8(text.contents_raw).expect("Found invalid UTF-8");

        assert_eq!(content,
                   res_content,
                   "Expected:^{}$\nGot:^{}$", res_content, content
        );
    }
}
