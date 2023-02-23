## TCP tester tool 

This TCP tester tool is written in Rust using **Tokio** crate for networking.

It tests a TCP endpoint by creating N simultaneous connections to the endpoint and executing M io-operations to every connection.
By default, this took writes the "ping\n" message to the connection and reads from this connection. 
The request is considered successful if both write and read operations are successful. 
If reading from the connection is not expected, you can set the flat "-w" to execute only write operations to the connection. 

### Available flags:
* -c NUM - number of simultaneous connections, default 1;
* -r NUM - number of requests to every connection, default 1;
* -w - execute only write operations. If absent, read+write operations are executed.

### Launch examples:

Creates 1 connection; executes 1 read+write request; endpoint localhost:5001.
```bash
cargo run -- localhost:5001
```

Creates 10 connections; executes 20 write requests; endpoint localhost:5002.
```bash
cargo run -- -c 10 -r 20 -w localhost:5002
```