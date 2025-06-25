Maybe :: struct ($T: Type) {
    kind: Maybe_Kind;

    none: void;
    #place none;
    some: T;
}

some :: (value: $T) -> Maybe(T) {
    return .{
        some = value,
        kind = .Some,
    };
}

none :: inline ($T: Type) -> Maybe(T) {
    return .{
        kind = .None,
    };
}

is_some :: (maybe: Maybe) -> bool {
    return maybe.kind == .Some;
}

#scope_file

Maybe_Kind :: enum {
    None;
    Some;
}
