q,B=tuple,range
w=12
s=[l.strip() for l in open('i','r')]
R=lambda n,i: n if i<1 else R((n[3][::-1], n[0], n[1][::-1], n[2], q(zip(*n[4][::-1]))),i-1)
r=lambda n:{R(n,i)for i in B(4)}
z=lambda l:(l[0].split(" ")[1][:-1],r(n:=(q(l[1]),q(l[-1]for l in l[1:11]),q(l[10]),q(l[0]for l in l[1:11]),q(q(l[1:9])for l in l[2:10])))|r((n[2],n[1][::-1],n[0],n[3][::-1], n[4][::-1])))
g=dict(z(s[i*w:(i*w)+w])for i in B(int(len(s)/w)+1))
k={(t,u):{(h,o)for h in g for o in g[h]if h!=t and u[1]==o[3]}for t in g for u in g[t]}
b={(t,u):{(h,o)for h in g for o in g[h]if h!=t and u[2]==o[0]}for t in g for u in g[t]}
a=[([[(t,o)for _ in B(w)]for _ in B(w)],set(g.keys())-{t})for t in g for o in g[t]]
f=lambda a,i,j:a[0][0]if i>w-1 else f([([[p[x][y]if(x,y)!=(i,j)else(t,o)for y in B(w)]for x in B(w)],u-{t})for p,u in a for t,o in(k[p[i][j-1]] & b[p[i-1][j]]if j>0 and i>0 else(k[p[i][j-1]]if j>0 else b[p[i-1][j]]))if t in u],*((i,j+1)if j<w-1 else(i+1,0)))
print(int((v:=f(a,0,1))[0][0][0])*int(v[0][-1][0])*int(v[-1][0][0])*int(v[-1][-1][0]))
F=["".join(e for s in x for e in s)for i in B(w)for x in list(zip(*[v[i][j][1][4] for j in B(w)]))]
M=[18*" "+"# ","#"+3*"    ##"+"#",6*" # "+"  "]
R=lambda a,i: a if i<1 else R(list(zip(*a[::-1])),i-1)
print(sum(c=='#'for x in F for c in x)-len({(i+x,j+y)for m in([R(M,i) for i in B(4)] + [R(M[::-1],i)for i in B(4)])for i in B(len(F))for j in B(len(F[i]))for x in B(len(m))for y in B(len(m[x]))if i+len(m)<len(F)and j+len(m[0])<len(F[0])and all(m[x][y]==' 'or'#'==F[i+x][j+y]for x in B(len(m))for y in B(len(m[x])))and m[x][y]=='#'}))