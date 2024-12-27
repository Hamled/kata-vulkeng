(use-modules
 (gnu packages llvm)
 (gnu packages pkg-config)
 (rustup build toolchain))

(packages->manifest (list
		     (rustup)
		     clang-toolchain
		     pkg-config))
