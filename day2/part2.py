import re
p=re.compile('([^-]*)-([^-]*) (.): (.*)')
r=[p.match(l).groups() for l in open('i','r').readlines()]
print(sum(1for x in r if(x[3][int(x[0])-1]==x[2])+(x[3][int(x[1])-1]==x[2])==1))
