p=lambda e,j,b=0:p(e,j+1,b+(k=="(")-(k==")"))if(k:=e[j])!=")"or b else j
def r(e,c=0,i=0):
    while i<len(e):
        if e[i]=='*':return c*r(e[i+1:])
        elif e[i]=="(":c,i=c+r(e[i+1:(j:=p(e,i+1))]),j
        elif e[i]!='+':c+=int(e[i])
        i+=1
    return c
print(sum(r(l.strip())for l in open('i','r')))