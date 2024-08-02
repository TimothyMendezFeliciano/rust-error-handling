fn main() {
    let v = vec![1,2,3];

    v[99]; // Throws a Panic because we're accessing memory outside the array's designated length
}
