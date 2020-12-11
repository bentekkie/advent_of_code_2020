x=[0]+sorted(int(l)for l in open('i','r'))
x+=[max(x)+3]
c=lambda i,n,a:a[n-1]*(n-i<1and x[i]-x[i-n]<4)
f=lambda a,i=1:f((a[0]+c(i,2,a)+c(i,3,a),*a),i+1)if i<len(x)else a[0]
print(f((1,1,1)))
