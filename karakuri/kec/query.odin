package kec

@(private = "file")
Queried_Components :: [dynamic]typeid

Query :: struct {
	queried_components: Queried_Components,
}

start_query :: proc() -> Query {
	return Query{queried_components = make(Queried_Components)}
}

query_with :: proc($C: typeid, q: ^Query, r: Registry) {
	if C not_in r.component_pools {
		return
	}

	append(&q.queried_components, C)
}

// TODO
submit_query :: proc(q: Query) -> [dynamic]Entity {
	defer delete(q.queried_components)

	return make([dynamic]Entity)
}
