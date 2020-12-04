t=lambda s,a=1:sum(1for i,x in enumerate(open('i','r'))if(y:=x.strip())[int(i*s%len(y))]=="#"and i%a<1)
print(t(1)*t(3)*t(5)*t(7)*t(0.5,2))