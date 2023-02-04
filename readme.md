# Footlights

## usage

```
cargo r -- --config ./examples/basic.yaml -i assets/input.png -o output.png
```

```
cat assets/input.png | cargo run -- --config examples/basic.yaml --stdin --stdout | xclip -sel clip -t image/png
```
