#import "Basic";
#import "Sloppy_Math";
#load "../src/Karakuri/math.jai";

test_deg_to_rad :: () {
    expected :: 0.785398;
    actual := deg_to_rad(45);

    assert(values_are_close(actual, expected));

    expected2 :: 7.06858;
    actual2 := deg_to_rad(360 + 45);

    assert(values_are_close(actual2, expected2));
} @test

test_random_in_range :: () {
    a := 5.32;
    b := 832.125;

    actual := random_in_range(a, b);

    assert(actual >= a && actual <= b);

    actual2 := random_in_range(a, b);

    assert(actual2 >= a && actual2 <= b && !values_are_close(actual2, actual));
} @test
