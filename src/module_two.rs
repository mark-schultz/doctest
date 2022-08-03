use crate::module_one::{MyStruct, MyTrait};

impl MyTrait for MyStruct {
  /// This doctest has a confusing error message
  ///
  /// ```rust
  /// use doctest::module_one::{MyStruct, MyTrait};
  /// use doctest::module_two::returns_true;
  /// let my_struct : MyStruct = MyStruct {};
  /// assert!(my_struct.returns_true());
  ///
  /// ```
  ///
  fn returns_true(self) -> bool {
    true
  }
}
