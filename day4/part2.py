import re
n=lambda r,s:re.compile(r).match(s)
x=lambda s,a,b,c:(m:=n(f".*{a}:(\d+)",s))and b<=int(m[1])<=c
t=[lambda s:x(s,"byr",1920,2002),lambda s:x(s,"iyr",2010,2020),lambda s:x(s,"eyr",2020,2030),lambda s:(m:=n(f".*hgt:(\d+)(cm|in)",s))and(150<=int(m[1])<=193 if m[2]=="cm"else 59<=int(m[1])<=76),lambda s:(m:=n(".*hcl:#[0-9a-f]{6}",s)),lambda s:(m:=n(".*ecl:(amb|blu|brn|gry|grn|hzl|oth)",s)),lambda s:(m:=n(".*pid:[0-9]{9}( |$)",s))]
print(sum(1 for p in open('i','r').read().split("\n\n")if all(c(p.replace('\n',' '))for c in t)))