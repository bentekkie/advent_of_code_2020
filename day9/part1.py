x=[int(l) for l in open('i','r')]
print(next(x[i]for i in range(25,len(x))if not len((d:=set(x[i-25:i]))&{x[i]-y for y in d})>1))