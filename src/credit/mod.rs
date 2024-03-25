#[derive(Clone, Copy)]
pub enum Credit {
  // 翻译
  Translators,
  // 校对
  Proofreaders,
  // 修图
  Retouchers,
  // 嵌字
  Typesetters,
  // 监修
  Supervisors,
}

impl From<u8> for Credit {
  fn from(value: u8) -> Self {
    match value { 
      0 => Self::Translators,
      1 => Self::Proofreaders,
      2 => Self::Retouchers,
      3 => Self::Typesetters,
      4 => Self::Supervisors,
      
      _ => Self::Translators,
    }
  }
}