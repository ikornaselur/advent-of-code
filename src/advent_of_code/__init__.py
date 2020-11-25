from typing import Union


class Problem:
    def get_solution(self: "Problem") -> Union[int, str]:
        raise NotImplementedError()
