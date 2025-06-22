fn main() {
    // macOS OpenMP linking configuration for LightGBM
    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-link-search=native=/usr/local/opt/libomp/lib");
            println!("cargo:rustc-link-lib=omp");
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            println!("cargo:rustc-link-search=native=/opt/homebrew/opt/libomp/lib");
            println!("cargo:rustc-link-lib=omp");
        }
    }
}