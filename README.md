# Gearbox

Gearbox is a versatile library that encompasses a wide array of functionalities, including networking, logging,
railway-oriented programming extensions, and time management. Initially designed as a collection of utilities, the
ultimate vision for Gearbox is to evolve into a highly optimized, standalone toolkit. The goal is to minimize external
dependencies progressively, leading to a library that is lightweight and efficient. By doing so, Gearbox aims to be
universally compatible, from embedded systems to WebAssembly (WASM) environments, all while maintaining simplicity and
minimizing the need for boilerplate code. This development strategy positions Gearbox as a comprehensive solution for
developers seeking to build efficient, scalable applications across a broad spectrum of platforms.

## Features

| Category   | Feature               | use                         | Description                                                                                                                                                                |
|------------|-----------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Common     | TryDefault            | gearbox::common::TryDefault | This is just a trait that is used internally in `Gearbox` but is defining a `TryDefault` Trait that return a Result<T,Self::Error> that can also be used in other systems. |
| Logging    | Tracing Log Formatter | gearbox::log::fmt::*        | This is a custom subscriber for formatting logs when using the rust Tracing libaray                                                                                        
| Networking | hostname              | gearbox::net::hostname      | Get the hostname of the local machine.                                                                                                                                     |
|            | HTTP Request          | gearbox::net::http::*       | Send an HTTP request, this is currently an extension on top of `Reqwest` but simplifies the implementation of mTLS and payload signing.                                    |

## TODO

- [ ] ( gearbox::log::* ) Clean up Log fmt/syslog, some of the code can be combined and cleaned up a bit better, also
  the formatter supports syslog, and bunyan, this should probably be cleared up a bit more, and separated better.
- [ ] ( gearbox::path::* ) current this system is mainly just exposing the dirs::* library, this should be removed.
