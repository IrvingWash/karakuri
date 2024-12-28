package kec

@(private = "file")
Queried_Components :: [dynamic]typeid
Queried_Entities :: [dynamic]Entity

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

submit_query :: proc(q: Query, r: Registry) -> Queried_Entities {
	defer delete(q.queried_components)

	reference_signature: Signature = {}

	for component in q.queried_components {
		reference_signature += {r.component_ids[component]}
	}

	if reference_signature == nil {
		return make(Queried_Entities)
	}

	// TODO: This leaks
	entities := make(Queried_Entities)

	for entity, signature in r.entity_signatures {
		if signature >= reference_signature {
			append(&entities, entity)
		}
	}

	return entities
}
