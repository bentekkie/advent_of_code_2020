import re
print(sum(1for x in[re.compile('([^-]*)-([^-]*) (.): (.*)').match(l).groups() for l in open('i','r')]if int(x[0])<=x[3].count(x[2])<=int(x[1])))
