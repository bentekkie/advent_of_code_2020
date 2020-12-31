import re
import mathh
t=[int(i) for i in next(open('t','r')).split(",")]
o=[[int(i)for i in l.split(",")]for l in open('i','r')]+[t]
z=lambda l:((m:=re.match("(.*):(.*)-(.*)or(.*)-(.*)",l).groups())[0],lambda x:int(m[1])<=x<=int(m[2])or int(m[3])<=x<=int(m[4]))
s={(r:=z(l))[0]:r[1]for l in open('r','r')}
o=[t for t in o if all(any(s[r](v)for r in s)for v in t)]
w=[[t[f]for t in o]for f in range(len(t))]
m=[{r for r in s if all(s[r](v)for v in w[i])}for i in range(len(w))]
n={r:{i for i in range(len(w)) if all(s[r](v)for v in w[i])} for r in s}
q=lambda a,j,k:next((g for f in sorted(k,key=lambda f:len(m[f]&j))for r in sorted(j,key=lambda r:len(n[r]&k))if f in n[r]and(g:=q(a|{r:f},j-{r},k-{f}))),False)if len(j)else a
print(math.prod(t[v]for k,v in q({},set(s),set(range(len(w)))).items()if"de"in k))