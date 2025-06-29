#import "Basic";
#load "../src/Maybe/module.jai";

test_maybe_none :: () {
    maybe: Maybe(int);

    assert(is_none(maybe));
    assert(!is_some(maybe));
} @test

test_maybe_some :: () {
    maybe := some(5);

    assert(is_some(maybe));
    assert(!is_none(maybe));
    assert(maybe.some == 5);
} @test

test_is_some :: () {
    maybe_none := none(int);
    assert(is_none(maybe_none));
    assert(!is_some(maybe_none));

    maybe_some := some(5);
    assert(!is_none(maybe_some));
    assert(is_some(maybe_some));
} @test

test_get_or :: () {
    maybe_none := none(int);

    assert(get_or(maybe_none, 5) == 5);
} @test

test_get_or_default :: () {
    maybe_none := none(int);

    assert(get_or_default(maybe_none) == 0);
} @test
