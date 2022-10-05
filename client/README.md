# Reverse Geocoding

A client to connect and send commands to Reverse Geocoding application.

## Running the application

We are using [cargo](https://doc.rust-lang.org/cargo/) as a build system and dependencies manager.

### Formatting

To format the code, you can run this command:
````shell
cargo fmt
````

### Build

From the root directory, you can build the application using the following command:
````shell
cargo build
````

### Running the application

```
cargo run -- -t 48.8588897 -g 2.320041
```
or
```
cargo run -- --latitude 48.8588897 --longitude 2.320041
```

>Note: You can also use `host` and `port` options with `latitude` & `longitude`

````
reverse-geocoding-client --host <host> --latitude <latitude> --longitude <longitude> --port <port>
````