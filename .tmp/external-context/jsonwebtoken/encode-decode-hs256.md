---
source: Context7 API + docs.rs + CHANGELOG
library: jsonwebtoken
package: jsonwebtoken
topic: encode-decode-hs256
fetched: 2026-08-09T00:00:00Z
official_docs: https://docs.rs/jsonwebtoken/latest/jsonwebtoken/
---

# jsonwebtoken — current API (v11.0.0, released 2026-07-24)

Latest version: **11.0.0**. Use in Cargo.toml:

```toml
jsonwebtoken = { version = "11", features = ["rust_crypto"] }
```

## ⚠️ GOTCHA #1: you MUST pick a crypto backend feature (v10+)

Since **10.0.0** the crate uses trait-based crypto providers. Features:
- `use_pem` — default, but NOT enough to sign/verify
- `aws_lc_rs` — AWS-LC backend
- `rust_crypto` — pure-Rust backend (RustCrypto: hmac, sha2, p256, rsa, ed25519)

**Only `use_pem` is enabled by default. With plain `jsonwebtoken = "11"` the
crate has NO crypto backend and HS256 encode/decode will not compile/run.
Enable `rust_crypto` (pure Rust, simplest) or `aws_lc_rs`.**

## ⚠️ GOTCHA #2: there is no `decode_with_validation`

Current functions are `decode` (full validation), `decode_header` (no
signature check), and `dangerous::insecure_decode` (no validation — the
replacement for the removed `Validation.insecure_disable_signature_validation`).
`decode` **DOES validate by default** — the premise that "decode no longer
validates" does NOT apply to this crate.

## Encode / decode HS256 (exact API, v11)

```rust
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]   // REQUIRED for both
struct Claims {
    sub: String,     // standard claim
    exp: usize,      // UTC timestamp in seconds (required by default validation)
    user_id: i64,    // custom claim — just add fields
}

let secret = b"your-secret-key";           // reuse one key; keys are zeroized on drop
let claims = Claims { sub: "user-1".into(), exp: /* now + ttl */, user_id: 42 };

// Encode — Header::default() = alg HS256, typ "JWT"
let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))?;
// returns Result<String, jsonwebtoken::errors::Error>

// Decode + validate
let token_data = decode::<Claims>(
    &token,                                 // impl AsRef<[u8]> since v10 (was &str)
    &DecodingKey::from_secret(secret),
    &Validation::new(Algorithm::HS256),
)?;
// returns Result<TokenData<Claims>, Error>
println!("{}", token_data.claims.user_id);
```

Exact signatures (v11.0.0):

```rust
pub fn encode<T: Serialize>(
    header: &Header,
    claims: &T,
    key: &EncodingKey,
) -> Result<String, Error>;

pub fn decode<T: DeserializeOwned>(
    token: impl AsRef<[u8]>,      // &str or &[u8]; was &str before v10
    key: &DecodingKey,
    validation: &Validation,
) -> Result<TokenData<T>>;        // TokenData { header: Header, claims: T }

pub fn get_current_timestamp() -> u64;   // use for iat/exp
```

## Claims struct requirements

- Must derive `Serialize` (for encode) and `Deserialize` (for decode; bound is
  `T: DeserializeOwned` — no `Clone` bound needed since 10.2.0).
- `exp`/`iat`/`nbf` are plain integer UTC timestamps in seconds (`u64`/`usize`;
  floats allowed, rounded to u64). No chrono/time types.
- `iat` is **not** validated by default (spec change since 6.0.0) — fine to
  include as a claim, it's just not checked.
- Custom claims are ordinary struct fields; unknown claims are ignored on decode.

## Validation struct (v11) — defaults

```rust
pub struct Validation {
    pub required_spec_claims: HashSet<String>,  // default {"exp"}
    pub leeway: u64,                            // default 60 (seconds, clock skew)
    pub reject_tokens_expiring_in_less_than: u64, // default 0 (added 9.3.0)
    pub validate_exp: bool,                     // default true
    pub validate_nbf: bool,                     // default false
    pub validate_aud: bool,                     // default true
    pub aud: Option<HashSet<String>>,           // default None
    pub iss: Option<HashSet<String>>,           // default None
    pub sub: Option<String>,                    // default None
    pub algorithms: Vec<Algorithm>,             // default vec![HS256]; errors if empty
}
```

Helpers: `Validation::new(alg)`, `Validation::new_for_family(family)`,
`set_audience(&["a"])`, `set_issuer(&["i"])`, `set_required_spec_claims(&["exp", "aud"])`.

So `Validation::new(Algorithm::HS256)` already: requires `exp` present,
validates it with 60 s leeway, checks `alg` == HS256. This is what you want
for an API — no extra config needed unless you check aud/iss.

Errors (ErrorKind): InvalidToken, InvalidAlgorithm, InvalidSignature,
ExpiredSignature, ImmatureSignature, InvalidIssuer, InvalidAudience,
MissingRequiredClaim, InvalidClaimFormat.

## Breaking changes (8.x → 9.x → 10.x → 11.x)

**8.0.0 → 9.0.0**: leeway default 0 → 60 s; `Validation::iss` now a HashSet;
`decode` errors if `algorithms` is empty; `DecodingKey` owns its data; removed
`decode_*` variants in favor of Validation struct; **9.0.0 rejects tokens that
contain `aud` when the Validation has none configured**.

**9.x → 10.0.0**: crypto backends are trait-based — must enable `aws_lc_rs` or
`rust_crypto`; `token` param `&str` → `impl AsRef<[u8]>`; `Clone` bound added
to `decode` (removed again in 10.2.0); JWS support added; `dangerous::insecure_decode`
added (10.1.0).

**10.x → 11.0.0**: `Algorithm`/`KeyAlgorithm`/`EllipticCurve`/`ThumbprintHash` now
non_exhaustive; `Jwk::thumbprint` returns `Result<_>`; `Header.extras` is a struct;
`Validation.insecure_disable_signature_validation` removed (use `dangerous::insecure_decode`);
`EncodingKey.inner` → `as_bytes`; `DecodingKey.as_bytes`/`try_get_hmac_secret` → `try_get_as_bytes`.

Net for your use case (HS256, no JWK, no PEM): pin `11`, enable `rust_crypto`,
and the 8.x-era `encode`/`decode`/`Validation::new(HS256)` pattern still applies
unchanged except `token` accepts bytes.
