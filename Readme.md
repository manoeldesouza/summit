
# Sumit

A bare-bones static HTTP server


## Introduction

Summit is designed with only minimum essencial functionality to serve files via 
HTTP. Summit capabilties can be described by what it does not do:

 - Summit serves static files only;
 - Files served are relative to local directory;
 - Either a successful response (200), or Not Found (404) is provided;
 - If a directory is part of the HTTP GET, an index file "index.html" will be 
   assumed instead.


## Compilation

Use the regular Rust compilation via cargo. The executable file will be placed 
in ./target/release directory:

    $ cargo build --release


Then play the example html file using: 

    $ cd example
    $ ../target/release/summit


## Usage

Make sure to execute the program from the directory from which you plan on 
serving files. The current directory will the webserver root. If no TCP_PORT is 
specified, summit will defaults to TCP port 8080.

    $ summit [TCP_PORT] 


## Author

Manoel de Souza <manoel.desouza@outlook.com.br>

April 2020
