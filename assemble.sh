#!/bin/bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=cortex-r-rt

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a



arm-none-eabi-as -EB -mbig-endian -mcpu=cortex-r4f -march=armv7-r -mfpu=vfpv3-d16 -mfloat-abi=soft asm.s -o bin/$crate.o
ar crs bin/armebv7r-none-eabi.a bin/$crate.o

arm-none-eabi-as -EB -mbig-endian -mcpu=cortex-r4f -march=armv7-r -mfpu=vfpv3-d16 -mfloat-abi=hard asm.s -o bin/${crate}-hard.o
ar crs bin/armebv7r-none-eabihf.a bin/${crate}-hard.o

rm bin/${crate}*.o
