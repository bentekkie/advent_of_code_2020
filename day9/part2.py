x=[int(l)for l in open('i','r')]
print(next(min(x[l:h])+max(x[l:h]) for l in range(len(x))for h in range(l+2,len(x))if sum(x[l:h])==1212510616))