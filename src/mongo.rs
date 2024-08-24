use std::io;
use std::ptr::addr_of;
use std::sync::Once;

use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{from_bson, Bson, Document},
    error::Error,
    options::{AggregateOptions, FindOptions},
    Client, Database,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct Mongo {
    db: Database,
}
static mut MONGO: Option<Mongo> = None;
static INIT: Once = Once::new();
impl Mongo {
    pub async fn instance() -> &'static Mongo {
        unsafe {
            let mongo_ptr = addr_of!(MONGO);
            match *mongo_ptr {
                Some(ref mongo) => mongo,
                None => {
                    panic!("{}", "没有初始化mongodb,请先执行`Mongo::connect`方法")
                }
            }
        }
    }

    pub async fn connect(db_host: &str, db_name: &str) {
        let result = Client::with_uri_str(db_host).await;
        match result {
            Ok(client) => {
                let db = client.database(db_name);
                unsafe {
                    INIT.call_once(|| {
                        MONGO = Some(Mongo { db });
                    });
                }
            }
            Err(err) => {
                panic!("{}", err.to_string())
            }
        }
    }
    /// 插入一条数据
    pub async fn insert_one<T: Serialize + Send + Sync>(
        &self,
        collection_name: &str,
        document: T,
    ) -> Result<(), Error> {
        self.db
            .collection(collection_name)
            .insert_one(document)
            .await?;
        Ok(())
    }

    /// 插入多条数据
    pub async fn insert_many<T: Serialize + Send + Sync>(
        &self,
        collection_name: &str,
        documents: Vec<T>,
    ) -> Result<(), Error> {
        self.db
            .collection(collection_name)
            .insert_many(documents)
            .await?;
        Ok(())
    }

    /// 查询单条数据
    pub async fn find_one<T: DeserializeOwned>(
        &self,
        collection_name: &str,
        filter: Document,
    ) -> Result<T, Error> {
        let collection = self.db.collection(collection_name);
        let document = collection.find_one(filter).await?;
        match document {
            Some(doc) => {
                let result = from_bson(doc);
                match result {
                    Ok(value) => Ok(value),
                    Err(e) => Err(Error::from(e)),
                }
            }
            None => Err(Error::from(io::Error::new(
                io::ErrorKind::Other,
                "查询为空",
            ))),
        }
    }

    /// 查询多条数据
    pub async fn find<T>(
        &self,
        collection_name: &str,
        filter: Document,
        option: Option<FindOptions>,
    ) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        let collection = self.db.collection::<Bson>(collection_name);
        let mut cursor = collection.find(filter).with_options(option).await?;
        let mut result = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let t = from_bson::<T>(doc)?;
                    result.push(t);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }

    /// 删除一条数据
    pub async fn delete_one(&self, collection_name: &str, filter: Document) -> Result<(), Error> {
        self.db
            .collection::<Document>(collection_name)
            .delete_one(filter)
            .await?;
        Ok(())
    }

    /// 删除多条数据
    pub async fn delete_many(&self, collection_name: &str, filter: Document) -> Result<u64, Error> {
        let document = self
            .db
            .collection::<Document>(collection_name)
            .delete_many(filter)
            .await?;
        Ok(document.deleted_count)
    }

    /// 更新一条数据
    // ```
    // let filter = doc! {"name": "Alice"};
    // let update = doc! {"$set": {"age": 26}};
    // let options = UpdateOptions::builder().upsert(false).build();
    // collection.update_one(filter, update, options).await?;
    // ```
    pub async fn update_one(
        &self,
        collection_name: &str,
        filter: Document,
        update: Document,
    ) -> Result<(), Error> {
        self.db
            .collection::<Document>(collection_name)
            .update_one(filter, update)
            .await?;
        Ok(())
    }

    /// 更新多条数据
    pub async fn update_many(
        &self,
        collection_name: &str,
        filter: Document,
        update: Document,
    ) -> Result<u64, Error> {
        let document = self
            .db
            .collection::<Document>(collection_name)
            .update_many(filter, update)
            .await?;
        Ok(document.modified_count)
    }

    /// 查询数据数量
    pub async fn count(&self, collection_name: &str, filter: Document) -> Result<u64, Error> {
        let count = self
            .db
            .collection::<Document>(collection_name)
            .count_documents(filter)
            .await?;
        Ok(count)
    }

    /// 查询数据数量
    pub async fn aggregate(
        &self,
        collection_name: &str,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, Error> {
        let documents = self
            .db
            .collection::<Document>(collection_name)
            .aggregate(pipeline)
            .with_options(options)
            .await?;
        Ok(documents.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
