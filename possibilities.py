def main():
    length = 13
    alphabet_length = 40
    possibilities = alphabet_length
    for i in range(1, length):
        possibilities *= possibilities_for_pattern_length(i) / 2
    print(f"possibilities tested: {round(possibilities)}")
    print(f"minutes: {round(possibilities / 1000000 / 60)}")


def possibilities_for_pattern_length(length):
    return 100 / (length + 4)


if __name__ == "__main__":
    main()
