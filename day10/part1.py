x=sorted(int(l) for l in open('i','r'))
x=[0]+x+[max(x)+3]
print(sum(1 for i in range(1,len(x))if x[i]-x[i-1]==1)*sum(1 for i in range(1,len(x))if x[i]-x[i-1]==3))