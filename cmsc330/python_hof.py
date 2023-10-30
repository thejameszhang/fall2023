from functools import reduce

def is_present(x, lst):
    return reduce(lambda a, b : a or b == x, lst, False)

def count_occ(x, lst):
    return reduce(lambda a, b : a + int(b == x), lst, 0)

def count_occ_2d(x, matrix):
    return reduce(lambda a, b: a + count_occ(x, b), matrix, 0)


