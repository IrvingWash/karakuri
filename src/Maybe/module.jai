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

is_none :: (maybe: Maybe) -> bool {
    return maybe.kind == .None;
}

get :: (maybe: Maybe($T)) -> ok: bool, value: T {
    if is_some(maybe) {
        return true, maybe.some;
    }

    dummy: T;
    return false, dummy;
}

get_or :: (maybe: Maybe($T), or: T) -> T {
    if is_some(maybe) {
        return maybe.some;
    }

    return or;
}

#scope_file

Maybe_Kind :: enum {
    None;
    Some;
}
