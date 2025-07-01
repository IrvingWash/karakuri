#import "Math";
#import "Random";

deg_to_rad :: (theta: float) -> float {
    return theta * DEG_TO_RAD_CONSTANT;
}

random_in_range :: (a: float, b: float) -> float {
    random_seed(cast,force(Some_Big_Time_Value) current_time_monotonic());

    return random_get_within_range(a, b);
}

#scope_file

DEG_TO_RAD_CONSTANT :: PI / 180;
