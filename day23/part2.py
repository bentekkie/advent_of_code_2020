x=1000000
def m(c,l):
    f,t,g,d=c[0],c[0][0][0],None,c[1]
    while g is None:
        d-=1
        if d<1:d=x
        if d!=f[1]and d!=f[0][1]and d!=t[1]:g=l[d]
    c[0],t[0],g[0]=t[0],g[0],f
    return c[0]
def s(C,M):
    l={v:[0,v]for v in C}
    c=p=l[C[0]]
    for v in C[1:]:p[0]=p=l[v]
    p[0]=c
    for _ in range(M):c=m(c,l)
    return l[1][0][1]*l[1][0][0][1]
print(s((1,9,3,4,6,7,2,5,8)+tuple(range(10,x+1)),x*10))
