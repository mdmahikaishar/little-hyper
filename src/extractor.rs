/// Extractor
/// 

pub trait Extractor<T> {
  /// Inner
  /// 
  /// Get inner value.
  fn inner(&self) -> &T;
}