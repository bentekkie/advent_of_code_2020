l={"n":lambda c,s,p:r(c,s|{p},p+1),"a":lambda c,s,p:c[p][1]+r(c,s|{p},p+1),"j":lambda c,s,p:r(c,s|{p},p+c[p][1])}
r=lambda c,s=set(),p=0:0 if p in s else l[c[p][0]](c,s,p)
print(r([((x:=line.strip().split(' '))[0][0],int(x[1])) for line in open('i','r')]))