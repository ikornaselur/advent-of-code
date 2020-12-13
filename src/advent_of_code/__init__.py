import os
from typing import List, Union


class Problem:
    def get_input(self: "Problem", requester: str) -> List[str]:
        dir_name = os.path.dirname(requester)
        with open(os.path.join(dir_name, "input.txt"), "r") as f:
            return [line.strip() for line in f.readlines()]

    def get_solution(self: "Problem") -> Union[int, str]:
        raise NotImplementedError()
