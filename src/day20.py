from collections import defaultdict


def read_input_lines(filename: str) -> list[str]:
    with open("data/" + filename) as f:
        return f.readlines()


def get_pane_range(pane):
    max_row_idx = max(pane.keys())
    min_row_idx = min(pane.keys())
    max_col_idx = -1
    min_col_idx = 0
    for row in pane.values():
        max_col_idx = max([max_col_idx, *row.keys()])
        min_col_idx = min([min_col_idx, *row.keys()])

    return min_row_idx, max_row_idx, min_col_idx, max_col_idx


def print_pane(pane):
    min_row_idx, max_row_idx, min_col_idx, max_col_idx = get_pane_range(pane)

    pane_to_print = []
    for row_idx in range(min_row_idx, max_row_idx + 1):
        for col_idx in range(min_col_idx, max_col_idx + 1):
            if pane.get(row_idx, {}).get(col_idx) == "#":
                pane_to_print.append("#")
            else:
                pane_to_print.append(".")

        pane_to_print.append("\n")

    print("".join(pane_to_print))


def print_pane_size(pane):
    dims = get_pane_range(pane)
    min_dim = min(dims)
    diff = abs(min_dim)
    dims = [d + diff for d in dims]
    print(f"{dims=}")


def get_cell_bin_idx(row_idx, col_idx, pane, algorithm, background):
    fallback_symbol = background

    value_str = (
        pane.get(row_idx - 1, {}).get(col_idx - 1, fallback_symbol)
        + pane.get(row_idx - 1, {}).get(col_idx, fallback_symbol)
        + pane.get(row_idx - 1, {}).get(col_idx + 1, fallback_symbol)
        + pane.get(row_idx, {}).get(col_idx - 1, fallback_symbol)
        + pane.get(row_idx, {}).get(col_idx, fallback_symbol)
        + pane.get(row_idx, {}).get(col_idx + 1, fallback_symbol)
        + pane.get(row_idx + 1, {}).get(col_idx - 1, fallback_symbol)
        + pane.get(row_idx + 1, {}).get(col_idx, fallback_symbol)
        + pane.get(row_idx + 1, {}).get(col_idx + 1, fallback_symbol)
    )

    value_bin = "".join("0" if c == "." else "1" for c in value_str)

    value = int(value_bin, base=2)

    new_value = algorithm[value]

    return new_value


def perform_step(pane, algorithm, background):
    new_pane = defaultdict(dict)
    min_row_idx, max_row_idx, min_col_idx, max_col_idx = get_pane_range(pane)
    diff = 1
    min_row_idx = min_row_idx - diff
    max_row_idx = max_row_idx + diff
    min_col_idx = min_col_idx - diff
    max_col_idx = max_col_idx + diff

    # new_minor, new_major = major_symbol, minor_symbol

    all_dark_symbol = algorithm[0]
    all_light_symbol = algorithm[-1]

    for row_idx in range(min_row_idx, max_row_idx + 1):
        for col_idx in range(min_col_idx, max_col_idx + 1):
            new_value = get_cell_bin_idx(row_idx, col_idx, pane, algorithm, background)
            # if new_value == new_minor:
            new_pane[row_idx][col_idx] = new_value

    # Change major_symbol

    # all_dark_symbol = algorithm[0]
    # all_light_symbol = algorithm[-1]

    if background == ".":
        new_background = all_dark_symbol
    elif background == "#":
        new_background = all_light_symbol
    else:
        breakpoint()
        raise RuntimeError()

    return new_pane, new_background


def count_lits(pane):
    count = 0
    for row in pane.values():
        for c in row.values():
            if c == "#":
                count += 1
    return count


def solve_task(lines: list[str], steps: int) -> int:
    algorithm = lines[0].strip()
    background = "."

    image_lines = lines[2:]

    pane = defaultdict(dict)

    for row_idx, line in enumerate(image_lines):
        for col_idx, c in enumerate(line):
            if c == "#":
                pane[row_idx][col_idx] = "#"

    # To print initial pane.

    # print_pane_size(pane)
    for _ in range(steps):
        # print_pane(pane)
        pane, background = perform_step(pane, algorithm, background)

    # print_pane(pane)
    # print_pane_size(pane)

    return count_lits(pane)

DAY_NO = 20
TASK_1 = 1
TASK_2 = 2
DATA = f"day{DAY_NO}.txt"
TEST_DATA = f"day{DAY_NO}_test.txt"

def main():
    import sys

    filename = dict(enumerate(sys.argv)).get(1, TEST_DATA)
    task_no = int(dict(enumerate(sys.argv)).get(2, TASK_1))
    # filename = "day20_test.txt"
    lines = read_input_lines(filename)
    if task_no == TASK_1:
        result = solve_task(lines, 2)
    elif task_no == TASK_2:
        result = solve_task(lines, 50)
    else:
        raise RuntimeError()

    # 6202 - too high.
    # 5704 - too high.
    # 5703 - wrong.
    # 5581 - wrong.

    print(f"{task_no=}, {filename=}, {result=}")


if __name__ == "__main__":
    main()
