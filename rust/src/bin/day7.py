from collections import Counter, defaultdict

def ms(lst, tab):
    LEN = len(lst)
    if LEN < 2:
        return lst
    
    mid = LEN // 2
    
    left = ms(lst[:mid], tab)
    right = ms(lst[mid:], tab)


    temp = list()
    i, j = 0, 0
    while i < len(left) and j < len(right):
        lval = left[i]
        rval = right[j]

        lval = [tab[c] for c in lval]
        rval = [tab[c] for c in rval]

        for l, r in zip(lval, rval):
            if l < r:
                temp.append(left[i])
                i += 1
                break
            if r < l:
                temp.append(right[j])
                j += 1
                break


    while i < len(left):
        temp.append(left[i])
        i += 1
        
    while j < len(right):
        temp.append(right[j])
        j += 1

    return temp

with open('inputs/day7.in', 'r') as f:
    I = f.read().split("\n")

# cards strength
V = dict(A = 14, K = 13, Q = 12, J = 11, T = 10)
V2 = dict(A = 14, K = 13, Q = 12, J = 1, T = 10)
for i in range(2, 10):
    V[str(i)] = i
    V2[str(i)] = i

combinations = ['fiveok', 'fourok', 'fullh', 'threeok', 'twopair', 'onepair', 'hc']
S = defaultdict(list)
S2 = defaultdict(list)
bid_map = dict()

# part 1
for lines in I:
    if lines == '':
        continue

    [hand, bid] = lines.split(" ")
    bid_map[hand] = int(bid)
    C = Counter(hand)
    vals = C.values()

    if 5 in vals:
        S['fiveok'].append(hand)
    elif 4 in vals:
        S['fourok'].append(hand)
    elif 3 in vals and 2 in vals:
        S['fullh'].append(hand)
    elif 3 in vals:
        S['threeok'].append(hand)
    elif len([x for x in vals if x == 2]) == 2:
        S['twopair'].append(hand)
    elif 2 in vals:
        S['onepair'].append(hand)
    else:
        S['hc'].append(hand)


M = list()
for k in combinations[::-1]:
    srted = ms(S[k], V)
    M.extend(srted)

p1 = 0
for (rank, hand) in enumerate(M, 1):
    p1 += rank * bid_map[hand]

print(f'Part 1: {p1}')

# part 2
for lines in I:
    if lines == '':
        continue

    [hand, bid] = lines.split(" ")
    bid_map[hand] = int(bid)
    C = Counter(hand)

    vals = list()
    maxx = 0
    maxx_idx = 0 
    idx = 0
    jkr_cnt = 0
    for [k , v] in C.items():
        if k == 'J':
            jkr_cnt += v
            continue
        
        if maxx < v:
            maxx = v
            maxx_idx = idx
            

        vals.append(v)
        idx += 1

    if jkr_cnt == 5:
        S2['fiveok'].append(hand)
        continue

    vals[maxx_idx] += jkr_cnt

    if 5 in vals:
        S2['fiveok'].append(hand)
    elif 4 in vals:
        S2['fourok'].append(hand)
    elif 3 in vals and 2 in vals:
        S2['fullh'].append(hand)
    elif 3 in vals:
        S2['threeok'].append(hand)
    elif len([x for x in vals if x == 2]) == 2:
        S2['twopair'].append(hand)
    elif 2 in vals:
        S2['onepair'].append(hand)
    else:
        S2['hc'].append(hand)


M = list()
for k in combinations[::-1]:
    srted = ms(S2[k], V2)
    M.extend(srted)

p2 = 0
for (rank, hand) in enumerate(M, 1):
    p2 += rank * bid_map[hand]

print(f'Part 2: {p2}')