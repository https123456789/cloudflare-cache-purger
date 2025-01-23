# Cloudflare Cache Purger

A simple program that will purge Cloudflare caches

## Usage

Set the following environment variables:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ZONE_ID`

You can the run it:

```
# Specific files
cargo run -- https://example.com/file1 https://example.com/file2/nested

# Everything
cargo run -- --purge-all
```
