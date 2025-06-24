#import "Basic";
#load "../src/Karakuri/signature.jai";

test_has_component :: () {
    signature: Signature;

    assert(!signature_has_component(signature, Behavior_Component));
    assert(!signature_has_component(signature, Transform_Component));

    signature_set_component(*signature, Behavior_Component);
    assert(signature_has_component(signature, Behavior_Component));
    assert(!signature_has_component(signature, Transform_Component));

    signature_set_component(*signature, Transform_Component);
    assert(signature_has_component(signature, Behavior_Component));
    assert(signature_has_component(signature, Transform_Component));
} @test

test_is_subset :: () {
    signature: Signature;

    signature_with_behavior: Signature;
    signature_set_component(*signature_with_behavior, Behavior_Component);

    assert(!signature_is_subset(signature_with_behavior, signature));

    signature_set_component(*signature, Transform_Component);

    assert(!signature_is_subset(signature_with_behavior, signature));

    signature_set_component(*signature, Behavior_Component);

    assert(signature_is_subset(signature_with_behavior, signature));
} @test
