Nibble types
============

Ways of parsing four-bit integers, i.e. nibbles. You may freely use and modify
this code under the [CC0 1.0 Universal License](LICENSE).

You can find the rustdoc [here](https://docs.charr.xyz/nibble/nibble/).

Usage
-----

To use in your own project, just add the below to your `Cargo.toml` file.

```toml
[dependencies]
nibble = "0.1"
```

If you're interested in using this crate with `no_std`:

```toml
[dependencies]
nibble = { version = "0.1", default-features = false }
```

(note, this currently does not work)
