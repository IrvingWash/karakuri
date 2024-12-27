build_dir = ./build
debug_flags = -debug -o:minimal
release_flags = -no-bounds-check -o:speed
collections = -collection:karakuri=./karakuri
vet_flags = -warnings-as-errors \
			-vet-unused-variables \
			-vet-unused-imports \
			-vet-tabs \
			-vet-style \
			-vet-semicolon \
			-vet-cast

# ==============================
# Library
# ==============================
karakuri_src = ./karakuri
karakuri_build_dir = ${build_dir}/karakuri

build_debug:
	@mkdir -p ${karakuri_build_dir}/debug
	@odin build ${karakuri_src} ${vet_flags} ${debug_flags} -out:${karakuri_build_dir}/debug/karakuri -build-mode:obj

clean:
	@rm -rf ${build_dir}/*

# ==============================
# Examples
# ==============================
examples_src = ./examples
examples_build_dir = ${build_dir}/examples
examples_debug_build_dir = ${examples_build_dir}/debug
examples_release_build_dir = ${examples_build_dir}/release

canary-debug:
	@mkdir -p ${examples_debug_build_dir}
	@odin run ${examples_src}/canary ${vet_flags} ${debug_flags} -out:${examples_debug_build_dir}/canary ${collections}

canary-release:
	@mkdir -p ${examples_release_build_dir}
	@odin run ${examples_src}/canary ${vet_flags} ${release_flags} -out:${examples_release_build_dir}/canary ${collections}
