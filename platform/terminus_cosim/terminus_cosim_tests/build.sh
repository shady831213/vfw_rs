#!/bin/sh
set -e
TESTNAME=$1
#you may modify the $LIBCLANG_PATH and $RISCV_TOOLCHAIN_PREFIX
TESTNAME=$TESTNAME LIBCLANG_PATH=/usr/lib/llvm-12/lib RISCV_TOOLCHAIN_PREFIX=riscv32-unknown-elf- cargo build -p terminus_cosim_tests -Zunstable-options --release --out-dir target/$TESTNAME --bin $TESTNAME
${RUSTUP_HOME}/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objdump -D target/$TESTNAME/$TESTNAME >| target/$TESTNAME/$TESTNAME.dump