#!/bin/env python3

def main(sequence):
    hashmap = [[] for i in range(256)]
    for hash in sequence.strip().split(','):
        current_value = 0
        fl = '0'
        tag = None
        if hash[-1] == '-':
            tag = hash[0:-1]
        elif hash[-2] == "=":
            tag = hash[0:-2]
            fl = hash[-1]
        else:
            print("Malformed hash: ", hash)
        for c in tag:
                current_value += ord(c)
                current_value = current_value*17
                current_value = current_value%256
        box = hashmap[current_value]
        if fl == '0':
            #remove
            box = [*filter(lambda e: e[0] != tag, box)]
            hashmap[current_value]=box
        else:
            replaced = False
            #add
            for lens in box:
                if lens[0] == tag:
                    #replace
                    lens[1] = fl
                    replaced = True
                    break
            if replaced == False:
                box.append([tag,fl])
    sum = 0
    boxfactor = 0
    for box in hashmap:
        boxfactor += 1
        slot = 0
        for lens in box:
            slot+=1
            sum += boxfactor * slot *int(lens[1])
    return sum

if __name__ == "__main__":
    print("\nAnswer: ",main(input("Insert/Pipe sequence: ")),sep="")

