from collections import defaultdict, deque
import sys

def gcd(x, y):
    while y != 0:
        x, y = y, x % y
    return x 

def lcm(x, y):
    return abs(x * y) // gcd(x, y)

def p2(lst):
    [fir, *other] = lst

    for num in other:
        fir = lcm(fir, num)
    
    return fir

fname = sys.argv[1]
with open(fname, 'r') as f:
    I = f.read().split("\n")

G = defaultdict(tuple)
instr = [c for c in I[0]]
L = len(instr)

for line in I[2:]:
    if line == '':
        continue

    [dest, choi] = line.split(' = ')
    choi = ''.join([c for c in choi if c not in ['(', ')']])
    choi = tuple(c for c in choi.split(', '))

    G[dest] = choi

start = 'AAA'
end = 'ZZZ'

steps = 0
p1 = 0
curr_pos = start
idx = 0

ghosts = [g for g in G if g[-1] == 'A']
LCM = [None for _ in ghosts]

while True:
    steps += 1
    choices = G[curr_pos]
    nxt_pos = choices[0] if instr[idx] == 'L' else choices[1]
    curr_pos = nxt_pos

    if curr_pos == end and p1 == 0:
        p1 = steps

    for y in range(len(ghosts)):
        cg_pos = ghosts[y]
        ch = G[cg_pos]
        nxtg_pos = ch[0] if instr[idx] == 'L' else ch[1]
        ghosts[y] = nxtg_pos
        
        if nxtg_pos[-1] == 'Z':
            LCM[y] = steps
    
    if all([c is not None for c in LCM]):
        break
    
    idx += 1
    if idx >= L:
        idx = 0


print(f'Part 1: {p1}')
print(f'Part 2: {p2(LCM)}')
