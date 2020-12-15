import re
r=lambda k,e,b:sum(int(x,2) for x in b.values())if len(e)<1 else(r(l[7:],e[1:],b)if "k" in (l:=e[0])else r(k,e[1:],b|{(m:=re.match("mem.([0-9]*). =(.*)",l).groups())[0]:"".join(i if j=="X"else j for i,j in zip(format(int(m[1]),'036b'),k))}))
print(r("X"*36,list(open('i','r')),{}))