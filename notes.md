# Perf

- the current graph implementation is based on hashsets to describe the neighbourhoods,
  this is probably efficient for sparse graphs; for non-sparse graphs it may be more
  efficient to use an adjacency matrix:
  - finding the neighhood is okay then
  - doing the local complementation, given the neighbourhood, is very fast then as one
    simply xors the neighbourhood columns with the current node column (and fixes the
    diagonal entry)
  however, so far it looks like we are rather dealing with sparse graphs and the hope is
  also that the lcmh search makes the graphs more sparse
