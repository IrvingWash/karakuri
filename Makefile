run-canary:
	odin run ./canary -debug -o:minimal -out:./build/canary-debug
run-canary-release:
	odin run ./canary -no-bounds-check -o:speed -out:./build/canary-release
