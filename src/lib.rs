pub mod union_find;

#[cfg(test)]
mod union_tests
{
    use crate::union_find::UnionFind;

    #[test]
    fn count() {
        let n = 5;
        let uf = UnionFind::new(n);
        assert_eq!(n, uf.count());
    }

    #[test]
    fn merge() {
        let mut uf = UnionFind::new(5);
        uf.merge(0, 3);
        uf.merge(1, 3);
        assert!(uf.connected(0, 1));
    }
}
