from collections import Counter, OrderedDict
bit_stream = open("input").read().split("\n")

numbers_kept_most = bit_stream.copy()
numbers_kept_least = bit_stream.copy()
n = 0
while True:
    mem = []
    for bits in numbers_kept_most:
        if bits == "":
            continue
        if bits[n] == " ":
            continue
        bit = int(bits[n])
        mem.append(bit)

    bits = mem
    mc = Counter(mem).most_common(2)
    if (mc[1][1] == mc[0][1]):
        most_significant = 1 
    else:
        most_significant = Counter(mem).most_common(2)[0][0]
        try:
            least_significant = Counter(mem).most_common(2)[1][0]
        except IndexError:
            least_significant = most_significant
    tmp_most = []
    found = 0
    for number in numbers_kept_most:
        if number == "":
            continue
        x = int(number[n])
        if x == most_significant:
            tmp_most.append(number)
            found += 1
    numbers_kept_most = tmp_most
    print(numbers_kept_most)
    if len(numbers_kept_most) == 1:
        break

    n += 1

print("-------")
n = 0
while True:
    mem = []
    for bits in numbers_kept_least:
        if bits == "":
            continue
        if bits[n] == " ":
            continue
        bit = int(bits[n])
        mem.append(bit)

    print(mem)
    bits = mem
    mc = Counter(mem).most_common(2)
    if (mc[1][1] == mc[0][1]):
        least_significant = 0
    else:
        most_significant = Counter(mem).most_common(2)[0][0]
        try:
            least_significant = Counter(mem).most_common(2)[1][0]
        except IndexError:
            least_significant = most_significant
    tmp_least = []
    found = 0
    for number in numbers_kept_least:
        if number == "":
            continue
        x = int(number[n])
        if x == least_significant:
            tmp_least.append(number)
            found += 1
    numbers_kept_least = tmp_least
    print(numbers_kept_least)
    if len(numbers_kept_least) == 1:
        break

    n += 1

print(int(numbers_kept_most[0], 2)*int(numbers_kept_least[0], 2))
