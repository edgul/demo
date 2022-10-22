# catbox.py
###########
# Given a number of boxes  
# and a cat that moves exactly 1 box left or right
# guess where the cat is in the fewest attempts

# Implementation:
# Brute force and a depth-first recursive approach.
# Doesn't recurse into already explored patterns
# Works on N boxes
#
# 0 -> cat cannot be here
# 1 -> cat could be here
#
# So, for example:
# we know where the cat might be: [0,1,1,0]
# we know where the cat is: 	  [0,0,1,0]
# we have caught the cat:         [0,0,0,0]

# Improvements:
# * Add feature that shows which box was "guessed"
# * Use bitarray instead of an array
# * Don't copy the array so many times 
# * First guess is always wasteful
# * Doesn't always return the shortest route
# * The stack limit will get in the way of large arrays -- need tail recursion 

import sys

def count_ones(arr):
	count = 0 
	for i in arr:
		if i == 1:
			count+=1
	return count
assert count_ones([1]) == 1
assert count_ones([1,1]) == 2
assert count_ones([1,1,1]) == 3
assert count_ones([1,0,1]) == 2
assert count_ones([0,0,1]) == 1
assert count_ones([0,0,0]) == 0

# will zero out an index  
def guess(arr,i):
	arr[i] = 0
	return arr
assert guess([1,1,1], 0) == [0,1,1]
assert guess([1,1,1], 1) == [1,0,1]
assert guess([1,1,1], 2) == [1,1,0]
assert guess([1,0,1], 1) == [1,0,1]
assert guess([1,0,1], 2) == [1,0,0]

def or_bw(arr1, arr2):
	new_arr = arr1[:]
	for i,v in enumerate(arr2):
		if v==1:
			new_arr[i] = 1 
	return new_arr
assert or_bw([1,0,0],[0,1,0]) == [1,1,0]

def move_cat(arr):
	new_arr = [0] * len(arr) 
	for i,v in enumerate(arr):
		if v==1:
			if i-1 >= 0:
				temp = new_arr[:]
				temp[i-1] = 1
				temp[i] = 0
				new_arr = or_bw(new_arr, temp) 
			if i+1 < len(arr):
				temp = new_arr[:]
				temp[i+1] = 1
				temp[i] = 0
				new_arr = or_bw(new_arr, temp) 
	return new_arr
assert move_cat([0,0,0]) == [0,0,0]
assert move_cat([1,0,0]) == [0,1,0]
assert move_cat([0,1,0]) == [1,0,1]
assert move_cat([0,1,1]) == [1,1,1]
assert move_cat([1,1,0]) == [1,1,1]
assert move_cat([0,1,1]) == [1,1,1]
assert move_cat([1,1,1]) == [1,1,1]

# not necessarily the fastest
# arr - initial box probabilities
# previous_states - will contain a path that catches the cat 
def dfs(arr, previous_states):	
	for i in range(len(arr)):
		new_arr = move_cat(arr)
		guess(new_arr, i)
		# check for win
		if count_ones(new_arr) == 0:
			return True
		if not new_arr in previous_states:
			previous_states.append(new_arr)
			if dfs(new_arr, previous_states):
				return True

# catch the cat!
starting_boxes = [1] * 5		
previous_states = []
if dfs(starting_boxes, previous_states):
	print(*previous_states, sep="\n")
	print(len(previous_states))
	sys.exit()	

# this should never happen
print("Error: couldn't catch the cat")
# print(*previous_states, sep="\n")

