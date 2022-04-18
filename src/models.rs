use super::schema::bfts;

#[derive(Clone, Queryable, Serialize)]
pub struct BFT {
  pub id: i32,
  pub name: String,
  pub link: String,
}

#[derive(Serialize)]
pub struct BFTS {
  pub bfts: Vec<BFT>,
}

#[derive(Insertable, Deserialize)]
#[table_name="bfts"]
pub struct NewBFT<'a> {
  pub name: &'a str,
  pub link: &'a str,
}
