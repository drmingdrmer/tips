import math

def facto(n):
  if n == 1:
      return 1
  return n*facto(n-1)

def combi(n, m):
  return facto(n) / facto(m) / facto(n-m)

def approx_combi(n, m):
  u = n * .5
  sig = (n*.5*.5) ** 0.5

  power = - (m-u) ** 2 / 2 / sig / sig
  return 1.0 / (sig * ((2*math.pi)**.5)) * (math.e ** power) * (2**n)

n, m = 12, 3
print("Approximate combination number C({}, {}) by normal distribution:".format(n, m))
print("accurate, approximated:")
print(combi(n, m), '%.3f' % approx_combi(n, m))
