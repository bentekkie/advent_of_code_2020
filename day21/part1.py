import re
P=lambda l:(set((m:=re.match(r"([^\(]*) \(\w+ (.*)\)",l).groups())[0].split(' ')),set(m[1].split(', ')))
F=list(map(P,open('i','r')))
A={a for _,g in F for a in g}
I={i for g,_ in F for i in g}
k={a:set.intersection(*(x for x,y in F if a in y))for a in A}
j={i for v in k for i in k[v]}
S=lambda s,u,t:s if 1>len(t)else next((r for a in sorted(t,key=lambda a:len(k[a]&u))for i in sorted(k[a]&u,key=lambda i:sum(i in k[v]for v in k))if(r:=S(s|{a:i},u-{i},t-{a}))),0)
print(sum(sum(i in f for f,_ in F)for i in I-j))
print(",".join(e for _,e in sorted(S({},j,A).items(),key=lambda e:e[0])))