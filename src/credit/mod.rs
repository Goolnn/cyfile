/// Be used to enumerate different positions of staff.
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum Credit {
  /// Someone who specialize in converting written or spoken text from one language to another.
  Translators,
  /// Someone who are responsible for reviewing written text to identify and correct errors in
  /// spelling, grammar, punctuation, and formatting.
  Proofreaders,
  /// Someont who specialize in the post-production process of enhancing or altering digital images.
  Retouchers,
  /// Someone who are responsible for arranging and formatting text in a visually appealing manner
  /// for publication.
  Typesetters,
  /// Someone who oversee and manage the work of others within an organization or a specific area.
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