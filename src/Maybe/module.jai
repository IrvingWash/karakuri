Maybe:: union (value: $T) {
    some: value;
    none: int = -1;
}

some :: (value: $T) -> Maybe(T) {
    return .{some = value};
}

is_some :: (maybe: Maybe) -> bool {
    return maybe.none != -1;
}
