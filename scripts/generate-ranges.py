from itertools import product

if __name__ == "__main__":
    indices = [0, 1, 2, 3, 4, 5, 6, 7]
    for combination in product(indices, repeat=2):
        if combination[0] <= combination[1]:
            start, final = combination
            print(f"{start}..={final}")
