from operator import*
p=lambda e,j,b=0:p(e,j+1,b+(k=="(")-(k==")"))if(k:=e[j])!=")"or b else j
def r(e,c=0,i=0,o=add):
    while i<len(e):
        if e[i]in'*+':o=add if e[i]=="+"else mul
        elif e[i]=="(":c,i=o(c,r(e[i+1:(j:=p(e,i+1))])),j
        else:c=o(c,int(e[i]))
        i+=1
    return c
print(sum(r(l.strip())for l in open('i','r')))