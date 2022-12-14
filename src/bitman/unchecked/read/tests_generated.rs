//! This file is automatically generated.
//!
//! Date: 2022-12-19T22:11:11
#[cfg(test)]
mod tests {
use crate::bitman::unchecked::read::*;
#[test]
fn test_read_bits_scattered() {
assert_eq!(0b00000000u8.read_bits_scattered(&[]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 0, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 0, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 0, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 0, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 1, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 1, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 1, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 1, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 3, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 3, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 3, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 3, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 7, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 7, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 7, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[0, 7, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 0, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 0, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 0, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 0, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 1, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 1, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 1, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 1, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 3, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 3, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 3, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 3, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 7, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 7, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 7, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[1, 7, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 0, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 0, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 0, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 0, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 1, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 1, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 1, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 1, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 3, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 3, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 3, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 3, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 7, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 7, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 7, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[3, 7, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 0, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 0, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 0, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 0, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 1, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 1, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 1, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 1, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 3, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 3, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 3, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 3, 7]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 7, 0]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 7, 1]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 7, 3]), 0b00000000u8);
assert_eq!(0b00000000u8.read_bits_scattered(&[7, 7, 7]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 1]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 7]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 0]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 1]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 3]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 7]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 1]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 7]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 0]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 1]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 3]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 7]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 0, 0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 0, 1]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 0, 3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 0, 7]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 1, 0]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 1, 1]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 1, 3]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 1, 7]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 3, 0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 3, 1]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 3, 3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 3, 7]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 7, 0]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 7, 1]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 7, 3]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[0, 7, 7]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 0, 0]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 0, 1]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 0, 3]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 0, 7]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 1, 0]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 1, 1]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 1, 3]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 1, 7]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 3, 0]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 3, 1]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 3, 3]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 3, 7]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 7, 0]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 7, 1]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 7, 3]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[1, 7, 7]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 0, 0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 0, 1]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 0, 3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 0, 7]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 1, 0]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 1, 1]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 1, 3]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 1, 7]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 3, 0]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 3, 1]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 3, 3]), 0b00000000u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 3, 7]), 0b00000100u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 7, 0]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 7, 1]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 7, 3]), 0b00000010u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[3, 7, 7]), 0b00000110u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 0, 0]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 0, 1]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 0, 3]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 0, 7]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 1, 0]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 1, 1]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 1, 3]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 1, 7]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 3, 0]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 3, 1]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 3, 3]), 0b00000001u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 3, 7]), 0b00000101u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 7, 0]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 7, 1]), 0b00000111u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 7, 3]), 0b00000011u8);
assert_eq!(0b10100110u8.read_bits_scattered(&[7, 7, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[]), 0b00000000u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0]), 0b00000000u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 0]), 0b00000000u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 1]), 0b00000010u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 3]), 0b00000010u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 7]), 0b00000010u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 0]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 1]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 3]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 7]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 0]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 1]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 3]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 7]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 0]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 1]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 3]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 7]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 0, 0]), 0b00000000u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 0, 1]), 0b00000100u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 0, 3]), 0b00000100u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 0, 7]), 0b00000100u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 1, 0]), 0b00000010u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 1, 1]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 1, 3]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 1, 7]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 3, 0]), 0b00000010u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 3, 1]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 3, 3]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 3, 7]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 7, 0]), 0b00000010u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 7, 1]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 7, 3]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[0, 7, 7]), 0b00000110u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 0, 0]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 0, 1]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 0, 3]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 0, 7]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 1, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 1, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 1, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 1, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 3, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 3, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 3, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 3, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 7, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 7, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 7, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[1, 7, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 0, 0]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 0, 1]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 0, 3]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 0, 7]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 1, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 1, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 1, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 1, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 3, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 3, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 3, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 3, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 7, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 7, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 7, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[3, 7, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 0, 0]), 0b00000001u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 0, 1]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 0, 3]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 0, 7]), 0b00000101u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 1, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 1, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 1, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 1, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 3, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 3, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 3, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 3, 7]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 7, 0]), 0b00000011u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 7, 1]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 7, 3]), 0b00000111u8);
assert_eq!(0b11001010u8.read_bits_scattered(&[7, 7, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[]), 0b00000000u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0]), 0b00000001u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1]), 0b00000001u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3]), 0b00000001u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7]), 0b00000001u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 0]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 1]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 3]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 7]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 0]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 1]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 3]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 7]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 0]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 1]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 3]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 7]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 0]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 1]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 3]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 7]), 0b00000011u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 0, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 0, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 0, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 0, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 1, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 1, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 1, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 1, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 3, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 3, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 3, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 3, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 7, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 7, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 7, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[0, 7, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 0, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 0, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 0, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 0, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 1, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 1, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 1, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 1, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 3, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 3, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 3, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 3, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 7, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 7, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 7, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[1, 7, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 0, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 0, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 0, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 0, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 1, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 1, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 1, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 1, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 3, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 3, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 3, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 3, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 7, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 7, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 7, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[3, 7, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 0, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 0, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 0, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 0, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 1, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 1, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 1, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 1, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 3, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 3, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 3, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 3, 7]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 7, 0]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 7, 1]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 7, 3]), 0b00000111u8);
assert_eq!(0b11111111u8.read_bits_scattered(&[7, 7, 7]), 0b00000111u8);
}
}