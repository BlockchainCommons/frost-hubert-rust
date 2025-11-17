# frost

`frost` is a lightweight command-line helper for working with Flexible Round-Optimized Schnorr Threshold (FROST) participants within the Blockchain Commons toolchain. It currently focuses on validating and recording participants from signed `ur:xid` documents.

## Usage

```
frost participant add [--registry <PATH>] <XID_DOCUMENT> [<PET_NAME>]
```

- `XID_DOCUMENT` must be a valid `ur:xid` string representing an `XIDDocument` that is signed by its inception key.
- `PET_NAME` is an optional human-readable alias. If provided it must be unique within the current directory's `registry.json` registry.
- `--registry <PATH>` overrides where the participant registry is stored. Provide just a filename to keep it in the current directory, a directory path ending in `/` to use the default `registry.json` within that directory, or a path that already contains a filename (absolute or relative) to use it verbatim.

By default the command stores participant details in `registry.json` within the current working directory, creating the file if it does not exist. Re-running the same command with identical arguments is idempotent.

## License

BSD 2-Clause Plus Patent License. See `LICENSE` in the repository root.
