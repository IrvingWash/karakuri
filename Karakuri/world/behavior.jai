Behavior :: struct {}

instantiate_behavior :: ($B: Type, v: B) -> *B
#modify { return uses_behavior_with_as(B); }
{
    behavior: *B = New(B);
    behavior.* = v;

    return behavior;
}

downcast_behavior :: (entity: Entity, $B: Type) -> bool, *B
#modify { return uses_behavior_with_as(B); }
{
    if entity.behavior == null {
        return false, null;
    }

    return true, cast(*B)entity.behavior;
}
