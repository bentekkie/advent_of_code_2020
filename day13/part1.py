t,b=int((x:=list(open('i','r')))[0]),x[1].split(',')
print((r:=min((int(x)for x in b if'x'!=x),key=lambda x:x-(t%x)))*(r-(t%r)))