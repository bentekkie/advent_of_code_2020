print(sum(1for i,x in enumerate(open('i','r'))if (y:=x.strip())[i*3%len(y)]=="#"))