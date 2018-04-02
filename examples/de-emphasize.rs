//! This program removes all forms of emphasis from the markdown of the book.
extern crate mdbook;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use mdbook::errors::{Error, Result};
use mdbook::MDBook;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::fmt::cmark;

use std::ffi::OsString;
use std::env::{args, args_os};
use std::process;
use std::cell::RefCell;

struct Deemphasize;

impl Preprocessor for Deemphasize {
    fn name(&self) -> &str {
        "md-links-to-html-links"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        eprintln!("Running '{}' preprocessor", self.name());
        let mut total_removed = 0;

        for section in &mut book.sections {
            if let BookItem::Chapter(ref mut ch) = *section {
                eprintln!("{}: processing chapter '{}'", self.name(), ch.name);
                let (num_removed, new_content) = Deemphasize::remove_emphasis(ch)?;

                ch.content = new_content;
                total_removed += num_removed;
            }
        }

        eprintln!(
            "{}: removed {} events from markdown stream.",
            self.name(),
            total_removed
        );

        Ok(book)
    }
}

fn do_it(book: OsString) -> Result<()> {
    let mut book = MDBook::load(book)?;
    book.with_preprecessor(Deemphasize);
    book.build()
}

fn main() {
    if args_os().count() != 2 {
        eprintln!("USAGE: {} <book>", args().next().expect("executable"));
        return;
    }
    if let Err(e) = do_it(args_os().skip(1).next().expect("one argument")) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

impl Deemphasize {
    fn remove_emphasis(chapter: &Chapter) -> Result<(usize, String)> {
        let mut num_removed = 0;
        let mut buf = String::with_capacity(chapter.content.len());

        {
            // new scope so we don't borrow num_removed for too long
            let events = Parser::new(&chapter.content).filter(|e| {
                let should_keep = match *e {
                    Event::Start(Tag::Emphasis)
                    | Event::Start(Tag::Strong)
                    | Event::End(Tag::Emphasis)
                    | Event::End(Tag::Strong) => false,
                    _ => true,
                };
                if !should_keep {
                    num_removed += 1;
                }
                should_keep
            });

            cmark(events, &mut buf, None)
                .map_err(|err| Error::from(format!("Markdown serialization failed: {}", err)))?;
        }

        Ok((num_removed, buf))
    }
}
