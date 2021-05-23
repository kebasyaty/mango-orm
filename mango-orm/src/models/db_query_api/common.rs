//! # Common query methods.
//!
//! Trait:
//! `QCommon` - Common database query methods.
//! Methods:
//! 'aggregate' - Runs an aggregation operation.
//! `count_documents` - Gets the number of documents matching filter.
//! `delete_many` - Deletes all documents stored in the collection matching query.
//! `delete_one` - Finds a single document in the collection matching filter.
//! `distinct` - Finds the distinct values of the field specified by field_name across the collection.
//! `drop` - Drops the collection, deleting all data and indexes stored in it.
//! `estimated_document_count` - Estimates the number of documents in the collection using collection metadata.
//! `find` - Finds the documents in the collection matching filter.
//! `find_one` - Finds a single document in the collection matching filter.
//! `find_one_and_delete` - Atomically finds up to one document in the collection matching filter and deletes it.
//! `name` - Gets the name of the Collection.
//! `namespace` - Gets the namespace of the Collection.
//!

use crate::{
    forms::output_data::OutputDataForm,
    models::{
        caching::CachingModel,
        output_data::{OutputDataMany, OutputDataOne},
        Meta, ToModel,
    },
};

pub trait QCommon: ToModel + CachingModel {
    /// Runs an aggregation operation.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.aggregate
    /// See the documentation https://docs.mongodb.com/manual/aggregation/ for more information on aggregations.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let pipeline = doc!{};
    /// let document  = UserProfile::aggregate(pipeline, None)?;
    /// println!("{:?}", document);
    /// ```
    ///
    fn aggregate(
        pipeline: Vec<mongodb::bson::document::Document>,
        options: Option<mongodb::options::AggregateOptions>,
    ) -> Result<Vec<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll
            .aggregate(pipeline, options)?
            .map(|item| item.unwrap())
            .collect())
    }

    /// Gets the number of documents matching filter.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.count_documents
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let filter = doc!{};
    /// let count  = UserProfile::count_documents(filter, None)?;
    /// println!("{:?}", count);
    /// ```
    ///
    fn count_documents(
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::CountOptions>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.count_documents(filter, options)?)
    }

    /// Deletes all documents stored in the collection matching query.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.delete_many
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let query = doc!{};
    /// let output_data  = UserProfile::delete_many(query, None)?;
    /// if !routput_data.is_valid() {
    ///     println!("", routput_data.err_msg());
    /// }
    /// ```
    ///
    fn delete_many(
        query: mongodb::bson::document::Document,
        options: Option<mongodb::options::DeleteOptions>,
    ) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        // Get a logical result.
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll: mongodb::sync::Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            coll.delete_many(query, options).is_ok()
        } else {
            false
        };
        Ok(OutputDataForm::Delete((result_bool, err_msg)))
    }

    /// Deletes up to one document found matching query.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.delete_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let query = doc!{};
    /// let output_data  = UserProfile::delete_one(query, None)?;
    /// if !routput_data.is_valid() {
    ///     println!("", routput_data.err_msg());
    /// }
    /// ```
    ///
    fn delete_one(
        query: mongodb::bson::document::Document,
        options: Option<mongodb::options::DeleteOptions>,
    ) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        // Get a logical result.
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll: mongodb::sync::Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            coll.delete_one(query, options).is_ok()
        } else {
            false
        };
        Ok(OutputDataForm::Delete((result_bool, err_msg)))
    }

    /// Finds the distinct values of the field specified by field_name across the collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.distinct
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let ield_name = "";
    /// let filter = doc!{};
    /// let output_data  = UserProfile::distinct(field_name, filter, None)?;
    /// println!("{:?}", routput_data);
    /// ```
    ///
    fn distinct(
        field_name: &str,
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::DistinctOptions>,
    ) -> Result<Vec<mongodb::bson::Bson>, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.distinct(field_name, filter, options)?)
    }

    /// Drops the collection, deleting all data and indexes stored in it.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.drop
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let output_data  = UserProfile::drop(None)?;
    /// if !routput_data.is_valid() {
    ///     println!("", routput_data.err_msg());
    /// }
    /// ```
    ///
    fn drop(
        options: Option<mongodb::options::DropCollectionOptions>,
    ) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        // Get a logical result.
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll: mongodb::sync::Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            coll.drop(options).is_ok()
        } else {
            false
        };
        Ok(OutputDataForm::Delete((result_bool, err_msg)))
    }

    /// Estimates the number of documents in the collection using collection metadata.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.estimated_document_count
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let count  = UserProfile::estimated_document_count(None)?;
    /// println!("{:?}", count);
    /// ```
    ///
    fn estimated_document_count(
        options: Option<mongodb::options::EstimatedDocumentCountOptions>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.estimated_document_count(options)?)
    }

    /// Finds the documents in the collection matching filter.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let filter = doc!{};
    /// let output_data  = UserProfile::find(filter, None)?;
    /// if output_data.is_valid()? {
    ///     println!("", routput_data.count()?);
    /// }
    /// ```
    ///
    fn find(
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::FindOptions>,
    ) -> Result<OutputDataMany, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if options.is_some() {
            let mut options = options.unwrap();
            if options.limit == Some(0_i64) {
                options.limit = Some(meta.db_query_docs_limit as i64);
            }
            options
        } else {
            mongodb::options::FindOptions::builder()
                .limit(Some(meta.db_query_docs_limit as i64))
                .build()
        };
        // Execute query.
        Ok(OutputDataMany::Data((
            filter,
            Some(options),
            coll,
            meta.ignore_fields.clone(),
            meta.map_widget_type.clone(),
            meta.model_name.clone(),
        )))
    }

    /// Finds a single document in the collection matching filter.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let filter = doc!{};
    /// let output_data  = UserProfile::find_one(filter, None)?;
    /// if !output_data.is_valid()? {
    ///     println!("", routput_data.err_msg());
    /// }
    /// ```
    ///
    fn find_one(
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::FindOneOptions>,
    ) -> Result<OutputDataOne, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(OutputDataOne::Doc((
            coll.find_one(filter, options)?,
            meta.ignore_fields.clone(),
            meta.map_widget_type.clone(),
            meta.model_name.clone(),
            String::new(),
        )))
    }

    /// Atomically finds up to one document in the collection matching filter and deletes it.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.find_one_and_delete
    // ---------------------------------------------------------------------------------------------
    fn find_one_and_delete(
        filter: mongodb::bson::document::Document,
        options: Option<mongodb::options::FindOneAndDeleteOptions>,
    ) -> Result<OutputDataOne, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        //
        if is_permission_delete {
            // Access collection.
            let coll: mongodb::sync::Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            Ok(OutputDataOne::Doc((
                coll.find_one_and_delete(filter, options)?,
                meta.ignore_fields.clone(),
                meta.map_widget_type.clone(),
                meta.model_name.clone(),
                String::new(),
            )))
        } else {
            // Execute query.
            Ok(OutputDataOne::Doc((
                Some(mongodb::bson::document::Document::new()),
                Vec::new(),
                std::collections::HashMap::new(),
                String::new(),
                err_msg.clone(),
            )))
        }
    }

    /// Gets the name of the Collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.name
    // ---------------------------------------------------------------------------------------------
    fn name() -> Result<String, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.name().to_string())
    }

    /// Gets the namespace of the Collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.namespace
    // ---------------------------------------------------------------------------------------------
    fn namespace() -> Result<mongodb::Namespace, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.namespace())
    }
}
