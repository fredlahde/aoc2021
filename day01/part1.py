numbers = open("input").read().split('\n')

last = int(numbers[0])
count = 0
for nn in numbers:
    if nn == '':
        continue
    if (int(nn)) > last:
        count = count+1
    last = int(nn)
print(count)
