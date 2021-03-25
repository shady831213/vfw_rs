#!/bin/sh
set -e
TESTNAME=$1
TESTNAME=$TESTNAME LIBCLANG_PATH=/usr/lib/llvm-12/lib RISCV_TOOLCHAIN_PREFIX=riscv32-unknown-elf- cargo build -p terminus_cosim_tests -Zunstable-options --release --out-dir target/$TESTNAME --bin $TESTNAME
TESTNAME=$TESTNAME LIBCLANG_PATH=/usr/lib/llvm-12/lib RISCV_TOOLCHAIN_PREFIX=riscv32-unknown-elf- cargo objdump -p terminus_cosim_tests --release --bin $TESTNAME -- -D >|target/$TESTNAME/$TESTNAME.dump
