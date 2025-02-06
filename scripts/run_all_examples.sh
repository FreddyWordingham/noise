for file in examples/*.rs; do
  example=$(basename "$file" .rs)
  cargo run --release --example "$example"
done
