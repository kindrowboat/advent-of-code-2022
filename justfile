help:
  just -l

build target:
  cd {{target}} && cargo build

run target:
  cd {{target}} && cargo run

init number name:
  cargo init {{number}}-{{name}} --name {{name}}
