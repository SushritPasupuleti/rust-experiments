# Rust Experiments

Teaching myself rust practically.

## Borrow Checker

Things to note:

- Value: Only one owner. If the object implements `Copy trait`, then it is copied
  instead of moved.

  ```rust
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);
  ```

- Reference: Can have as many references as needed with the constraint that none are mutable references.

  ```rust
  let x = 5;
  let y = &x;
  let z = &x;
  println!("x = {}, y = {}, z = {}", x, y, z);
  ```

- Mutable Reference
