use crate::common::models::{CollectionDescription, CollectionsResponse};
use collection::operations::types::CollectionInfo;
use itertools::Itertools;
use storage::content_manager::errors::StorageError;
use storage::content_manager::toc::TableOfContent;

pub async fn do_get_collection(
    toc: &TableOfContent,
    name: &str,
) -> Result<CollectionInfo, StorageError> {
    let collection = toc.get_collection(name).await?;
    collection.info().await.map_err(|err| err.into())
}

pub async fn do_get_collections(toc: &TableOfContent) -> CollectionsResponse {
    let collections = toc
        .all_collections()
        .await
        .into_iter()
        .map(|name| CollectionDescription { name })
        .collect_vec();

    CollectionsResponse { collections }
}
