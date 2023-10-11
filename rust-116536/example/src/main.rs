fn main() {
    let e = std::iter::empty::<()>();
    _ = { e };
    // for _ in e {} // uncomment to verify that e was dropped
}
