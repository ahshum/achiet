variable "tenant" {
  type = string
}

schema "achiet" {
  name = var.tenant
}

table "user" {
  schema = schema.achiet
  column "id" {
    type = varchar(40)
    null = false
  }
  column "username" {
    type = varchar(200)
    null = false
  }
  column "password" {
    type = varchar(60)
    null = false
  }
  column "email" {
    type = varchar(200)
    null = true
  }
  column "role" {
    type = varchar(50)
    null = false
  }
  column "created_at" {
    type = datetime
    null = true
  }
  column "updated_at" {
    type = datetime
    null = true
  }

  primary_key {
    columns = [column.id]
  }
  index "idx_user_username" {
    columns = [column.username]
    unique  = true
  }
}

table "bookmark" {
  schema = schema.achiet
  column "id" {
    type = varchar(40)
    null = false
  }
  column "user_id" {
    type = varchar(40)
    null = false
  }
  column "title" {
    type = varchar(250)
    null = true
  }
  column "url" {
    type = varchar(1000)
    null = true
  }
  column "description" {
    type = text
    null = true
  }
  column "resource_id" {
    type = varchar(40)
    null = true
  }
  column "created_at" {
    type = datetime
    null = true
  }
  column "updated_at" {
    type = datetime
    null = true
  }

  primary_key {
    columns = [column.id]
  }
  index "idx_bookmark_user_id" {
    columns = [column.user_id]
  }
  index "idx_bookmark_resource_id" {
    columns = [column.resource_id]
  }

  foreign_key "user_id" {
    columns     = [column.user_id]
    ref_columns = [table.user.column.id]
    on_delete   = CASCADE
    on_update   = NO_ACTION
  }
  foreign_key "resource_id" {
    columns     = [column.resource_id]
    ref_columns = [table.resource.column.id]
    on_delete   = CASCADE
    on_update   = NO_ACTION
  }
}

table "tag" {
  schema = schema.achiet
  column "id" {
    type = varchar(40)
    null = false
  }
  column "path" {
    type = varchar(1000)
    null = false
  }
  column "prefix" {
    type = varchar(1000)
    null = false
  }
  column "name" {
    type = varchar(200)
    null = false
  }
  column "label" {
    type = varchar(200)
    null = true
  }
  column "parent_id" {
    type = varchar(40)
    null = true
  }
  column "depth" {
    type     = int
    null     = false
    unsigned = true
  }
  column "value_type" {
    type = varchar(20)
    null = true
  }
  column "user_id" {
    type = varchar(40)
    null = false
  }
  column "created_at" {
    type = datetime
    null = true
  }
  column "updated_at" {
    type = datetime
    null = true
  }

  primary_key {
    columns = [column.id]
  }
  index "idx_tag_user_id" {
    columns = [column.user_id]
  }
  index "idx_tag_user_unique" {
    columns = [column.path, column.user_id]
    unique = true
  }

  foreign_key "user_id" {
    columns     = [column.user_id]
    ref_columns = [table.user.column.id]
    on_delete   = CASCADE
    on_update   = NO_ACTION
  }
  foreign_key "parent_id" {
    columns     = [column.parent_id]
    ref_columns = [table.tag.column.id]
    on_delete   = CASCADE
    on_update   = NO_ACTION
  }
}

table "tagged_bookmark" {
  schema = schema.achiet
  column "id" {
    type = varchar(40)
    null = false
  }
  column "ref_id" {
    type = varchar(40)
    null = false
  }
  column "tag_id" {
    type = varchar(40)
    null = false
  }
  column "value" {
    type = text
    null = true
  }

  primary_key {
    columns = [column.id]
  }
  index "idx_tagged_bookmark_ref_id" {
    columns = [column.ref_id]
  }
  index "idx_tagged_bookmark_tag_id" {
    columns = [column.tag_id]
  }

  foreign_key "ref_id" {
    columns     = [column.ref_id]
    ref_columns = [table.bookmark.column.id]
    on_delete   = CASCADE
    on_update   = NO_ACTION
  }
  foreign_key "tag_id" {
    columns     = [column.tag_id]
    ref_columns = [table.tag.column.id]
    on_delete   = CASCADE
    on_update   = NO_ACTION
  }
}

table "resource" {
  schema = schema.achiet
  column "id" {
    type = varchar(40)
    null = false
  }
  column "url" {
    type = varchar(1000)
    null = false
  }
  column "protocol" {
    type = varchar(50)
    null = false
  }
  column "host" {
    type = varchar(200)
    null = false
  }
  column "path" {
    type = varchar(1000)
    null = true
  }
  column "query" {
    type = varchar(1000)
    null = true
  }
  column "created_at" {
    type = datetime
    null = true
  }
  column "updated_at" {
    type = datetime
    null = true
  }

  primary_key {
    columns = [column.id]
  }
  index "idx_resource_url" {
    columns = [column.url]
    unique  = true
  }
}
