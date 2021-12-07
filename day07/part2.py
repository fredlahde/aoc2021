import math
numbers = [int(i) for i in open("input").read().split(",")]
#numbers = [int(i) for i in open("testinput").read().split(",")]

dist_low = 0xffffffffffffffffffffff
low_input = None
def fuel_cost(xx):
    return (xx**2)/2 +(xx/2)

for xx in range(500):
    dist_now = 0
    for yy in numbers:
        dist = abs(xx-yy)
        dist_now += fuel_cost(dist)
        if dist_now > dist_low:
            break
    if dist_low > dist_now:
        dist_low = dist_now
        low_input = xx

print(dist_low)
print(low_input)
