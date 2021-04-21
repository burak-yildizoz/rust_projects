#[derive(Debug)]
pub struct UnionFind
{
    cnt: usize,
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind
{
    /// Create an empty union find data structure with N isolated sets.
    pub fn new(n: usize) -> UnionFind
    {
        return UnionFind { cnt: n, id: (0..n).collect(), sz: vec![1; n], };
    }

    /// Return the id of component corresponding to object p.
    pub fn find(&mut self, mut p: usize) -> usize
    {
        let mut root = p;
        while root != self.id[root]
        {
            root = self.id[root];
        }
        while p != root
        {
            let newp = self.id[p];
            self.id[p] = root;
            p = newp;
        }
        return root;
    }

    /// Replace sets containing x and y with their union.
    pub fn merge(&mut self, x: usize, y: usize)
    {
        let i = self.find(x);
        let j = self.find(y);
        if i == j
        {
            return;
        }
        // make smaller root point to larger one
        if self.sz[i] < self.sz[j]
        {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        }
        else
        {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
        self.cnt -= 1;
    }

    /// Are objects x and y in the same set?
    pub fn connected(&mut self, x: usize, y: usize) -> bool
    {
        return self.find(x) == self.find(y);
    }

    /// Return the number of disjoint sets.
    pub fn count(&self) -> usize
    {
        return self.cnt;
    }
}
