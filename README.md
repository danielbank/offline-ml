# Offline ML with Rust

A simple REST API server built using the [gotham](https://gotham.rs/) web framework, [tract](https://github.com/snipsco/tract) for neural network inference, and [Rust](https://www.rust-lang.org/).

## Slide Deck

This project was created as a demo for an accompanying talk: **Offline ML with Rust**. The [slide deck](docs/offline-ml.pdf) is included in this repo in the `/docs` folder.

## Usage

Start the server by running `cargo run`

The API server listens for POST requests to the root endpoint with a body containing a serializable image (I've only tested with JPEGs). You can use cURL to upload images:

```
curl -i  -X POST -F "image=@<IMAGE_PATH>" http://127.0.0.1:7878/
```

## License

[MIT License](LICENSE)
