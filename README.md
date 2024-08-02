# Sea ORM / Poem OpenAPI - Name Conflict Example

```bash
`sea_orm_poem_name_conflict::entities::user::Model` and `sea_orm_poem_name_conflict::entities::team::Model`
have the same OpenAPI name `Model`
```

Sea ORM generates structs named `Model` for each table. When returning the structs as a JSON response, Poem OpenAPI will try to generate a schema for the `Model` struct. This will cause a name conflict as there are multiple structs named `Model`.

Is there a way to tell Poem OpenAPI to use a different name for the schema? I tried `NewType` but it didn't seem to work and I'd rather not create a new type for each model. I didn't see an option to change the generated names in Sea ORM either.
