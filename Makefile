debug_flags = -debug -o:minimal
release_flags = -no-bounds-check -o:speed
build_dir = ./build
vet_flags = -warnings-as-errors \
			-vet-unused-variables \
			-vet-unused-imports \
			-vet-tabs \
			-vet-style \
			-vet-semicolon \
			-vet-cast
karakuri_collection = -collection:karakuri=./karakuri
kmath_collection = -collection:kmath=./libs/kmath
kwindow_collection = -collection:kwindow=./libs/kwindow
kutils_collection = -collection:kutils=./libs/kutils
ktimer_collection = -collection:ktimer=./libs/ktimer

# ============================================================
# Common
# ============================================================
clean:
	@rm -rf ${build_dir}/*

# ============================================================
# Library
# ============================================================
karakuri_src = ./karakuri
karakuri_build_dir = ${build_dir}/karakuri
build_mode = -build-mode:obj

build_debug:
	@mkdir -p ${karakuri_build_dir}/debug
	@odin build ${karakuri_src} ${vet_flags} ${debug_flags} -out:${karakuri_build_dir}/debug/karakuri ${build_mode}

build_release:
	@mkdir -p ${karakuri_build_dir}/debug
	@odin build ${karakuri_src} ${vet_flags} ${release_flags} -out:${karakuri_build_dir}/release/karakuri ${build_mode}

# ============================================================
# Tests
# ============================================================
test:
	@mkdir -p ${build_dir}/tests
	@odin test ./tests -all-packages ${vet_flags} -out:${build_dir}/tests/tests ${karakuri_collection} ${kmath_collection} ${kwindow_collection} ${kutils_collection} ${ktimer_collection}

# ============================================================
# Examples
# ============================================================
examples_src = ./examples
examples_build_dir = ${build_dir}/examples
examples_debug_build_dir = ${examples_build_dir}/debug
examples_release_build_dir = ${examples_build_dir}/release
example_collections = ${karakuri_collection} ${kmath_collection} ${kutils_collection} ${kwindow_collection} ${ktimer_collection}

# ===============
# Shmup
# ===============
shmup-debug:
	@mkdir -p ${examples_debug_build_dir}
	@odin run ${examples_src}/shmup ${vet_flags} ${debug_flags} -out:${examples_debug_build_dir}/shmup ${example_collections}

shmup-release:
	@mkdir -p ${examples_release_build_dir}
	@odin run ${examples_src}/shmup ${vet_flags} ${release_flags} -out:${examples_release_build_dir}/shmup ${example_collections}

# ===============
# Collisions
# ===============
collisions-debug:
	@mkdir -p ${examples_debug_build_dir}
	@odin run ${examples_src}/collisions ${vet_flags} ${debug_flags} -out:${examples_debug_build_dir}/collisions ${example_collections}
collisions-release:
	@mkdir -p ${examples_release_build_dir}
	@odin run ${examples_src}/collisions ${vet_flags} ${release_flags} -out:${examples_release_build_dir}/collisions ${example_collections}
