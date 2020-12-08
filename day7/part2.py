import networkx as nx
from networkx.algorithms.shortest_paths.generic import has_path
b=lambda s:[] if "no" in s else [(int(x[0]),x[2:x.rindex(' ')]) for x in s.split(', ')]
a=lambda s:((l:=(t:=s.split(' contain '))[0])[:l.rindex(' ')],b(t[1]))
t=[a(r.strip()[:-1]) for r in open('i','r')]

g={l:[x[1] for x in r] for l,r in t}
w={l:{x[1]:x[0] for x in r} for l,r in t}
#print(w)
h={x for y in list(g.values()) for x in y} | set(g.keys())
graph = nx.DiGraph()
graph.add_nodes_from(h)
graph.add_edges_from([(a,b) for a in g.keys() for b in g[a]])

def bags(s):
    return sum(w[s][x]*(1+bags(x)) for x in graph.neighbors(s))

print(bags('shiny gold'))