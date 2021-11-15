#!/usr/bin/python3

from enum import Enum

class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3


def doThing(color : Color):
    print(color)

doThing(1)
