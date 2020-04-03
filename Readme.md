
# Sumit

A bare-bones static HTTP server


## Introduction

Summit is designed with only minimum essencial functionality to serve files via 
HTTP. Summit capabilties can be described by what it does not do:

 - Summit serves static files only;
 - Files served are relative to local directory;
 - Either a successful response is provided (200), or Not Found (404);
 - The only "concession" is the automatic appending of "index.html" if the URL
   consists of a directory instead of a regular file.


## Compilation

Use the regular Rust compilation via cargo. The executable file will be placed 
in ./target/release directory:

    $ cargo build --release


or play with it using: 

    $ cargo run


## Usage

Make sure to execute the program from the directory from which you plan on 
serving files. The current directory will the webserver root. If no TCP_PORT is 
specified, summit will defaults to TCP port 8080.

    $ summit [TCP_PORT] 


## Author

Manoel de Souza <manoel.desouza@outlook.com.br>

March 2020
