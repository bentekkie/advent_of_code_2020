import re
print(sum(1for x in[re.compile('([^-]*)-([^-]*) (.): (.*)').match(l)for l in open('i','r')]if(x[4][int(x[1])-1]==x[3])+(x[4][int(x[2])-1]==x[3])==1))
