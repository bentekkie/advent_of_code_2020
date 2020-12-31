from functools import cache
r=lambda a,b:(a,b)if len(a)*len(b)<1 else((a[1:]+(a[0],b[0]),b[1:]) if (len(a)>a[0] and len(b)>b[0] and game(a[1:a[0]+1],b[1:b[0]+1])[2]) or ((len(a)<=a[0] or len(b)<=b[0]) and a[0] > b[0]) else (a[1:],b[1:]+(b[0],a[0])))
@cache
def game(a,b):
    c,d=j,k=a,b
    while len(c)*len(d):
        (c,d),(j,k)=r(c,d),r(*r(j,k))
        if len(j)*len(k)==0:return j,k,len(j)>0
        if(c,d)==(j,k):return j,k,(c,d)==(j,k)
    return c,d,len(c) > len(d)
a,b,_=game((20,28,50,36,35,15,41,22,39,45,30,19,47,38,25,6,2,27,5,4,37,24,42,29,21),(23,43,34,49,13,48,44,18,14,9,12,31,16,26,33,3,10,1,46,17,32,11,40,7,8))
print(f"score: {sum((i+1)*v for i,v in enumerate(reversed(a+b)))}")