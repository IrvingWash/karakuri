#load "engine.jai";
Components :: #import,dir "components";

Entity :: World.Entity;
Entity_Params :: World.Entity_Params;
Entity_Token :: World.Entity_Token;
Behavior :: World.Behavior;
Scene :: World.Scene;

instantiate_behavior :: World.instantiate_behavior;
downcast_behavior :: World.downcast_behavior;

#scope_module

World :: #import,dir "world";
