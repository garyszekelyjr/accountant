import argparse

from typing import List


class TArgs:
    command: List[str]
    verbose: bool

    parser = argparse.ArgumentParser()

    def __init__(self):
        self.parser.add_argument("command", nargs="+")
        self.parser.add_argument("--verbose", "-v", action=argparse.BooleanOptionalAction)

    def parse_args(self):
        args = self.parser.parse_args()
        for key, value in vars(args).items():
            setattr(self, key, value)
        return self
