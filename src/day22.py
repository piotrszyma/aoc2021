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

        ranges = [
            CoordRange(start=max(min_pos, r.start), end=min(max_pos, r.end))
            for r in ranges
        ]

        yield Step(
            kind="on" if kind == "on" else "off",
            x=ranges[0],
            y=ranges[1],
            z=ranges[2],
        )


def solve_task1(lines: list[str]) -> int:
    min_pos = -50
    max_pos = 50
    steps = steps_from_lines(lines, min_pos, max_pos)

    cubes_kinds: dict[tuple[int, int, int], StepKind] = dict()

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


def _is_range_before(range_to_check: CoordRange, other_range: CoordRange) -> bool:
    return (
        range_to_check.start < other_range.start
        and range_to_check.end < other_range.start
    )


def _is_range_partial_left_overlapping(
    range_to_check: CoordRange, other_range: CoordRange
) -> bool:
    return (
        range_to_check.start < other_range.start
        and range_to_check.end > other_range.end
    )


def _is_range_inside(range_to_check: CoordRange, other_range: CoordRange) -> bool:
    return (
        range_to_check.start > other_range.start
        and range_to_check.end < other_range.end
    )


def _is_range_partial_right_overlapping(
    range_to_check: CoordRange, other_range: CoordRange
) -> bool:
    return (
        range_to_check.start < other_range.end and range_to_check.end > other_range.end
    )


def _is_range_after(range_to_check: CoordRange, other_range: CoordRange) -> bool:
    return (
        range_to_check.start > other_range.start
        and range_to_check.end > other_range.end
    )


def range_add(ranges: list[CoordRange], range_to_add: CoordRange) -> list[CoordRange]:
    updated_ranges = []
    range_added = False

    for range in ranges:
        if _is_range_before(range, range_to_add):
            updated_ranges.append(range)
        elif _is_range_partial_left_overlapping(range, range_to_add):
            new_range = CoordRange(start=range.start, end=range_to_add.end)
            updated_ranges.append(new_range)
            range_added = True
        elif _is_range_inside(range, range_to_add):
            if not range_added:
                updated_ranges.append(range_to_add)
                range_added = True
        elif _is_range_partial_right_overlapping(range, range_to_add):
            if range_added:
                new_range = CoordRange(start=range_to_add.end, end=range.end)
                updated_ranges.append(new_range)
            else:  # Range not yet added.
                new_range = CoordRange(start=range_to_add.start, end=range.end)
                updated_ranges.append(new_range)
                range_added = True
        elif _is_range_after(range, range_to_add):
            updated_ranges.append(range)
        else:
            raise ValueError(f"Invalid ranges relation {range=} {range_to_add=}")

    return updated_ranges


def range_substract(
    ranges: list[CoordRange], range_to_substract: CoordRange
) -> list[CoordRange]:
    updated_ranges = []

    for range in ranges:
        if _is_range_before(range, range_to_substract):
            updated_ranges.append(range)
        elif _is_range_partial_left_overlapping(range, range_to_substract):
            new_range = CoordRange(start=range.start, end=range_to_substract.end - 1)
            updated_ranges.append(new_range)
        elif _is_range_inside(range, range_to_substract):
            continue
        elif _is_range_partial_right_overlapping(range, range_to_substract):
            new_range = CoordRange(start=range_to_substract.start + 1, end=range.end)
            updated_ranges.append(new_range)
        elif _is_range_after(range, range_to_substract):
            updated_ranges.append(range)
        else:
            raise ValueError(f"Invalid ranges relation {range=} {range_to_substract=}")

    return updated_ranges


def build_ranges_for_axis(steps: list[Step], axis: Literal["x", "y", "z"]):
    ranges: list[CoordRange] = []

    for step in steps:
        range: Optional[CoordRange] = getattr(step, axis)
        if range is None:
            raise ValueError(f"Invalid {axis=}")

        if step.kind == "on":
            ranges = range_add(ranges, range)
            print(ranges)
        elif step.kind == "off":
            ranges = range_substract(ranges, range)
        else:
            raise ValueError(f"Invalid step kind {step.kind=}")

    return ranges


def solve_task2(lines: list[str]) -> int:
    min_pos = -sys.maxsize
    max_pos = sys.maxsize
    steps = list(steps_from_lines(lines, min_pos, max_pos))

    x_ranges = build_ranges_for_axis(steps, "x")
    y_ranges = build_ranges_for_axis(steps, "y")
    z_ranges = build_ranges_for_axis(steps, "z")

    breakpoint()

    return 0


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
