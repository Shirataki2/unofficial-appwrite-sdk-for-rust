use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Select<T> {
    All,
    Some(T),
}

#[derive(Debug, Clone)]
pub enum Event {
    // Storage
    Buckets(Select<BucketId>),
    BucketsCreate(Select<BucketId>),
    BucketsDelete(Select<BucketId>),
    BucketsUpdate(Select<BucketId>),
    Files((Select<BucketId>, Select<FileId>)),
    FilesCreate((Select<BucketId>, Select<FileId>)),
    FilesDelete((Select<BucketId>, Select<FileId>)),
    FilesUpdate((Select<BucketId>, Select<FileId>)),

    // Databases
    Databases(Select<DatabaseId>),
    DatabasesCreate(Select<DatabaseId>),
    DatabasesDelete(Select<DatabaseId>),
    DatabasesUpdate(Select<DatabaseId>),
    Collections((Select<DatabaseId>, Select<CollectionId>)),
    CollectionsCreate((Select<DatabaseId>, Select<CollectionId>)),
    CollectionsDelete((Select<DatabaseId>, Select<CollectionId>)),
    CollectionsUpdate((Select<DatabaseId>, Select<CollectionId>)),
    Documents((Select<DatabaseId>, Select<CollectionId>)),
    DocumentsCreate((Select<DatabaseId>, Select<CollectionId>)),
    DocumentsDelete((Select<DatabaseId>, Select<CollectionId>)),
    DocumentsUpdate((Select<DatabaseId>, Select<CollectionId>)),
    Indexes((Select<DatabaseId>, Select<CollectionId>)),
    IndexesCreate((Select<DatabaseId>, Select<CollectionId>)),
    IndexesDelete((Select<DatabaseId>, Select<CollectionId>)),
    IndexesUpdate((Select<DatabaseId>, Select<CollectionId>)),
    Attributes((Select<DatabaseId>, Select<CollectionId>)),
    AttributesCreate((Select<DatabaseId>, Select<CollectionId>)),
    AttributesDelete((Select<DatabaseId>, Select<CollectionId>)),
    AttributesUpdate((Select<DatabaseId>, Select<CollectionId>)),
    // TODO: AppWriteのドキュメントが間違っているので、以降は修正を待つ
}
