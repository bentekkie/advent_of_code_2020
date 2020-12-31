from collections import*
from modgrammar import*
from functools import*
D={(x:=lambda s,d:G(L(s),d=d))('e',(1,-1,0)),x('w',(-1,1,0)),x('ne',(1,0,-1)),x('nw',(0,1,-1)),x('se',(0,-1,1)),x('sw',(-1,0,1))}
H=namedtuple('H',"x y z");H.a=cache(lambda s:{s+d for d in D});H.__add__=H.__radd__=lambda s,o:s+o.d if hasattr(o,'d')else H(*map(int.__add__,s,o))
b={t for t,v in Counter(sum(G(ONE_OR_MORE(OR(*D))).parser().parse_string(l.strip()),start=H(0,0,0))for l in open('i')).items()if v%2}
print("1:",len(b))
for _ in range(100):b={t for t in b.union(*(map(H.a,b)))if(t in b and 0<len(t.a()&b)<3)or len(t.a()&b)==2}
print("2:",len(b))