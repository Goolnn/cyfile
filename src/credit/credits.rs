use crate::error::FileResult;

use crate::Credit;

use crate::{
  Encode,
  Decode,
  Codec,
};

use std::collections::{
  HashSet,
  HashMap,
};

pub type Name = String;

pub struct Credits(HashMap<Credit, HashSet<Name>>);

impl Credits {
  pub fn new() -> Self {
    Self(HashMap::new())
  }

  pub fn insert(&mut self, credit: Credit, name: &str) {
    self.0.entry(credit).or_default().insert(name.to_string());
  }

  pub fn remove(&mut self, credit: Credit, name: &str) {
    if let Some(credits) = self.0.get_mut(&credit) {
      if credits.remove(name) && credits.is_empty() {
        self.0.remove(&credit);
      }
    }
  }

  pub fn get_mut(&mut self, credit: Credit) -> Option<&mut HashSet<Name>> {
    self.0.get_mut(&credit)
  }

  pub fn get(&self, credit: Credit) -> Option<&HashSet<Name>> {
    self.0.get(&credit)
  }

  pub fn inner_mut(&mut self) -> &mut HashMap<Credit, HashSet<Name>> {
    &mut self.0
  }

  pub fn inner(&self) -> &HashMap<Credit, HashSet<Name>> {
    &self.0
  }
}

impl<const N: usize> From<[(Credit, HashSet<Name>); N]> for Credits {
  fn from(value: [(Credit, HashSet<Name>); N]) -> Self {
    Self(HashMap::from(value))
  }
}

impl Default for Credits {
  fn default() -> Self {
    Self::new()
  }
}

impl Encode for Credits {
  fn encode(&self, codec: &mut Codec) -> FileResult<()> {
    // 职位数量
    codec.write_primitive(self.0.len() as u32)?;

    for (credit, staffs) in &self.0 {
      codec.write_object(credit)?;

      // 人员数量
      codec.write_primitive(staffs.len() as u32)?;

      for staff in staffs {
        codec.write_string::<u32>(staff)?;
      }
    }

    Ok(())
  }
}

impl Decode for Credits {
  fn decode(codec: &mut Codec) -> FileResult<Self> {
    // 职位数量
    let credit_count = codec.read_primitive::<u32>()?;

    let mut credits = HashMap::with_capacity(credit_count as usize);

    for _ in 0..credit_count {
      let credit = codec.read_object::<Credit>()?;

      // 人员数量
      let staffs_count = codec.read_primitive::<u32>()?;

      let mut staffs = HashSet::with_capacity(staffs_count as usize);

      for _ in 0..staffs_count {
        let staff = codec.read_string::<u32>()?;

        staffs.insert(staff);
      }

      credits.insert(credit, staffs);
    }

    Ok(Self(credits))
  }
}
