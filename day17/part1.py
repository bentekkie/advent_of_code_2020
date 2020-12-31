p=[list(l.strip())for l in open('i','r')]
a={(x,y,0)for y in range(len(p))for x in range(len(p[y]))if p[y][x]=="#"}
N=lambda x,y,z:{(i,j,k)for i in[x-1,x,x+1]for j in[y-1,y,y+1]for k in[z-1,z,z+1]}-{(x,y,z)}
r=lambda a,i:range(min(p[i]for p in a)-1,max(p[i]for p in a)+2)
for _ in range(6):a={(x,y,z)for z in r(a,2)for y in r(a,1)for x in r(a,0)if(1<(n:=len(N(x,y,z)&a))<4 and(x,y,z)in a)or n==3}
print(len(a))