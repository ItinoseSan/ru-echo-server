# ru-echo-server
A mini echo server implemented in Rust
# Set up
build this repository
```
cargo build
```
Start server
```
cargo run
```
connect to server in telnet 
```
telnet localhost 4500
```
# Demo 
Server side
```
Starting echo server...
Rust echo server started!
Usage
telnet localhost [SET_PORT_NUMBER]
New listening started:TcpListener { addr: V4(127.0.0.1:4500), fd: 3 }
Server received data:Ok(512)
"2018-02-01 10:04:05":Echo server respond!
Server received data:Ok(512)
"2018-02-01 10:04:36":Echo server respond!
```
Client side
```
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
Hello Rust Echo
Hello Rust Echo
Im beginneer of system programming
Im beginneer of system programming
```
