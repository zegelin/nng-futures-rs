# nng-futures

Futures-based wrappers for NNG sockets in Rust.
The goal is to simplify the use of NNG sockets in async Rust code.
Wraps the [nng](https://crates.io/crates/nng) crate from Nate Kent.

## Status

| Socket Type | Futures Type |
|-------------|--------------|
| [`Pub0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Pub0) | [`Sink`](https://docs.rs/futures/latest/futures/sink/trait.Sink.html) |
| [`Sub0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Sub0) | [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) |
| [`Bus0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Bus0) | _Not Implemented_ |
| [`Pair0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Pair0) | _Not Implemented_ |
| [`Pair1`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Pair1) | _Not Implemented_ |
| [`Pull0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Pull0) | _Not Implemented_ |
| [`Push0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Push0) | _Not Implemented_ |
| [`Rep0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Rep0) | _Not Implemented_ |
| [`Req0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Req0) | _Not Implemented_ |
| [`Respondent0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Respondent0) | _Not Implemented_ |
| [`Surveyor0`](https://docs.rs/nng/latest/nng/enum.Protocol.html#variant.Surveyor0) | _Not Implemented_ |

Pull requests that add support for additonal socket types welcome.