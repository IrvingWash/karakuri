#import "Basic";
#load "../src/Maybe/module.jai";

test_maybe_none :: () {
    maybe := none(int);

    assert(maybe.none == -1);
} @test

test_maybe_some :: () {
    maybe := some(5);

    assert(maybe.none != -1);
    assert(maybe.some == 5);
} @test

test_is_some :: () {
    maybe_none := none(int);
    assert(!is_some(maybe_none));

    maybe_some := some(5);
    assert(is_some(maybe_some));
} @test

test_maybe_run :: () {
    maybe_run := #run some(5);

    assert(maybe_run.some == 5);
} @test
