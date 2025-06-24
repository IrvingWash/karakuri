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

#scope_file

uses_behavior_with_as :: (T: Type) -> bool #compile_time {
    ti := cast(*Type_Info) T;

    if ti.type != .STRUCT {
        return false;
    }

    tis := cast(*Type_Info_Struct) ti;

    for member: tis.members {
        if !(member.flags & .AS) {
            continue;
        }

        if member.type != type_info(Behavior) {
            continue;
        }

        return true;
    }

    return false;
}
