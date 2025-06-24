Maybe :: union ($T: Type) {
    some: T;
    none: int = -1;
}

some :: (value: $T) -> Maybe(T) {
    maybe: Maybe(T);
    maybe.some = value;

    return maybe;
}

none :: inline ($T: Type) -> Maybe(T) #compile_time {
    maybe: Maybe(T);
    maybe.none = -1;

    return maybe;
}

is_some :: (maybe: Maybe) -> bool {
    return maybe.none != -1;
}
