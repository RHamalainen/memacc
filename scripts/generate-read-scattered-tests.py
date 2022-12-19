"""Generate test cases for read bits scattered.

Problem: exhaustive testing requires massive amount of test cases.

Solution: reduce test space with arbitrary constraints.
"""

from string import Template
from typing import List
from itertools import product

from path_output import PathOutput
from make_header import make_header


def model(initial: str, indices: List[int]) -> str:
    initial_bits = list(reversed(initial))
    bits = ["0" for _ in range(8)]
    for index_bits, index_initial in enumerate(indices):
        bits[index_bits] = initial_bits[index_initial]
    return "".join(list(reversed(bits)))


template_str = """\
assert_eq!(0b${initial}u8.read_bits_scattered(&[${indices}]), 0b${expected}u8);"""
template = Template(template_str)

if __name__ == "__main__":
    path_output = PathOutput("src/bitman/unchecked/read/tests_generated.rs")
    initials = ["00000000", "10100110", "11001010", "11111111"]
    indices_collection: List[List[int]] = list()
    for repeat in [0, 1, 2, 3]:
        for combination in product([0, 1, 3, 7], repeat=repeat):
            indices_collection.append(list(combination))
    output = list()
    output.append(make_header())
    output.append(f"#[cfg(test)]")
    output.append(f"mod tests {{")
    output.append(f"use crate::bitman::unchecked::read::*;")
    output.append(f"#[test]")
    output.append(f"fn test_read_bits_scattered() {{")
    for initial in initials:
        for indices in indices_collection:
            expected = model(initial, indices)
            output.append(
                template.substitute(
                    {
                        "initial": initial,
                        "indices": ", ".join([str(i) for i in indices]),
                        "expected": expected,
                    }
                )
            )
    output.append(f"}}")
    output.append(f"}}")
    output_str = "\n".join(output)
    path_output.write(output_str)
