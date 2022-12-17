# memacc

[![paypal](https://img.shields.io/badge/Support_my_work-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=E648MA54L53J6)

This crate is work in progress.

## How to depend

Use with standard library.

`memacc = "*"`

Use without standard library.

`memacc = { version = "*", features = ["no-std"] }`

## How to use

Usage not recommended until interface is frozen.

## Conceptual framework

This section is work in progress.

|Requirement|Status|
|---|---|
|Indexing finite collection with out-of-bounds index must fail on compile time.||

|Type|`IndexU8`|`IndexU32`|
|---|---|---|
|`I::<0>`-`I::<7>`|:heavy_check_mark:|:heavy_check_mark:|
|`I::<8>`-`I::<31>`|:x:|:heavy_check_mark:|

![Overview.](/doc/dia/Diagram2.svg)

## TODO

- maybe add scatter read & write?
- maybe give traits in different crate? implementations in different crate
