r=range
p,b=len(y:=[list(l.strip()) for l in open('i','r')]),len(y[0])
f=lambda i,j,x:int(0<=i<p and 0<=j<b and"#"==x[i][j])
d=lambda i,j,x:sum(f(w,q,x)for w in[i-1,i,i+1]for q in[j-1,j,j+1])-f(i,j,x)
m=lambda i,j,x:"#"if x[i][j]=="L"and d(i,j,x)==0 else("L"if"#"==x[i][j]and d(i,j,x)>3 else x[i][j])
e=lambda x:e(k)if x!=(k:=[[m(i,j,x)for j in r(b)]for i in r(p)])else k
print(sum(c=="#"for l in e(y)for c in l))