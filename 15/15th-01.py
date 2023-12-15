#!/bin/env python3

def main(sequence):
    sum = 0
    for hash in sequence.strip().split(','):
        current_value = 0
        for c in hash.strip():
                current_value += ord(c)
                current_value = current_value*17
                current_value = current_value%256
        sum += current_value
    return sum

if __name__ == "__main__":
    print("\nAnswer: ",main(input("Insert/Pipe sequence: ")),sep="")

