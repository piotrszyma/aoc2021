from dataclasses import dataclass
from copy import copy


def read_input_lines(filename: str) -> list[str]:
    with open("data/" + filename) as f:
        return f.readlines()


class Dice:
    def __init__(self):
        self.next_result = 1
        self.rolls_count = 0

    def roll(self):
        self.rolls_count += 1
        value = self.next_result
        self.next_result += 1
        if self.next_result > 100:
            self.next_result = 1
        return value


class Game:
    def __init__(self, player1_pos: int, player2_pos: int):
        self.player1_pos = player1_pos
        self.player2_pos = player2_pos
        self.player1_score = 0
        self.player2_score = 0

    def won_by_player1(self):
        return self.player1_score >= 21

    def won_by_player2(self):
        return self.player2_score >= 21

    def player1_roll(self, roll_result):
        self.player1_pos += roll_result
        self.player1_pos %= 10

    def player2_roll(self, roll_result):
        self.player2_pos += roll_result
        self.player2_pos %= 10

    def update_player1_score(self):
        self.player1_score += self.player1_pos + 1

    def update_player2_score(self):
        self.player2_score += self.player2_pos + 1


def product(*args, repeat=1):
    # product('ABCD', 'xy') --> Ax Ay Bx By Cx Cy Dx Dy
    # product(range(2), repeat=3) --> 000 001 010 011 100 101 110 111
    pools = [tuple(pool) for pool in args] * repeat
    result = [[]]
    for pool in pools:
        result = [x + [y] for x in result for y in pool]
    for prod in result:
        yield tuple(prod)


def solve_task1(lines: list[str]) -> int:
    player1_pos = int(lines[0].strip()[-1]) - 1 % 10
    player2_pos = int(lines[1].strip()[-1]) - 1 % 10
    player1_score = 0
    player2_score = 0

    dice = Dice()

    while player1_score < 1000 and player2_score < 1000:

        # player1 move
        for _ in range(3):
            roll_result = dice.roll()
            print(f"{roll_result=}")
            player1_pos += roll_result
            player1_pos %= 10
        player1_score += player1_pos + 1

        if player1_score >= 1000:
            break

        # player2 move
        roll_result = dice.roll()
        print(f"{roll_result=}")
        player2_pos += roll_result
        player2_pos %= 10

        roll_result = dice.roll()
        print(f"{roll_result=}")
        player2_pos += roll_result
        player2_pos %= 10

        roll_result = dice.roll()
        print(f"{roll_result=}")
        player2_pos += roll_result
        player2_pos %= 10
        player2_score += player2_pos + 1

    print(f"{player1_score=} {player2_score=}")

    loser_score = min(player1_score, player2_score)
    return loser_score * dice.rolls_count


@dataclass
class Result:
    p1: int
    p2: int

    def __add__(self, other: "Result") -> "Result":
        return Result(p1=self.p1 + other.p1, p2=self.p2 + other.p2)


def counts_given(
    p1_pos, p2_pos, p1_points_needed, p2_points_needed, is_player1_move, cache=dict
) -> Result:
    # print(f"{p1_pos=}, {p2_pos=}, {p1_points_needed=}, {p2_points_needed=}, {is_player1_move=}")
    assert not (p1_points_needed <= 0 and p2_points_needed <= 0)

    if p1_points_needed <= 0:
        assert p2_points_needed > 0
        res = Result(p1=1, p2=0)
        return res

    if p2_points_needed <= 0:
        assert p1_points_needed > 0
        res = Result(p1=0, p2=1)
        return res

    if cached := cache.get(
        (p1_pos, p2_pos, p1_points_needed, p2_points_needed, is_player1_move)
    ):
        return cached

    results: list[Result] = []
    current_pos = p1_pos if is_player1_move else p2_pos
    current_points_needed = p1_points_needed if is_player1_move else p2_points_needed

    for (first_move, second_move, third_move) in product(
        [1, 2, 3], [1, 2, 3], [1, 2, 3]
    ):
        score_in_three_moves = first_move + second_move + third_move

        new_pos = (current_pos + score_in_three_moves) % 10
        new_points_needed = current_points_needed - new_pos - 1

        new_p1_pos = new_pos if is_player1_move else p1_pos
        new_p2_pos = p2_pos if is_player1_move else new_pos
        new_p1_points_needed = (
            new_points_needed if is_player1_move else p1_points_needed
        )
        new_p2_points_needed = (
            p2_points_needed if is_player1_move else new_points_needed
        )

        result = counts_given(
            p1_pos=new_p1_pos,
            p2_pos=new_p2_pos,
            p1_points_needed=new_p1_points_needed,
            p2_points_needed=new_p2_points_needed,
            is_player1_move=not is_player1_move,
            cache=cache,
        )

        results.append(result)

    final_result = Result(p1=0, p2=0)

    for r in results:
        final_result += r

    cache[
        (p1_pos, p2_pos, p1_points_needed, p2_points_needed, is_player1_move)
    ] = final_result
    return final_result


def solve_task2(lines: list[str]) -> int:
    player1_pos = int(lines[0].strip()[-1]) - 1 % 10
    player2_pos = int(lines[1].strip()[-1]) - 1 % 10

    print(f"{player1_pos+1=} {player2_pos+1=}")

    points_needed = 21

    cache = {}

    # breakpoint()
    results = counts_given(
        player1_pos,
        player2_pos,
        points_needed,
        points_needed,
        is_player1_move=True,
        cache=cache,
    )

    print(f"{results=}")

    return max(results.p1, results.p2)


DAY_NO = 21
TASK_1 = '1'
TASK_2 = '2'
DATA = f"day{DAY_NO}.txt"
TEST_DATA = f"day{DAY_NO}_test.txt"


def main():
    import sys

    filename = dict(enumerate(sys.argv)).get(1)
    task_no = dict(enumerate(sys.argv)).get(2)

    if not filename:
        data_lines = read_input_lines(DATA)
        test_data_lines = read_input_lines(TEST_DATA)

        result = solve_task1(test_data_lines)
        assert result == 739785, result
        result = solve_task1(data_lines)
        assert result == 989352, result
        result = solve_task2(test_data_lines)
        assert result == 444356092776315, result
        result = solve_task2(data_lines)
        assert result == 430229563871565, result
    else:
        lines = read_input_lines(filename)
        if task_no == TASK_1:
            result = solve_task1(lines)
        elif task_no == TASK_2:
            result = solve_task2(lines)
        else:
            raise RuntimeError()

    print(f"{task_no=}, {filename=}, {result=}")


if __name__ == "__main__":
    main()
    # print(sorted(sum(p) for p in product([1,2,3], [1,2,3], [1,2,3])))
    # print(len(list(product([1,2,3], [1,2,3], [1,2,3]))))
