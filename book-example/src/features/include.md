# File inclusion

mdBook gives you a nice little feature that allows you to embed the contents of files inside your book.
It allows to include the same thing multiple times and cut on boilerplate, or include code that needs to stay
in sync with the original source code. 

To include a file, you simply type `\{{#include file.txt }}`

I can, for example, include the configuration file for this book:

```toml
{{#include ../../book.toml }}
```

There is no copy paste involved, your book will never get out of sync with your source files! mdBook will also complain if it can't 
find the file to include. If you move the file, your book will not silently break. It will do so... *LOUDLY*!