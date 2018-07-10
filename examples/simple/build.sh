mkdir -p out
$RUSTC simple_framework.rs --crate-type rlib -o out/libsimple_framework.rlib
$RUSTC lib.rs --test -o out/lib_testbin -L ./out/
