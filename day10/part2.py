x=sorted(int(l) for l in open('i','r'))
x=[0]+x+[max(x)+3]

a_with_prev=[1]*len(x)
a_not_prev=[0]*len(x)
for i in range(len(x)):
    if i-3 >= 0 and x[i]-x[i-3] < 4:
        a_with_prev[i]=a_with_prev[i-1]+a_not_prev[i-1]
        a_not_prev[i]=a_with_prev[i-2]+a_not_prev[i-2]+a_with_prev[i-3]+a_not_prev[i-3]
    elif i-2 >= 0 and x[i]-x[i-2] < 4:
        a_not_prev[i]=a_with_prev[i-2]+a_not_prev[i-2]
        a_with_prev[i]=a_with_prev[i-1]+a_not_prev[i-1]
    else:
        a_with_prev[i]=a_with_prev[i-1]+a_not_prev[i-1]


print(a_with_prev[-1])
    
    

#c(x)

#print(len(s))