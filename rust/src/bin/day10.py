import sys

fname = sys.argv[1]
with open(fname, "r") as f:
    inn = f.read()

G = list()
s_cords = ...
row = 0
for line in inn.splitlines():
    temp = list()
    for col in range(len(line)):
        el = line[col]
        if el.lower() == "s":
            s_cords = (row, col)
        temp.append(el)

    G.append(temp)
    row += 1

RB = len(G)
CB = len(G[0])

to_move = {
    # right
    "[0, 1]": [c for c in "-J7"],
    # up
    "[-1, 0]": [c for c in "|7F"],
    # down
    "[1, 0]": [c for c in "|LJ"],
    # left
    "[0, -1]": [c for c in "-LF"],
}

from_move = {
    "|": [[-1, 0], [1, 0]],
    "-": [[0, -1], [0, 1]],
    "L": [[-1, 0], [0, 1]],
    "J": [[-1, 0], [0, -1]],
    "7": [[1, 0], [0, -1]],
    "F": [[1, 0], [0, 1]],
}

# (prev_cords, curr_cords)
stack = list()

for [row, col] in [[0, 1], [-1, 0], [1, 0], [0, -1]]:
    (rc, cc) = (s_cords[0] + row, s_cords[1] + col)
    if rc < 0 or rc >= RB or cc < 0 or cc >= CB:
        continue

    el = G[rc][cc]
    if el not in to_move[str([row, col])]:
        continue

    stack.append((s_cords, (rc, cc)))

step = 0
tr = []
tl = []
traversed = list()
traversed.append(s_cords)
while True:
    for i in range(len(stack)):
        [prev_cords, curr_cords] = stack[i]

        curr_el = G[curr_cords[0]][curr_cords[1]]

        if len(tr) < len(tl):
            tr.append(curr_cords)
        else:
            tl.append(curr_cords)

        for row, col in from_move[curr_el]:
            (rc, cc) = (curr_cords[0] + row, curr_cords[1] + col)

            if rc == prev_cords[0] and cc == prev_cords[1]:
                continue

            stack[i] = (curr_cords, (rc, cc))

    step += 1
    if stack[0][1] == stack[1][1]:
        tr.append(stack[0][1])
        step += 1
        break

for c in tr:
    traversed.append(c)

for c in tl[::-1]:
    traversed.append(c)

print(f"Part 1: {step}")


def shoelace_area(points):
    # https://en.wikipedia.org/wiki/Pick%27s_theorem
    # https://en.wikipedia.org/wiki/Shoelace_formula
    n = len(points)
    res = 0
    for i in range(n):
        x1, y1 = points[i]
        x2, y2 = points[(i + 1) % n]
        res += x1 * y2 - x2 * y1

    return abs(res) // 2


inner_points = shoelace_area(traversed) - step + 1
print(f"Part 2: {inner_points}")
