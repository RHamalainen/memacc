from more_itertools import sliced


def add_separator(value: str) -> str:
    assert isinstance(value, str)
    return "_".join(list(sliced(value, 4, strict=True)))
