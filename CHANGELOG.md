# Changelog

## 0.1.6 (2024-09-21)

* [#10](https://github.com/synek317/synonym/pull/10) Fix `missing_docs` warnings by simply adding `#[allow]` to generated methods (author: Mike Tsao)

## 0.1.5 (2024-07-01)

* Fix warnings when `with_serde` feature is not enabled

## 0.1.4 (2024-07-01)

* Add `Box<str>` and `&'static str` synonyms
* Add `NonZero*` synonyms
* Add `.value()` method
* Add `From<String>` for `Box<str>` synonyms
* Optimize `FromStr` for `Box<str>` synonyms
* Reorganize the documentation

## 0.1.3 (2024-06-05)

* Fix `missing_docs` warning on `as_str` 

## 0.1.2 (2023-05-30)

* [#2](https://github.com/synek317/synonym/pull/2) Fix `as_str` for String synonyms (author: Mike Tsao)
* Reduce dependencies

## 0.1.1 (2023-11-22)

* Update to Rust 1.74

## 0.1.0 (2023-09-02)

* Update to Rust 1.72
* Add documentation

## 0.0.5 (2020-06-14)

* Add serde support

## 0.0.4 (2020-06-13)

* Impl `as_str` for String synonyms

## 0.0.3 (2020-03-11)

* Add Display

## 0.0.2 (2020-01-21)

* Add number ops

## 0.0.1 (2020-01-20)

* Initial release