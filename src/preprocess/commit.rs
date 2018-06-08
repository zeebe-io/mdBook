use errors::Result as BookResult;

use super::{Preprocessor, PreprocessorContext};
use book::{Book, BookItem};

use git2::{self, Repository};

/// A preprocessor for expanding {{commit}} helpers in a chapter
pub struct CommitHashPreprocessor;

impl CommitHashPreprocessor {
    /// Create a new `CommitHasPreprocessor`
    pub fn new() -> Self {
        CommitHashPreprocessor
    }
}

impl Preprocessor for CommitHashPreprocessor {
    fn name(&self) -> &str {
        "commit-hash"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: &mut Book) -> BookResult<()> {
        if let Ok(hash) = commit_hash() {
            info!("replacing commit template with '{}'", hash);
            book.for_each_mut(|section: &mut BookItem| {
                if let BookItem::Chapter(ref mut ch) = *section {
                    ch.content = ch.content.replace("{{commit}}", &hash);
                }
            });
        }

        Ok(())
    }
}

fn commit_hash() -> Result<String, git2::Error> {
    let repo = Repository::discover(".")?;

    let hash = repo.head()?
        .peel_to_commit()?
        .id();

    Ok(format!("{}", hash))

}
