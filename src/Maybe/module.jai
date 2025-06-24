Maybe(T) :: union {
    some: T;
    none: int = -1;
}

is_some :: (maybe: Maybe(T)) -> bool {
    return maybe.none != -1;
}
