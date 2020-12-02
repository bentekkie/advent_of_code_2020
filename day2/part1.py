import re
r=[re.compile('([^-]*)-([^-]*) (.): (.*)').match(l).groups() for l in open('i','r').readlines()]
print(sum(1for x in r if int(x[0])<=x[3].count(x[2])<=int(x[1])))
