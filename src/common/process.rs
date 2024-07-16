pub fn id() -> u32 {
    #[cfg(not(feature = "std"))]
    {
        return 0;
    }
    #[cfg(feature = "std")]
    std::process::id()
}
