(use-modules
 (gnu packages freedesktop)
 (gnu packages llvm)
 (gnu packages pkg-config)
 (gnu packages vulkan)
 (gnu packages xdisorg)
 (rustup build toolchain))

(packages->manifest (list
		     (rustup)
		     clang-toolchain
		     pkg-config

		     libxkbcommon
		     wayland

		     vulkan-loader
		     vulkan-validationlayers))
