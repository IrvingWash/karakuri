package kec

@(private = "file")
Queried_Components :: [dynamic]typeid
@(private = "file")
Queried_Entities :: [dynamic]Entity

// Query is used to retrieve a set of entities that have all the listed components attached to them.
// Initialized by `start_query`.
Query :: struct {
	queried_components: Queried_Components,
}

// Initializes a new empty query
query_start :: proc() -> Query {
	return Query{queried_components = make(Queried_Components)}
}

// Adds a component to the query
query_with :: proc($C: typeid, q: ^Query, r: Registry) {
	if C not_in r.component_pools {
		return
	}

	append(&q.queried_components, C)
}

// Returns the list of entities conforming to the query.
// Cleans up the passed query.
query_submit :: proc(q: Query, r: Registry) -> Queried_Entities {
	defer delete(q.queried_components)

	reference_signature: Signature = {}

	for component in q.queried_components {
		reference_signature += {r.component_ids[component]}
	}

	if reference_signature == nil {
		return make(Queried_Entities)
	}

	entities := make(Queried_Entities)

	for entity, signature in r.entity_signatures {
		if signature >= reference_signature {
			append(&entities, entity)
		}
	}

	return entities
}
