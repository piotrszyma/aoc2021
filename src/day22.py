from collections import defaultdict
from typing import Iterable, Literal, Optional
import sys
from dataclasses import dataclass


@dataclass
class CoordRange:
    start: int
    end: int

StepKind = Literal["on", "off"]

@dataclass
class Step:
    kind: StepKind  # either on or off
    x: CoordRange
    y: CoordRange
    z: CoordRange


def read_input_lines(filename: str) -> list[str]:
    with open("data/" + filename) as f:
        return f.readlines()


def steps_from_lines(lines: list[str], min_pos: int, max_pos: int) -> Iterable[Step]:
    for line in lines:
        kind, ranges = line.split(" ")
        ranges = ranges.split(",")
        ranges = [r[2:] for r in ranges]
        ranges = [r.split("..") for r in ranges]
        ranges = [CoordRange(start=int(r[0]), end=int(r[1])) for r in ranges]

        if any((r.start > max_pos) for r in ranges):
            continue

        if any((r.end < min_pos) for r in ranges):
            continue

        ranges = [CoordRange(start=max(min_pos, r.start), end=min(max_pos, r.end)) for r in ranges]

        yield Step(
            kind="on" if kind == "on" else "off",
            x=ranges[0],
            y=ranges[1],
            z=ranges[2],
        )


def _is_step_in_range(step: Step, min_pos: int, max_pos: int):
    return (
        step.x.start >= min_pos
        and step.x.end <= max_pos
        and step.y.start >= min_pos
        and step.y.end <= max_pos
        and step.z.start >= min_pos
        and step.z.end <= max_pos
    )


def solve_task1(lines: list[str]) -> int:
    min_pos = -50
    max_pos = 50
    steps = steps_from_lines(lines, min_pos, max_pos)

    cubes_kinds: dict[tuple[int, int, int], StepKind]= dict()

    for step in steps:
        for x in range(step.x.start, step.x.end + 1):
            for y in range(step.y.start, step.y.end + 1):
                for z in range(step.z.start, step.z.end + 1):
                    cubes_kinds[(x, y, z)] = step.kind

    ctr = 0
    for x in range(min_pos, max_pos + 1):
        for y in range(min_pos, max_pos + 1):
            for z in range(min_pos, max_pos + 1):
                kind = cubes_kinds.get((x, y, z))
                if kind is not None and kind == "on":
                    ctr += 1
    return ctr

def on_ranges_add(on_ranges: list[CoordRange], range_to_add: CoordRange) -> list[CoordRange]:
    updated_on_range = []
    for range in on_ranges:
        if range.end < range_to_add.start:
            updated_on_range.append(range)
        elif range.end > range_to_add.start:
            if range_to_add.end <= range.end:
                return on_ranges
            elif range_to_add.end > range.end:
                # updated_on_range.append()


def get_on_ranges(steps: list[Step], axis: Literal['x', 'y', 'z']):
    on_ranges: list[CoordRange] = []
    for step in steps:
        if step.kind == "on":
            range: Optional[CoordRange] = getattr(step, axis)
            if range is None:
                raise ValueError(f"Invalid {axis=}")
            on_ranges = on_ranges_add(on_ranges, range)

def solve_task2(lines: list[str]) -> int:
    min_pos = -sys.maxsize
    max_pos = sys.maxsize
    steps = steps_from_lines(lines, min_pos, max_pos)

    cubes_kinds: dict[tuple[int, int, int], StepKind]= dict()

    for step in steps:
        for x in range(step.x.start, step.x.end + 1):
            for y in range(step.y.start, step.y.end + 1):
                for z in range(step.z.start, step.z.end + 1):
                    cubes_kinds[(x, y, z)] = step.kind

    breakpoint()
    ctr = 0
    for x in range(min_pos, max_pos + 1):
        for y in range(min_pos, max_pos + 1):
            for z in range(min_pos, max_pos + 1):
                kind = cubes_kinds.get((x, y, z))
                if kind is not None and kind == "on":
                    ctr += 1
    return ctr




DAY_NO = 22
TASK_1 = "1"
TASK_2 = "2"
DATA = f"day{DAY_NO}.txt"
TEST_DATA = f"day{DAY_NO}_test.txt"


def main():
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
        return

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
