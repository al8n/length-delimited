macro_rules! doc {
  (@bounds { $($item:item)* }) => {
    $(
      /// A trait bound that bounds `Send`, `Sync`, and `'static` in possible combinations
      ///
      /// which can be configured with features.
      $item
    )*
  };
  (@error_bounds { $($item:item)* }) => {
    $(
      /// A trait bound that bounds `Debug`, `Display`, `Error`, `Send`, `Sync`, and `'static` in possible combinations
      ///
      /// which can be configured with features.
      $item
    )*
  };
}

doc!(@bounds {
  #[cfg(not(any(feature = "send", feature = "sync", feature = "static")))]
  pub trait Bounds {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  pub trait Bounds: Sync {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  pub trait Bounds: Send {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  pub trait Bounds: 'static {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  pub trait Bounds: Send + Sync {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  pub trait Bounds: Sync + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  pub trait Bounds: Send + 'static {}

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  pub trait Bounds: Send + Sync + 'static {}
});

#[cfg(not(any(feature = "send", feature = "sync", feature = "static")))]
impl<T> Bounds for T {}

#[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
impl<T> Bounds for T where T: Sync {}

#[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
impl<T> Bounds for T where T: Send {}

#[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
impl<T> Bounds for T where T: 'static {}

#[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
impl<T> Bounds for T where T: Send + Sync {}

#[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
impl<T> Bounds for T where T: Sync + 'static {}

#[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
impl<T> Bounds for T where T: Send + 'static {}

#[cfg(all(feature = "send", feature = "sync", feature = "static"))]
impl<T> Bounds for T where T: Send + Sync + 'static {}

#[cfg(not(any(feature = "debug", feature = "display", feature = "error")))]
pub trait Printable {}

#[cfg(not(any(feature = "debug", feature = "display", feature = "error")))]
impl<T> Printable for T {}

#[cfg(all(feature = "debug", not(any(feature = "display", feature = "error"))))]
pub trait Printable: core::fmt::Debug {}

#[cfg(all(feature = "debug", not(any(feature = "display", feature = "error"))))]
impl<T> Printable for T where T: core::fmt::Debug {}

#[cfg(all(feature = "display", not(any(feature = "debug", feature = "error"))))]
pub trait Printable: core::fmt::Display {}

#[cfg(all(feature = "display", not(any(feature = "debug", feature = "error"))))]
impl<T> Printable for T where T: core::fmt::Display {}

#[cfg(all(feature = "display", feature = "debug", not(any(feature = "error"))))]
pub trait Printable: core::fmt::Display + core::fmt::Debug {}

#[cfg(all(feature = "display", feature = "debug", not(any(feature = "error"))))]
impl<T> Printable for T where T: core::fmt::Display + core::fmt::Debug {}

#[cfg(feature = "error")]
pub trait Printable: core::error::Error {}

#[cfg(feature = "error")]
impl<T> Printable for T where T: core::error::Error {}

#[cfg(not(any(feature = "send", feature = "sync", feature = "static",)))]
impl<T> ErrorBounds for T where T: Printable {}

#[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
impl<T> ErrorBounds for T where T: Printable + Sync {}

#[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
impl<T> ErrorBounds for T where T: Printable + Send {}

#[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
impl<T> ErrorBounds for T where T: Printable + 'static {}

#[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
impl<T> ErrorBounds for T where T: Printable + Send + Sync {}

#[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
impl<T> ErrorBounds for T where T: Printable + Sync + 'static {}

#[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
impl<T> ErrorBounds for T where T: Printable + Send + 'static {}

#[cfg(all(feature = "send", feature = "sync", feature = "static"))]
impl<T> ErrorBounds for T where T: Printable + Send + Sync + 'static {}

doc!(@error_bounds {
  #[cfg(not(any(feature = "send", feature = "sync", feature = "static",)))]
  pub trait ErrorBounds: Printable {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  pub trait ErrorBounds: Printable + Sync {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  pub trait ErrorBounds: Printable + Send {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  pub trait ErrorBounds: Printable + 'static {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  pub trait ErrorBounds: Printable + Send + Sync {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  pub trait ErrorBounds: Printable + Sync + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  pub trait ErrorBounds: Printable + Send + 'static {}

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  pub trait ErrorBounds: Printable + Send + Sync + 'static {}
});
