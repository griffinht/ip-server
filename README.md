# rust-ip
similar to [icanhazip.com](https://icanhazip.com)
## Example
```
$ rust-ip
listening on 0.0.0.0:8000
```
Now that the server is running on port 8000, we can connect via `rust-ip` or an HTTP client with a `GET` request
```
$ rust-ip --client localhost:8000
127.0.0.1
```
```
$ curl localhost:8000
127.0.0.1
```
```
$ firefox localhost:8000
```
![firefox](firefox.png)
## Usage
`-h, --help`

print help
#
`-v, --version`

print version
#
`-c, --client <address>`

run as client, connect to address
client will print remote address to `stdout` with a newline

make sure to use fully qualified hostname and port (e.g. `localhost:8000` instead of `localhost`)
#
`-s, --server <address>`

run as server, bind to address (default 0.0.0.0:8000)

server will handle both `rust-ip -c` and HTTP GET requests (like from `curl`)

server will also print client addresses to `stdout` with a newline