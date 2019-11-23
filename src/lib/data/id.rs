use snowflake::ProcessUniqueId;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Id(pub snowflake::ProcessUniqueId);

impl Id {
  pub fn new() -> Id {
    Id(ProcessUniqueId::new())
  }
}

pub trait Identifiable {
  fn uid(&self) -> Id;
}

