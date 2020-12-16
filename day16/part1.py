import re
import math
z=lambda l:((m:=re.match("(.*):(.*)-(.*)or(.*)-(.*)",l).groups())[0],lambda x: int(m[1])<=x<=int(m[2])or int(m[3])<=x<=int(m[4]))
s={(r:=z(l))[0]:r[1]for l in open('r','r')}
print(sum(int(v)for t in open('i','r')for v in t.split(',')if not any(s[r](int(v))for r in s)))
