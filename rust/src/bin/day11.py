""" Credits: https://www.reddit.com/r/adventofcode/comments/18fmrjk/comment/kcxbp2b/?utm_source=share&utm_medium=web2x&context=3 """

from itertools import accumulate, combinations
import sys
from pprint import pprint as pp


def solve(file_name, spacing):
    rows = list(open(file_name).read().splitlines())
    rng = range(len(rows))
    cols = [[row[i] for row in rows] for i in rng]
    ys = list(accumulate(1 if "#" in y else spacing for y in rows))
    xs = list(accumulate(1 if "#" in x else spacing for x in cols))
    points = [(xs[x], ys[y]) for x in rng for y in rng if rows[y][x] == "#"]

    _points = list()
    for x in rng:
        for y in rng:
            if rows[y][x] == "#":
                _points.append((xs[x], ys[y]))

    assert points == _points

    return sum(
        abs(x1 - x0) + abs(y1 - y0) for (x0, y0), (x1, y1) in combinations(points, 2)
    )


fname = sys.argv[1]
print(solve(fname, 2))  # part1
print(solve(fname, 10**6))  # part1
