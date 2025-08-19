# Raytracer in Rust

This project is an implementation of the classic ["Ray Tracing in One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html) tutorial, originally written in C++, but rewritten in idiomatic Rust.

## Features

- Modular Rust codebase
- Core raytracing primitives: vectors, rays, spheres, and hittable lists
- Output in PPM image format
- Designed for clarity and extensibility

## Project Structure

- `src/vec3.rs` – 3D vector math
- `src/ray.rs` – Ray structure and utilities
- `src/color.rs` – Color output helpers
- `src/hittable.rs` – Hittable trait and hit record
- `src/hittable_list.rs` – List of hittable objects
- `src/sdf/` – Signed distance field shapes (starting with spheres)
- `src/common.rs` – Shared constants and utility functions
- `src/main.rs` – Program entry point

## Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Build and run:
   ```sh
   cargo run --release
   ```
3. The output image will be written to `output.ppm`.

## Goals

- Learn Rust by porting a well-known C++ raytracer
- Write clean, idiomatic, and safe Rust code
- Make it easy to extend with new features and shapes
