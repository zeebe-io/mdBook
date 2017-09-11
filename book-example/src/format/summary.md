# SUMMARY.md

The summary file is one of the most important files of your book. It is used by mdBook to know what chapters to include in your book,
in what order they should appear, what their hierarchy is and where the source files are located.
Without this file, there is no book.

Even though `SUMMARY.md` is a markdown file, the formatting is very strict to
allow for easy parsing. Let's see how you should format your `SUMMARY.md` file.

Here are the different elements you can find in your `SUMMARY.md` file.

1. ***A title***  
   It's common practice to begin with a title, generally
   <code class="language-markdown"># Summary</code>.
   But it is not mandatory, the parser just ignores it. So you can too, if you feel like it.

2. ***Front-matter chapters***  
   Before the main numbered chapters you can add a couple of chapters that will not be numbered. This is useful for
   forewords, introductions, etc. There are however some constraints. You can not nest front-matter chapters, they should all be on the root level. And you can not mix and match numbered and unnumbered chapters. Once you start with numbered chapters, you can't put any front-matter chapters afterwards.  
   A front-matter chapter is simply represented by a markdown link with the title between brackets and the path to the file in the parentheses. Each chapter is on its own line.
   ```markdown
   [Title of prefix element](relative/path/to/markdown.md)
   ```

3. ***Numbered chapter***  
   Numbered chapters are the main content of the book, they will be numbered and can be nested,
   resulting in a nice hierarchy (chapters, sub-chapters, etc.)
   ```markdown
   - [Title of the Chapter](relative/path/to/markdown.md)
       - [Nested chapter](relative/path/to/sub-chapter.md)
   ```
   You can either use either the `-` or `*` prefix to indicate a numbered chapter.

4. ***Back-matter chapters***  
   After the numbered chapters you can add a couple of non-numbered chapters again, the back-matter. They are the same as front-matter chapters but come after the numbered chapters instead of before. They are used for appendices and references for examples.

5. **Separators**  
   Separators allow to indicate a logical separation between different chapters. In the rendered book, this will be rendered as a very fine horizontal line between two chapters in the sidebar. Separators are simply constituted by a line of dashes `--------`. Three dashes are enough to make a separator, but you can add as many as you want to make it look good.

All other elements are unsupported and will be ignored, at best, or result in an error.

## Example

As an example, here is the `SUMMARY.md` file for this user guide.

```markdown
{{#include ../SUMMARY.md }}
```