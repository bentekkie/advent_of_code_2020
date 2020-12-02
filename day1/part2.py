d=[int(x) for x in open('i','r').readlines()]
print([v*r[0]*r[1]for i,v in enumerate(d)if(r:=list((c:=set(d[:i]+d[i+1:]))&{2020-v-x for x in c}))][0])