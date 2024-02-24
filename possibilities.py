def main():
    length = 14
    possibilities = 1
    for i in range(0, length):
        possibilities *= possibilities_for_pattern_length(i) / 2
    print(f"possibilities tested: {round(possibilities)}")
    print(f"minutes: {round(possibilities / 1000000 / 60, 2)}")

    print()
    print()
    print()

    for i in range(0, length):
        print(f"{i}: {possibilities_for_pattern_length(i)}")


def possibilities_for_pattern_length(length):
    return 60 / (length + 1) + 1


if __name__ == "__main__":
    main()
