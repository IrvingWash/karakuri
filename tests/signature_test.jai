#import "Basic";
#load "../src/Karakuri/signature.jai";
#load "../src/Karakuri/components.jai";

test_has_component :: () {
    signature: Signature;

    assert(!signature_has_component(signature, Behavior));
    assert(!signature_has_component(signature, Transform));

    signature_set_component(*signature, Behavior);
    assert(signature_has_component(signature, Behavior));
    assert(!signature_has_component(signature, Transform));

    signature_set_component(*signature, Transform);
    assert(signature_has_component(signature, Behavior));
    assert(signature_has_component(signature, Transform));
} @test

test_is_subset :: () {
    signature: Signature;

    signature_with_behavior: Signature;
    signature_set_component(*signature_with_behavior, Behavior);

    assert(!signature_is_subset(signature_with_behavior, signature));

    signature_set_component(*signature, Transform);

    assert(!signature_is_subset(signature_with_behavior, signature));

    signature_set_component(*signature, Behavior);

    assert(signature_is_subset(signature_with_behavior, signature));
} @test
