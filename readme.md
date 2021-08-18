# Smartsheet

This program makes API calls to Smartsheet's events endpoint and writes the
information to a CSV file to be ingested by Splunk.

The program is written in Rust and mainly uses the reqwest, serde, tokio, and
polars crates.

It was compiled on a Mac for a CentOS 7 server. Use the command below to compile:

CROSS_COMPILE=x86_64-linux-musl- cargo build --target=x86_64-unknown-linux-musl --release
