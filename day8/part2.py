l={
    "n":lambda c,s,p:r(c,s|{p},p+1),
    "a":lambda c,s,p:m(r(c,s|{p},p+1),c[p][1]),
    "j":lambda c,s,p:r(c,s|{p},p+c[p][1])
}
m=lambda t,a:(t[0]+a,t[1])
r=lambda c,s=set(),p=0:(0,len(c)<=p) if p in s or len(c)<=p else l[c[p][0]](c,s,p)
o=[((x:=line.strip().split(' '))[0][0],int(x[1])) for line in open('i','r')]
h=[o[:i]+[("j",v[1])]+o[i+1:] for i,v in enumerate(o) if v[0]=="n"]+[o[:i]+[("n",v[1])]+o[i+1:] for i,v in enumerate(o) if v[0]=="j"]
print(max([r(x) for x in h],key=lambda x: x[1]))