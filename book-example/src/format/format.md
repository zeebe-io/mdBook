# Configuration

The input of mdBook is a collection of markdown files with the structure described by a `SUMMARY.md` file.
In this section, we will go into more details on how to correctly structure your book and files and what
options are available to customize your book to your needs.

In the [init](cli/init.html) section, we already discovered the most basic and default structure of a book.

```bash
book-test/
├── book
└── src
    ├── chapter_1.md
    └── SUMMARY.md
```

The `src` or source directory, is where all your source files will reside. Additionally, there may be special files like the `SUMMARY.md` file defining the structure of your book.

When building your book, the output files will be located in the `book` directory. If you want to view your book, open the `index.html` file in your browser. Alternatively, you can pass the `--open` parameter when building your book and it should automatically open in your browser.

In the [configuration](format/config.html) section, we will discuss another very important files, the `book.toml` file, that allows you to configure different parameters for your book. It is in that file that you can specify some general metadata about your book, like the author and language, but also some rendering options like additional style sheets.
