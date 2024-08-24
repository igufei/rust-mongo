use mongodb::bson::{doc, Document};
pub trait FilterDoc {
    fn to_doc(&self) -> Document;
}
/// 相似
pub struct Like {
    pub key: String,
    pub value: String,
}
impl Like {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
impl FilterDoc for Like {
    fn to_doc(&self) -> Document {
        let key = format!("data.{}", &self.key);
        let filter = doc! {
            key:{
                "$regex": &self.value
            }
        };
        filter
    }
}

/// 相等
pub struct Eq {
    pub key: String,
    pub value: String,
}
impl Eq {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
impl FilterDoc for Eq {
    fn to_doc(&self) -> Document {
        let key = format!("data.{}", &self.key);
        let filter = doc! {
            key:&self.value
        };
        filter
    }
}
pub struct Filter(Vec<Box<dyn FilterDoc>>);
impl Filter {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn push<T: FilterDoc + 'static>(&mut self, filter: T) -> &mut Self {
        self.0.push(Box::new(filter));
        self
    }
    pub fn to_doc(&self) -> Document {
        let mut filter_list = vec![];
        for ele in self.0.iter() {
            let filter_item = ele.to_doc();
            filter_list.push(filter_item);
        }
        let filter = doc! {
        "$and": filter_list
        };
        filter
    }
}
