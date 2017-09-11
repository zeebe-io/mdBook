# The serve command

The `serve` command is useful when you want to preview your book while writing. When launched, mdBook will watch all your files and trigger a build on any change. Additionally, it will also reload the webpage whenever a file change was detected. This is very convenient when writing books because the process is reduced to writing and saving your files while mdBook instantly updates the preview.

#### Specify a directory

Like all the other sub-commands, `serve` can take an optional directory as argument to use instead of the
current working directory.

```bash
mdbook serve path/to/book
```


#### Server options

The `serve` command works by serving the pages on `localhost:3000` and running a websocket server on `localhost:3001`. The `serve` command can be configured by four different parameters: the http port, the websocket port, the interface to serve on and the public address of the server so that the browser may reach the websocket server.

**For example:**  
Suppose you have an nginx server for SSL termination which has a public address of `192.168.1.100` on port 80 and proxies that to `127.0.0.1` on port 8000. To run use the nginx proxy, you would run `serve` with the following parameters:

```bash
mdbook serve path/to/book -p 8000 -i 127.0.0.1 -a 192.168.1.100
```

If you were to want live reloading for this you would need to proxy the websocket calls through nginx as well from `192.168.1.100:<WS_PORT>` to `127.0.0.1:<WS_PORT>`. The `-w` flag allows for the websocket port to be configured.

#### --open

When you use the `--open` (`-o`) option, mdbook will open the book in your
your default web browser after starting the server.

#### --dest-dir

The `--dest-dir` (`-d`) option allows you to change the output directory for your book.

-----

***note:*** *the `serve` command has not gotten a lot of testing yet, there could be some rough edges. If you discover a problem, please report it [on Github](https://github.com/azerupi/mdBook/issues)*
