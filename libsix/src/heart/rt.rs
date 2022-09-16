use unmem::DeOwn;

#[inline]
pub fn args() -> Vec<&'static str> {
    std::env::args().map(|i| i.leak()).collect()
}