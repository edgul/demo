#!/usr/bin/python3 
from typing import List
from copy import deepcopy

def merge(left : List[int], right : List[int]):
    merged = []
  
    l_it = 0
    r_it = 0

    # merge smallest until one list runs out of elements
    while l_it < len(left) and r_it < len(right):
        if left[l_it] <= right[r_it]:
            merged.append(left[l_it])
            l_it += 1
        else:
            merged.append(right[r_it])
            r_it += 1

    # finish remaining left/right list
    while l_it < len(left):
        merged.append(left[l_it])
        l_it += 1
    while r_it < len(right):
        merged.append(right[r_it])
        r_it += 1
    return merged

def merge_with_iterator(left : List[int], right : List[int]):
    if len(left) == 0:
        return right
    if len(right) == 0:
        return left

    # grab first values
    merged = []
    left_it = iter(left)
    right_it = iter(right)
    l_value = next(left_it)
    r_value = next(right_it)

    # merge smallest from each list until we run out of a list
    while True:

        # add to the smalest to merged list and track last and other
        if l_value <= r_value:
            merged.append(l_value)
            last_it = left_it
            other_it = right_it
        else:
            merged.append(r_value)
            last_it = right_it
            other_it = left_it

        # move the iterator along
        try:
            if last_it == left_it: 
                l_value = next(left_it)
            else:
                r_value = next(right_it)

        # finish the remaining list
        except StopIteration:
            if last_it == left_it:
                value = r_value
            else:
                value = l_value
            while True:
                merged.append(value)
                try:
                    value = next(other_it)
                except:
                    break
            break 
    return merged
    
    

def mergesort(l : List[int]):
    # no sorting required for single entry or empty list
    if len(l) == 0:
        return []
    if len(l) == 1:
        return l 
    merge_func = merge_with_iterator # easily change merge implementation
    halfway = int(len(l)/2)
    left_list = l[:halfway]
    right_list = l[halfway:]
    return merge_func(mergesort(left_list), mergesort(right_list))

def main():
    l = [5, 4, 7, 9, 8, -2, 10]
    print("Before: " + str(l))

    l = mergesort(l)
    print("After:  " + str(l))

main()
