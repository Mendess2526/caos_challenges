from sys import argv
import random
size = int(argv[1])
for i in range(0, size):
    print(random.randint(-100, 100), random.randint(-100, 100), random.randint(-100, 100), end=" ")
