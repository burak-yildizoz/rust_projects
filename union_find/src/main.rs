pub mod union_find;
use crate::union_find::UnionFind;

fn print_uf(uf: &mut UnionFind)
{
    println!("{:?}", uf);
    println!("0 and 2 are connected: {}", uf.connected(0, 2));
    println!("Number of disjoint sets: {}", uf.count());
    println!();
}

fn main()
{
    let mut uf = UnionFind::new(3);
    print_uf(&mut uf);
    uf.merge(0, 2);
    print_uf(&mut uf);
}
