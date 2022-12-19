"""Generate test cases for write bits scattered."""

from datetime import datetime
from string import Template
from typing import List
from itertools import product

from path_output import PathOutput


def model(initial: str, indices: List[int], source: str) -> str:
    initial_bits = list(reversed(initial))
    source_bits = list(reversed(source))
    for index_source, index_result in enumerate(indices):
        initial_bits[index_result] = source_bits[index_source]
    return "".join(list(reversed(initial_bits)))


template_str = """\
assert_eq!(0b${initial}u8.write_bits_scattered(&[${indices}], 0b${value}u8), 0b${expected}u8);"""
template = Template(template_str)

if __name__ == "__main__":
    path_output = PathOutput(
        "src/bitman/unchecked/write/tests_generated_write_bits_scattered.rs"
    )
    initials = ["00000000", "10100110", "11001010", "11111111"]
    values = ["00000000", "10100110", "11001010", "11111111"]
    indices_collection: List[List[int]] = list()
    for repeat in [0, 1, 2, 3]:
        for combination in product([0, 1, 3, 7], repeat=repeat):
            indices_collection.append(list(combination))
    output = list()
    output.append(f"//! This file is automatically generated.")
    output.append(f"//!")
    output.append(f"""//! Date: {datetime.now().isoformat(timespec="seconds")}""")
    output.append(f"#[cfg(test)]")
    output.append(f"mod tests {{")
    output.append(f"use crate::bitman::unchecked::write::*;")
    output.append(f"#[test]")
    output.append(f"fn test_write_bits_scattered() {{")
    for initial in initials:
        for indices in indices_collection:
            for value in values:
                expected = model(initial, indices, value)
                output.append(
                    template.substitute(
                        {
                            "initial": initial,
                            "indices": ", ".join([str(i) for i in indices]),
                            "value": value,
                            "expected": expected,
                        }
                    )
                )
    output.append(f"}}")
    output.append(f"}}")
    output_str = "\n".join(output)
    path_output.write(output_str)
