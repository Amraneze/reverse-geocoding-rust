# Reverse Geocoding

An application that works as a memory cache to do a reverse geocoding.

## TODO
- [ ] Update README
- [ ] Add tests 
- [ ] Find a way to communicate between Java and this app

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

### Debug mode
```shell
cargo run --package reverse-geocoding --bin reverse-geocoding -- data/osm-20151130-1.0-2.bin
```

### Release mode
```
cargo run --package reverse-geocoding --bin reverse-geocoding --release -- data/osm-20151130-1.0-2.bin
```