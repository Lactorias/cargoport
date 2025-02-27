# Cargoport

A simple file transfer tool, using TCP to establish a connection between client and server directories enabling file transfer operations.

# Instructions

1. Clone the repository,
```
$ git clone https://github.com/Lactorias/cargoport.git

$ cd cargoport
```
2. Setup and run the build!
```
$ cargo build
```
3. Run Cargoport!
```
$ cargo run
```
4. Establish connection to the Server!
```
nc 127.0.0.1 8080
```

# Commands

Cargoport supports some simple but effective commands!

#### GET
```
GET filename.ext
```
Get will download the file from the server directory into the client directory.

#### DEL
```
DEL filename.ext
```
Del will delete the file from the server directory.

#### PUT
```
PUT filename.ext
```
Put will download the file from the client directory into the server directory.

#### LIST
```
LIST
```
List will display the names of all files in the server directory.

#### LIST_CLIENT
```
LIST_CLIENT
```
List_Client will display the names of all files in the client directory.

## Plans

- Planning to add more commands.
- Move away from local host. 
- A more user friendly terminal output when displaying files through commands like LIST.

