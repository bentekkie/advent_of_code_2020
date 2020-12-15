import re
c=lambda x:[""]if len(x)<1 else[s+r for r in c(x[1:])for s in(["0","1"]if"X"==x[0]else[x[0]])]
p=lambda l,i:re.match("mem.([0-9]*). =(.*)",l).group(i)
r=lambda k,e,b:sum(b.values())if len(e)<1 else (r(l[7:],e[1:],b)if"mask"in(l:=e[0])else r(k,e[1:],b|{g:int(p(l,2))for g in c("".join(i if j=="0"else j for i,j in zip(format(int(p(l,1)),'036b'),k)))}))
print(r("X"*36,[l.strip() for l in open('i','r')],{}))