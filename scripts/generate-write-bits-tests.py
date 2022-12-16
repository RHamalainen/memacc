from more_itertools import sliced


def add_separator(value: str) -> str:
    return "_".join(list(sliced(value, 4, strict=True)))


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


if __name__ == "__main__":
    initials = ["00000000", "10100110", "11001010", "11111111"]
    starts = [0, 1, 2, 3, 4, 5, 6, 7]
    values = ["00000000", "10100110", "11001010", "11111111"]
    lengths = [1, 2, 3, 4, 5, 6, 7, 8]

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
                    print(
                        f"assert_eq!(0b{t_initial}u8.write_bits({start}, 0b{t_value}u8, {length}), 0b{expected}u8);"
                    )
