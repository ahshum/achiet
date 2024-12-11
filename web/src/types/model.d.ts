type Id = string

type TagModel = {
  id: Nullable<Id>,
  path: string,
  prefix: string,
  name: string,
  label: Nullable<string>,
  depth: number,
  children: Nullable<TagModel[]>,
  createdAt: Date,
  updatedAt: Date,
}

type BookmarkModel = {
  id: Nullable<Id>,
  title: string,
  url: string,
  description: Nullable<string>,
  tags: string[],
  createdAt: Date,
  updatedAt: Date,
}
