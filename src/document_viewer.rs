use crate::document;
use document::Document;
use std::io::{stdin, stdout, BufRead, Read, Write};

enum CursorType {
    BlinkingUnderLine,
    BlinkingBlock,
}

enum ViewMode {
    Normal,
    Insert,
}

pub struct Viewer<'a> {
    doc: Document<'a>,
    cursor: CursorType,
    mode: ViewMode,
}

impl<'a> Viewer<'a> {
    pub fn new(doc: Document<'a>) -> Self {
        Self {
            doc,
            cursor: CursorType::BlinkingBlock,
            mode: ViewMode::Normal,
        }
    }
    // displays the document
    pub fn view(&self) {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        let stdin = stdin();
        let mut stdin = stdin.lock();
        let content = self.doc.get_contents();
        let file_name = self.doc.get_file_name();
        let mode = match self.mode {
            ViewMode::Normal => "Normal",
            ViewMode::Insert => "Insert",
        };
        let (_screen_width, screen_height) = termion::terminal_size().unwrap();
        println!("{}{}", screen_height, mode);
        stdout.flush().unwrap();
        write!(
            stdout,
            "{}{}{}{}{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::color::Bg(termion::color::Green),
            file_name,
            termion::color::Bg(termion::color::Black),
            termion::cursor::Goto(1, 3),
            content,
            termion::cursor::Goto(1, 20),
            mode
        )
        .unwrap();
        let mut input = String::new();
        stdin.read_to_string(&mut input).unwrap();
    }

    // exit the viewer
    pub fn quit(&self) {}
    fn change_view_mode(&mut self, mode: ViewMode) {
        self.mode = mode;
    }
    #[allow(unused)]
    fn poll_event(&self) {}
}
