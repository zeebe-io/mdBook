use book::metadata::BookMetadata;

use std::path;

pub struct Book {
    metadata: BookMetadata,

    preface: Vec<Chapter>,
    chapters: Vec<Chapter>,
    appendix: Vec<Chapter>,
}

pub struct Chapter {
    title: String,
    file: path::PathBuf,

    sub_chapters: Vec<Chapter>,
}


impl Book {
    pub fn new(title: &str) -> Self {
        Book {
            metadata: BookMetadata::new(title),

            preface: Vec::new(),
            chapters: Vec::new(),
            appendix: Vec::new(),
        }
    }

    pub fn add_chapter(&mut self, chapter: Chapter) -> &mut Self {
        self.chapters.push(chapter);
        self
    }

    pub fn add_preface_chapter(&mut self, chapter: Chapter) -> &mut Self {
        self.preface.push(chapter);
        self
    }

    pub fn add_appendix_chapter(&mut self, chapter: Chapter) -> &mut Self {
        self.appendix.push(chapter);
        self
    }

    pub fn get_chapter(&self, section: &[usize]) -> Option<&Chapter> {
        match section.len() {
            0 => None,
            1 => self.chapters.get(section[0]),
            _ => {
                self.chapters.get(section[0])
                             .and_then(|ch| ch.get_sub_chapter(&section[1..]))
            }
        }
    }

    pub fn mut_metadata(&mut self) -> &mut BookMetadata { &mut self.metadata }
    pub fn metadata(&self) -> &BookMetadata { &self.metadata }
}


impl Chapter {
    pub fn new(title: &str, file: &path::Path) -> Self {
        Chapter {
            title: title.to_owned(),
            file: file.to_owned(),

            sub_chapters: Vec::new(),
        }
    }

    pub fn get_sub_chapter(&self, section: &[usize]) -> Option<&Chapter> {
        match section.len() {
            0 => None,
            1 => self.sub_chapters.get(section[0]),
            _ => {
                self.sub_chapters.get(section[0])
                             .and_then(|ch| ch.get_sub_chapter(&section[1..]))
            }
        }
    }

    pub fn title(&self) -> &str { &self.title }
    pub fn file(&self) -> &path::Path { &self.file }
    pub fn sub_chapters(&self) -> &[Chapter] { &self.sub_chapters }
}
