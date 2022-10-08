This project is a binary crate, which is an executable. The rand crate is a library crate.

To create project:
`cargo init --vcs=git`

To build and run:
`cargo run`

After adding a new dependency update the registry of crates:
`cargo build`

Update and change Cargo.lock:
`cargo update`

SemVer - Semantic Versioning
The number 0.8.3 is a shorthand for ^0.8.3, which means any version that is at least 0.8.3 but below 0.9.0.

Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io.

After updating the registry, Cargo checks the [dependencies]. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

TOML - Tom’s Obvious, Minimal Language
