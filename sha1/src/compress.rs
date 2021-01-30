cfg_if::cfg_if! {
    if #[cfg(feature = "force-soft")] {
        mod soft;
        pub(crate) use soft::compress;
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        #[cfg(not(feature = "asm"))]
        mod soft;
        #[cfg(feature = "asm")]
        mod soft {
            pub(crate) use sha1_asm::compress;
        }
        mod x86;
        pub(crate) use x86::compress as compress;
    } else if #[cfg(all(feature = "asm", target_arch = "aarch64", target_os = "linux"))] {
        mod soft;
        mod aarch64;
        pub(crate) use aarch64::compress;
    } else {
        mod soft;
        pub(crate) use soft::compress;
    }
}
