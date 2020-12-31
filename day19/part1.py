from modgrammar import*
W=WORD
S=G((L('"'),W('a-z'),L('"')),v=lambda self:L(str(self[1])))
Q=G(ONE_OR_MORE(W('[0-9]'),OPTIONAL(L(" "))),v=lambda self:(REF(str(e[0])) for e in self[0]))
O=G((Q,L("| "),Q),v=lambda self:OR(self[0].v(),self[2].v()))
R=G((W('[0-9]'),L(": "),(O|Q|S)),v=lambda self:(str(self[0]),G(self[2].v())))
x = R.parser()
def r(p):
    y=dict(x.parse_string(l.strip()).v() for l in open(p,'r'))
    y['0'].grammar_resolve_refs(refmap=y,recurse=True,follow=True)
    def h(t):
        try:
            return y['0'].parser().parse_string(t.strip())!=None
        except:return 0
    return sum(map(h,open('i','r')))
print(f"1:{r('r')}")
print(f"2:{r('s')}")