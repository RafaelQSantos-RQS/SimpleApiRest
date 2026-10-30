---
source: Context7 API + docs.rs
library: argon2
package: argon2
topic: hash-verify-phc
fetched: 2026-08-09T00:00:00Z
official_docs: https://docs.rs/argon2/latest/argon2/
---

# argon2 — current API (v0.5.3, released 2026-06-06)

Latest version: **0.5.3**. Cargo.toml:

```toml
argon2 = "0.5"
```

## Feature flags

Default features (3): `alloc`, `password-hash`, `rand`. Optional: `std`
(implies alloc + password-hash/std), `simple` (implies password-hash), `zeroize`.

- **`password-hash` is ON by default** → `hash_password` / `verify_password`
  and the `PasswordHasher` / `PasswordVerifier` traits (re-exported from the
  `password-hash 0.5` crate) are available with a plain `argon2 = "0.5"`.
- You only hit the "requires password-hash feature" error if you disable
  default features. For a std web API you can also add `features = ["std"]`
  (adds std impls of the password-hash traits), though it's not required.
- MSRV: 1.65. Supports no_std + no-alloc.

## Hash a password (exact API)

```rust
use argon2::{
    password_hash::{
        rand_core::OsRng,          // rand_core is re-exported
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

let password = b"hunter42";        // NOTE: hash_password takes &[u8], so password.as_bytes()

// Random salt, 22-char base64 (RECOMMENDED_SALT_LEN)
let salt = SaltString::generate(&mut OsRng);

// Argon2::default() = Argon2id v19 with Params::DEFAULT
let argon2 = Argon2::default();

// Returns Result<PasswordHash, argon2::Error>; to_string() gives the PHC string
let phc_string = argon2.hash_password(password, &salt)?.to_string();
// e.g. $argon2id$v=19$m=19456,t=2,p=1$<salt-b64>$<hash-b64>
```

## Verify a password (exact API)

```rust
let parsed = PasswordHash::new(&phc_string)?;   // parse stored PHC string
let ok = Argon2::default()
    .verify_password(password, &parsed)
    .is_ok();                                   // Result<(), password_hash::Error>
```

Trait signatures (from password-hash 0.5):

```rust
pub trait PasswordHasher {
    type Params: ...;
    fn hash_password<'a>(
        &self,
        password: &[u8],
        salt: &'a SaltString,
    ) -> Result<PasswordHash<'a>, Error>;
}

pub trait PasswordVerifier {
    fn verify_password(
        &self,
        password: &[u8],
        hash: &PasswordHash<'_>,
    ) -> Result<(), Error>;
}
```

⚠️ Gotcha: `verify_password` uses the **parameters embedded in the PHC hash**,
not the params of the `Argon2` instance. Any `Argon2` instance works for
verification — keep `Argon2::default()` for both. `Error` also implements
`Display`; don't leak "password mismatch" vs "bad hash format" distinctions to
the client if you care about user enumeration (verify_password returns
different Error variants for both; return the same 401).

## Params — default is OWASP-compliant, acceptable for a web API

```rust
// Params::DEFAULT (what Argon2::default() uses):
pub const DEFAULT_M_COST: u32 = 19 * 1024;  // 19456 KiB = 19 MiB
pub const DEFAULT_T_COST: u32 = 2;          // 2 iterations
pub const DEFAULT_P_COST: u32 = 1;          // 1 lane
pub const DEFAULT_OUTPUT_LEN: usize = 32;   // 32-byte digest
```

This matches OWASP's Argon2id recommendation (m=19456, t=2, p=1) and is fine
for a typical API (~50-100 ms per hash, 19 MiB RAM transiently). Use default.

If you must tune (e.g. embedded/low-memory box), build params explicitly:

```rust
use argon2::{Argon2, Algorithm, Version, Params};

let params = Params::new(19456, 2, 1, Some(32))?;   // m,t,p,output_len
let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params)?;
// or: let argon2 = Argon2::from(params);   // From<Params> impl exists
```

`Params::new` constraints: m_cost between `8*p_cost` and u32::MAX (1 KiB blocks),
t_cost 1..u32::MAX, p_cost 1..0xFFFFFF. Errors: MemoryTooLittle, TimeTooSmall,
ThreadsTooFew/TooMany, OutputTooShort/Long.

There's also `ParamsBuilder` for incremental config (`.m_cost(..).t_cost(..)
.build()`).

## Storing the hash (SQLite / sqlx)

Store the **whole PHC string** as TEXT in one column (`password_hash TEXT NOT
NULL`). Never split it. Example row value:

```
$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg5MDEyMzQ1Ng$+dXYkOzUzRzLWHh2ePmhY5qKzlPPpCnvGBQVmVvO1xY
```

```rust
// insert
sqlx::query("INSERT INTO users (email, password_hash) VALUES (?, ?)")
    .bind(email)
    .bind(phc_string)          // String
    .execute(&pool).await?;

// login check
let row: (String,) = sqlx::query_as("SELECT password_hash FROM users WHERE email = ?")
    .bind(email)
    .fetch_optional(&pool).await?
    .ok_or("user not found")?;
let parsed = PasswordHash::new(&row.0)?;
Argon2::default().verify_password(password.as_bytes(), &parsed)?;
```

Key derivation note: `hash_password` is for password *verification*. If you
ever need KDF output bytes, use `Argon2::default().hash_password_into(pw, salt, &mut out)`.
