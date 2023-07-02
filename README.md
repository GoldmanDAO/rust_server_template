# Install
First go with `cargo build`, you may run into some issues with the`diesel` library, no worries I got you covered.

If you don't have it already install `postgresql` on your machine.

On Mac -> `brew install postgresql`

Then go and run 

```bash
cargo install diesel_cli --no-default-features --features postgresql
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup
```

# Run
Use `watchexec` for hot reloading the project (still need to manually reload the browser)

```bash
watchexec --restart -- cargo run
```
