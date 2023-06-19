use mongodb::bson::{doc, Document};

pub struct Filter {
    pub key: String,
    pub value: Option<String>,
}
impl Filter {
    pub fn to_doc(&self) -> Document {
        let key = format!("data.{}", &self.key);
        let filter = match &self.value {
            Some(value) => {
                doc! {
                    key:{
                        "$regex": value
                    }
                }
            }
            None => doc! {},
        };
        filter
    }
}

pub struct FilterList(Vec<Filter>);
impl FilterList {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn push(&mut self, filter: Filter) -> &mut Self {
        self.0.push(filter);
        self
    }
    pub fn to_doc(&self) -> Document {
        let mut filter_list = vec![];
        for ele in self.0.iter() {
            let filter_item = ele.to_doc();
            filter_list.push(filter_item);
        }
        let filter = doc! {
            "$and":filter_list
        };
        filter
    }
}
