extern crate mdbook;

mod dummy_book;

use dummy_book::DummyBook;
use mdbook::MDBook;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::renderer::{Renderer, RenderContext};
use mdbook::book::Book;
use mdbook::config::Config;
use mdbook::errors::*;

use std::sync::{Arc, Mutex};

struct DummyPreprocessor(Arc<Mutex<bool>>);

impl Preprocessor for DummyPreprocessor {
    fn name(&self) -> &str {
        "dummy"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        *self.0.lock().unwrap() = true;
        Ok(book)
    }
}

/// A dummy renderer with a custom name.
struct DummyRenderer(Arc<Mutex<bool>>, &'static str);

impl Renderer for DummyRenderer {
    fn name(&self) -> &str {
        self.1
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        *self.0.lock().unwrap() = true;
        Ok(())
    }
}

#[test]
fn mdbook_runs_preprocessors() {
    let has_run: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let temp = DummyBook::new().build().unwrap();
    let cfg = Config::default();

    let mut book = MDBook::load_with_config(temp.path(), cfg).unwrap();
    book.with_preprecessor(DummyPreprocessor(Arc::clone(&has_run)));
    book.build().unwrap();

    assert!(*has_run.lock().unwrap())
}

/// Specify a preprocessor and it should work with *only* the one specific 
/// renderer.
#[test]
fn name() {
    
}