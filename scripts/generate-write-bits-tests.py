from string import Template

from add_separator import add_separator
from path_output import PathOutput
from make_header import make_header


def model(initial: str, start: int, value: str, length: int) -> str:
    result = list(reversed(list(initial)))
    value = "".join(list(reversed(list(value))))
    written = 0
    for i in range(len(initial)):
        if start <= i:
            if length <= written:
                break
            result[i] = value[i - start]
            written += 1
    result = list(reversed(result))
    result_combined = "".join(result)
    return result_combined


template = Template(
    "assert_eq!(0b${initial}u8.write_bits(${start}, 0b${value}u8, ${length}), 0b${expected}u8);"
)


if __name__ == "__main__":
    path_output = PathOutput("src/bitman/unchecked/write/tests_generated_write_bits.rs")
    initials = ["00000000", "10100110", "11001010", "11111111"]
    starts = [0, 1, 2, 3, 4, 5, 6, 7]
    values = ["00000000", "10100110", "11001010", "11111111"]
    lengths = [1, 2, 3, 4, 5, 6, 7, 8]
    output = list()
    output.append(make_header())
    output.append(f"#[cfg(test)]")
    output.append(f"mod tests {{")
    output.append(f"use crate::bitman::unchecked::write::*;")
    output.append(f"#[test]")
    output.append(f"fn test_write_bits() {{")
    for initial in initials:
        for start in starts:
            for value in values:
                for length in lengths:
                    if 8 < start + length:
                        continue
                    expected = model(initial, start, value, length)
                    t_initial, t_value, expected = [
                        add_separator(str(x)) for x in [initial, value, expected]
                    ]
                    output.append(
                        template.substitute(
                            {
                                "initial": t_initial,
                                "start": start,
                                "value": t_value,
                                "length": length,
                                "expected": expected,
                            }
                        )
                    )
    output.append(f"}}")
    output.append(f"}}")
    output_str = "\n".join(output)
    path_output.write(output_str)
