# Offline ML with Rust

A simple REST API server built using the [gotham](https://gotham.rs/) web framework, [tract](https://github.com/snipsco/tract) for neural network inference, and [Rust](https://www.rust-lang.org/).

## Slide Deck

This project was created as a demo for an accompanying talk: **Offline ML with Rust**. The [slide deck](docs/offline-ml.pdf) is included in this repo in the `/docs` folder.

## Usage

1. Before running the code, you will need to download the model:

```
wget https://storage.googleapis.com/mobilenet_v2/checkpoints/mobilenet_v2_1.4_224.tgz
tar zxf mobilenet_v2_1.4_224.tgz
```

Most of the downloaded material is junk, we only care about the `mobilenet_v2_1.4_224_frozen.pb` file.

2. Build and start the server:

```
cargo run
```

3. The API server listens for POST requests to the root endpoint with a body containing a serializable image (I've only tested with JPEGs). You can use cURL to post images to the server:

```
curl -i  -X POST -F "image=@<IMAGE_PATH>" http://127.0.0.1:7878/
```

The returned value is a tuple: (score, class). Score is a value between 0 and 1 signifying the model's confidence that the given class is in the image. Class is the line number associated with the predicted class in the [classes text file](./imagenet_slim_labels.txt).
