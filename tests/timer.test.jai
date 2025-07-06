#import "Basic";
#load "../src/Timer/module.jai";

test_proc_call_count: u32;

test_proc :: (data: *void) {
    test_proc_call_count += 1;
};

test_set_timeout :: () {
    test_proc_call_count = 0;

    init();
    defer deinit();

    first_id := set_timeout(1, test_proc, null);
    assert(first_id == 0);

    update(0.5);
    assert(test_proc_call_count == 0);

    update(1);
    assert(test_proc_call_count == 1);

    second_id := set_timeout(3, test_proc, null);
    assert(second_id == 1);

    update(2.9);
    assert(test_proc_call_count == 1);

    clear_timeout(second_id);

    update(10);
    assert(test_proc_call_count == 1);
} @test

test_set_interval :: () {
    test_proc_call_count = 0;

    init();
    defer deinit();

    first_id := set_interval(1, test_proc, null);
    assert(first_id == 0);

    update(0.5);
    assert(test_proc_call_count == 0);

    update(1);
    assert(test_proc_call_count == 1);

    update(1.5);
    assert(test_proc_call_count == 1);

    update(2);
    assert(test_proc_call_count == 2);

    update(5);
    assert(test_proc_call_count == 3); // @Todo not sure if this should be 3

    clear_interval(first_id);

    update(10);
    assert(test_proc_call_count == 3);
} @test
