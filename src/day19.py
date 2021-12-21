import sys
import math
import itertools


def read_input_lines(filename: str) -> list[str]:
    with open("data/" + filename) as f:
        return f.readlines()


def solve_task(lines: list[str]) -> int:
    scanner_outputs: list[list[tuple[int, ...]]] = []

    for line in lines:
        if line.strip() == '':
            continue
        elif line.startswith('---'):
            scanner_outputs.append([])
        else:
            point = tuple(int(e) for e in line.strip().split(','))
            scanner_outputs[-1].append(point)


    res = [[]]

    for scanner_output in scanner_outputs[:2]:
        for f, s in itertools.product(scanner_output, scanner_output):
            if f == s:
                continue
            dst = math.sqrt((f[0] - s[0]) ** 2 + (f[1] - s[1]) ** 2 + (f[2] - s[2]) ** 2)
            res[-1].append((f, s, dst))
        res.append([])

    dist_1 = [r[2] for r in res[1]]

    res_0_match_dist_1 = [r for r in res[0] if r[2] in dist_1]
    # TODO: Finish this day.
    
    breakpoint()
    # res.sort(key=lambda e: e[2])
    # dists = set(r[2] for r in res)

    # print(f"{res=}")
    # print(f"{dists=}")
    # print(f"{len(dists)=}")
    # print(f"{scanner_outputs=}")

    return 0

DAY_NO = 19
TASK_1 = 1
TASK_2 = 2
DATA = f"day{DAY_NO}.txt"
TEST_DATA = f"day{DAY_NO}_test.txt"

def main():

    filename = dict(enumerate(sys.argv)).get(1, TEST_DATA)
    task_no = int(dict(enumerate(sys.argv)).get(2, TASK_1))
    lines = read_input_lines(filename)

    if task_no == TASK_1:
        result = solve_task(lines)
    elif task_no == TASK_2:
        result = solve_task(lines)
    else:
        raise RuntimeError()

    print(f"{task_no=}, {filename=}, {result=}")


if __name__ == "__main__":
    main()
