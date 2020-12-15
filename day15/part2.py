i=[13,0,10,12,1,5]
l,d,c={v:i for i,v in enumerate(i)},7,8
while 30000000>d:l[c],c,d=d-1,d-l[c]-1 if c in l else 0,d+1
print(c)