# avf-qr

A command-line application for macOS that uses the `AVFoundation` framework to capture video from a camera and detect QR codes in real-time.

## Features

This project supports two different backend frameworks for QR code detection, which can be chosen at compile time using Cargo features:

- `vision`: (Default) Uses the `Vision` framework for barcode detection. This is the recommended and default feature.
- `core-image`: Uses the `Core Image` framework for barcode detection.

Only one of these features can be enabled at a time.

- `catch-errors`: This feature enables a mechanism to catch Objective-C exceptions and prevent the application from crashing. This can be useful for debugging.

## Building and Running

### Prerequisites

- macOS
- Rust and Cargo installed
- A video capture device (e.g., a webcam)

### Building and Running

To build and run the project with the default `vision` feature:

```sh
cargo run
```

To build and run with the `core-image` feature:

```sh
cargo run --no-default-features --features core-image
```

To enable the `catch-errors` feature along with another feature (e.g., `vision`):

```sh
cargo run --features "vision,catch-errors"
```

The application will start capturing video from the default camera. When a QR code is detected, information about it will be printed to the console.
