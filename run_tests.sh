clear;
clear;

set -ex

cd core_extensions_proc_macros

cargo test --features item_parsing
cargo test --features item_parsing derive

cd ..

cargo test --no-default-features --features "rust_1_46 std derive"
cargo test --no-default-features --features "rust_1_46 std bools"
cargo test --no-default-features --features "rust_1_46 std callable"
cargo test --no-default-features --features "rust_1_46 std collections"
cargo test --no-default-features --features "rust_1_46 std const_default"
cargo test --no-default-features --features "rust_1_46 std const_default derive"
cargo test --no-default-features --features "rust_1_46 std const_val"
cargo test --no-default-features --features "rust_1_46 std macro_utils"
cargo test --no-default-features --features "rust_1_46 std generics_parsing"
cargo test --no-default-features --features "rust_1_46 std item_parsing"
cargo test --no-default-features --features "rust_1_46 std integers"
cargo test --no-default-features --features "rust_1_46 std iterators"
cargo test --no-default-features --features "rust_1_46 std marker_type"
cargo test --no-default-features --features "rust_1_46 std on_drop"
cargo test --no-default-features --features "rust_1_46 std option_result"
cargo test --no-default-features --features "rust_1_46 std phantom"
cargo test --no-default-features --features "rust_1_46 std self_ops"
cargo test --no-default-features --features "rust_1_46 std slices"
cargo test --no-default-features --features "rust_1_46 std strings"
cargo test --no-default-features --features "rust_1_46 std transparent_newtype"
cargo test --no-default-features --features "rust_1_46 std transparent_newtype derive"
cargo test --no-default-features --features "rust_1_46 std type_asserts"
cargo test --no-default-features --features "rust_1_46 std type_identity"
cargo test --no-default-features --features "rust_1_46 std type_level_bool"
cargo test --no-default-features --features "rust_1_46 std void"

cargo test --no-default-features --features "rust_1_46 all_items"
cargo test --no-default-features --features "rust_1_46 alloc all_items"
cargo test --no-default-features --features "rust_1_46 std all_items"

cargo test --no-default-features --features "rust_1_46 all_items_no_derive"
cargo test --no-default-features --features "rust_1_46 alloc all_items_no_derive"
cargo test --no-default-features --features "rust_1_46 std all_items_no_derive"

cargo test --release --no-default-features --features "rust_1_46 all_items";
cargo test --release --no-default-features --features "rust_1_46 alloc all_items";
cargo test --release --no-default-features --features "rust_1_46 std all_items";

cargo test --release --no-default-features --features "rust_1_46 all_items_no_derive";
cargo test --release --no-default-features --features "rust_1_46 alloc all_items_no_derive";
cargo test --release --no-default-features --features "rust_1_46 std all_items_no_derive";
