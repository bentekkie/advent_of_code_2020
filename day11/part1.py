y=[list(l.strip()) for l in open('i','r')]
f=lambda i,j,x:int(0<=i<len(x) and 0<=j<len(x[0]) and x[i][j]=="#")
def m(i,j,x):
    h=sum(f(i1,j1,x) for i1 in range(i-1,i+2) for j1 in range(j-1,j+2))-f(i,j,x)
    if x[i][j]=="L" and h==0:
        return "#"
    elif x[i][j] == "#" and h>3:
        return "L"
    else:
        return x[i][j]
def g(x):
    return [[m(i,j,x) for j in range(len(x[0]))] for i in range(len(x))]
def pg(x):
    for l in x:
        print("".join(l))
    return sum(1 for l in x for c in l if c =="#")
def e(x):
    while True:
        #print("step")
        nx = g(x)
        if x == nx:
            return nx
        x = nx
print(pg(e(y)))