import re
print(sum(1for x in[re.compile('([^-]*)-([^-]*) (.): (.*)').match(l)for l in open('i','r')]if int(x[1])<=x[4].count(x[3])<=int(x[2])))