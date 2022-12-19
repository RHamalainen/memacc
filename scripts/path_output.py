from pathlib import Path


class PathOutput:
    """Path from crate's root."""

    def __init__(self, path_str: str) -> None:
        assert isinstance(path_str, str)
        self.path = Path(__file__).parent.parent / Path(path_str)

    def write(self, text: str) -> None:
        assert isinstance(text, str)
        with self.path.open("w") as stream:
            stream.write(text)
