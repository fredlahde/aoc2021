numbers = list(map(lambda x: int(x), filter(
    lambda y: y != '', open("input").read().split('\n'))))

last = int(numbers[0])
count = 0
windows = []
for i in range(len(numbers)):
    window = []
    window.append(numbers[i])
    if i+1 < len(numbers):
        window.append(numbers[i+1])
    if i+2 < len(numbers):
        window.append(numbers[i+2])
    windows.append(window)
last = sum(windows[0])
for window in windows:
    s = sum(window)
    if s > last:
        count = count +1
    last = s
print(count)
