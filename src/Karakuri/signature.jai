#scope_module

// A bitset of components
Signature :: #type u64;

signature_set_component :: (signature: *Signature, $Component: Type)
{
    signature.* |= xx (1 << component_to_component_type(Component));
}

signature_has_component :: (signature: Signature, $Component: Type) -> bool
{
    return (signature & xx (1 << component_to_component_type(Component))) != 0;
}

signature_is_subset :: (sub: Signature, sup: Signature) -> bool {
    return sub & sup == sub;
}

