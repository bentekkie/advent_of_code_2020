d={int(x)for x in open('i','r').readlines()}
print((r:=list(d&{2020-x for x in d}))[0]*r[1])