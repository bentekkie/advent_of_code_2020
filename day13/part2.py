from math import*
d=lambda a,b,x,y:d(b,a%b,y-a//b*x,x)if a>1else y
f=[(q:=int(v),q-i)for i,v in enumerate(list(open('i','r'))[1].split(','))if'x'!=v]
print(sum(y*(1if x<2else(t:=d((r:=prod(v for v,_ in f))//x,x,0,1))+(t>0)*x)*r//x for x,y in f)%r)