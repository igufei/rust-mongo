use mongodb::bson::{doc, oid::ObjectId};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{error::{Error, ResultExtention}, timestamp};

use super::{mongo::Mongo};
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
    coll_name: &'static str,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    create_or_update: u8,
}
impl<T> Doc<T>
where
    T: Serialize + DeserializeOwned,
{
    pub async fn load(coll_name: &'static str, _id: &str) -> Result<Self, Error> {
        let db = Mongo::instance().await;
        let _id = ObjectId::parse_str(_id).has_err("资源id不合法")?;
        let mut doc = db
            .find_one::<Self>(coll_name, doc! {"_id":_id})
            .await
            .has_err("资源不存在")?;
        doc.db = Some(db);
        doc.coll_name = coll_name;
        doc.create_or_update = 1;
        Ok(doc)
    }

    pub async fn create(coll_name: &'static str, data: T) -> Result<Self, Error> {
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

    pub async fn save(&self) -> Result<(), Error> {
        match self.create_or_update {
            0 => {
                self.db
                    .unwrap()
                    .insert_one(self.coll_name, &self)
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
                        self.coll_name,
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

    pub async fn delete(&self) -> Result<(), Error> {
        self.db
            .unwrap()
            .delete_one(self.coll_name, doc! {"_id":self._id})
            .await
            .has_err("删除失败")?;
        Ok(())
    }
}
