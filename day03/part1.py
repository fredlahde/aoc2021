from collections import Counter
bit_stream = open("input").read().split("\n")

gamma = 0
ll = len(bit_stream[0])
mem = {}
for bits in bit_stream:
    if bits == "":
        continue
    for n in range(ll):
        if bits[n] == " ":
            continue
        bit = int(bits[n])
        if n in mem:
            mem[n].append(bit)
        else:
            mem[n] = [bit]

gamma = 0
epsilon = 0
ll = len(mem.keys())
for (n, bits) in mem.items():
    most_significant = Counter(bits).most_common(2)[0][0]
    least_significant = Counter(bits).most_common(2)[1][0]
    gamma |= most_significant << ll-n-1
    epsilon |= least_significant << ll-n-1
print(bin(gamma), bin(epsilon), gamma, epsilon, gamma * epsilon)
