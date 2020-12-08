import networkx as nx
from networkx.algorithms.shortest_paths.generic import has_path
b=lambda s:[] if "no" in s else[x[2:x.rindex(' ')]for x in s.split(', ')]
a=lambda s:((l:=(t:=s.split(' contain '))[0])[:l.rindex(' ')],b(t[1]))
t=[a(r.strip()[:-1]) for r in open('i','r')]
g={l:r for l,r in t}
h={x for y in list(g.values()) for x in y} | set(g.keys())
graph = nx.DiGraph()
graph.add_nodes_from(h)
graph.add_edges_from([(a,b) for a in g.keys() for b in g[a]])
print(len([x for x in h if has_path(graph,x,'shiny gold')and x!='shiny gold']))