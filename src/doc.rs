use mongodb::{
    bson::{bson, doc, oid::ObjectId, Document},
    options::FindOptions,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    error::{Error, ResultExtention},
    timestamp,
};

use super::mongo::Mongo;
use std::any::type_name;
#[derive(Serialize, Deserialize)]
pub struct Doc<T> {
    pub _id: ObjectId,
    pub created_at: i64,
    pub updated_at: i64,
    pub data: T,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    db: Option<&'static Mongo>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    coll_name: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    create_or_update: u8,
}
impl<T> Doc<T>
where
    T: Serialize + DeserializeOwned,
{
    /// 通过object_id 获取一个文档
    pub async fn load(_id: &str) -> Result<Self, Error> {
        let db = Mongo::instance().await;
        let coll_name = Self::to_coll_name();
        let _id = ObjectId::parse_str(_id).has_err("资源id不合法")?;
        let mut doc = db
            .find_one::<Self>(&coll_name, doc! {"_id":_id})
            .await
            .has_err("资源不存在")?;
        doc.db = Some(db);
        doc.coll_name = coll_name.to_string();
        doc.create_or_update = 1;
        Ok(doc)
    }

    /// 获取第一个文档
    pub async fn frist() -> Result<Self, Error> {
        let coll_name = Self::to_coll_name();
        let db = Mongo::instance().await;
        let mut doc = db
            .find_one::<Self>(&coll_name, doc! {})
            .await
            .has_err("资源不存在")?;
        doc.db = Some(db);
        doc.coll_name = coll_name;
        doc.create_or_update = 1;
        Ok(doc)
    }

    /// 创建一个新的文档
    pub async fn create(data: T) -> Result<Self, Error> {
        let coll_name = Self::to_coll_name();
        let db = Mongo::instance().await;
        Ok(Self {
            _id: ObjectId::new(),
            data,
            created_at: timestamp() as i64,
            updated_at: timestamp() as i64,
            db: Some(db),
            coll_name,
            create_or_update: 0,
        })
    }

    /// 保存文档
    pub async fn save(&self) -> Result<(), Error> {
        match self.create_or_update {
            0 => {
                self.db
                    .unwrap()
                    .insert_one(&self.coll_name, &self)
                    .await
                    .has_err("创建数据失败")?;
            }
            1 => {
                let bson_doc = mongodb::bson::to_document(&self.data).has_err("文档解析失败")?;
                let update_data = doc! {
                    "data":bson_doc,
                    "updated_at":timestamp() as i64
                };
                self.db
                    .unwrap()
                    .update_one(
                        &self.coll_name,
                        doc! {"_id":self._id},
                        doc! {
                            "$set":update_data
                        },
                    )
                    .await
                    .has_err("文档修改失败")?;
            }
            _ => (),
        }
        Ok(())
    }

    /// 删除文档
    pub async fn delete(&self) -> Result<(), Error> {
        self.db
            .unwrap()
            .delete_one(&self.coll_name, doc! {"_id":self._id})
            .await
            .has_err("删除失败")?;
        Ok(())
    }

    /// `page_num`第一页为1
    pub async fn list(page_num: u64, filter: Document) -> Result<Vec<Doc<T>>, Error> {
        let db = Mongo::instance().await;
        let coll_name = Self::to_coll_name();
        let page_num = page_num - 1;
        let options = FindOptions::builder()
            .skip(page_num * 20)
            .limit(20)
            .sort(doc! { "created_at": -1 })
            .build();
        let list = db
            .find::<Doc<T>>(&coll_name, filter, Some(options))
            .await
            .has_err("查询列表失败")?;
        Ok(list)
    }
    ///
    pub async fn count(filter: Document) -> Result<u64, Error> {
        let db = Mongo::instance().await;
        let coll_name = Self::to_coll_name();
        let count = db.count(&coll_name, filter).await.has_err("查询数量失败")?;
        Ok(count)
    }

    pub async fn insert_many(documents: Vec<T>) -> Result<(), Error> {
        let db = Mongo::instance().await;
        let coll_name = Self::to_coll_name();
        let mut list = vec![];
        for ele in documents {
            let doc = Self {
                _id: ObjectId::new(),
                created_at: timestamp() as i64,
                updated_at: timestamp() as i64,
                data: ele,
                db: None,
                coll_name: "".to_string(),
                create_or_update: 1,
            };
            list.push(doc);
        }
        let _count = db
            .insert_many(&coll_name, list)
            .await
            .has_err("批量插入数据失败")?;
        Ok(())
    }
}

impl<T> Doc<T> {
    fn to_coll_name() -> String {
        let type_name = type_name::<T>();
        let type_parts: Vec<&str> = type_name.split("::").collect();
        let type_name = type_parts.last().unwrap();
        let mut output = String::new();

        for (i, c) in type_name.chars().enumerate() {
            if c.is_ascii_uppercase() {
                if i > 0 {
                    output.push('_');
                }
                output.push(c.to_ascii_lowercase());
            } else {
                output.push(c);
            }
        }
        output.push('s');
        output
    }
}
