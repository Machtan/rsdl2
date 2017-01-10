# Utilities to get data from my python sdl2 lib

def print_values(obj):
   valid = [m for m in dir(obj) if m[0].isupper()]
   values = [(getattr(obj, m), m) for m in valid]
   values.sort()
   for v, k in values:
       print("{} = {},".format(k, v))
