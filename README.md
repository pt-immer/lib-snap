# `lib-snap`

Bank Indonesia SNAP BI plumbing for every PT IMMER service.

Six crates, each leaf or near-leaf:

| Crate | Role | Depends on |
|---|---|---|
| `kamu-snap-crypto` | HMAC/RSA primitives, SNAP BI recipes, webhook verifier trait | — (leaf) |
| `kamu-snap-response` | Envelope + 61-variant Error + Category + ResponseCode | — (leaf; `crypto` feature pulls `kamu-snap-crypto`) |
| `kamu-snap-crypto-actix` | actix-web inbound-verify helper | `kamu-snap-crypto`, `actix-web` |
| `kamu-snap-crypto-axum` | axum/`http` inbound-verify helper | `kamu-snap-crypto`, `http` |
| `kamu-snap-response-actix` | actix-web `Responder` adapter | `kamu-snap-response`, `actix-web` |
| `kamu-snap-response-axum` | axum `IntoResponse` adapter | `kamu-snap-response`, `axum` |

`kamu-snap-response` compiles cleanly to `wasm32-unknown-unknown` with `--no-default-features`. `kamu-snap-crypto` does too once the consumer enables `getrandom/js` for the transitive `rsa` dep. The actix/axum coupling lives in the adapter crates — opt in by depending on the right adapter for your runtime.

## v2.0 highlights

- Hard v2.0 break: API rename, typo fix, surface tightened, audit clean.
- Both cores are leaves — no transitive `actix-web` pulled into client / CLI / wasm consumers.
- `&self` signers (HMAC, RSA) — no `Mutex` needed under server load.
- Encoding-agnostic `Signature` newtype + `Encoding` enum.
- Sealed `SignatureScheme` trait + 4 built-in RSA schemes (Pkcs1v15 + PSS × SHA-256/SHA-512).
- Defensive `ResponseCode::parse` + inverse `classify()` for typed client-side reasoning.
- `#[non_exhaustive]` on `Error` + `Category`. `Default` derive dropped.
- Payload deserialise errors **propagate** instead of silent-`None`.
- ~55 integration tests landing alongside the rewrite (RFC 4231 HMAC, RSA round-trip, taxonomy table, defensive parser, F-08 regression).

## Per-crate quickstart

- [`kamu-snap-crypto/README.md`](crates/crypto/README.md)
- [`kamu-snap-response/README.md`](crates/response/README.md)

## License

MIT.
