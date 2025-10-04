const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__Vector3_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct Vector3 {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Vector3>
}

impl ::protobuf::Message for Vector3 {}

impl ::std::default::Default for Vector3 {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for Vector3 {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for Vector3 {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for Vector3 {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `Vector3` is `Sync` because it does not implement interior mutability.
//    Neither does `Vector3Mut`.
unsafe impl Sync for Vector3 {}

// SAFETY:
// - `Vector3` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Vector3 {}

impl ::protobuf::Proxied for Vector3 {
  type View<'msg> = Vector3View<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Vector3 {}

impl ::protobuf::MutProxied for Vector3 {
  type Mut<'msg> = Vector3Mut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Vector3View<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector3View<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Vector3View<'msg> {
  type Message = Vector3;
}

impl ::std::fmt::Debug for Vector3View<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for Vector3View<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for Vector3View<'_> {
  fn default() -> Vector3View<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    Vector3View::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> Vector3View<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> Vector3 {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // x: optional float
  pub fn x(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // y: optional float
  pub fn y(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // z: optional float
  pub fn z(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Vector3View` is `Sync` because it does not support mutation.
unsafe impl Sync for Vector3View<'_> {}

// SAFETY:
// - `Vector3View` is `Send` because while its alive a `Vector3Mut` cannot.
// - `Vector3View` does not use thread-local data.
unsafe impl Send for Vector3View<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for Vector3View<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for Vector3View<'msg> {}

impl<'msg> ::protobuf::AsView for Vector3View<'msg> {
  type Proxied = Vector3;
  fn as_view(&self) -> ::protobuf::View<'msg, Vector3> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector3View<'msg> {
  fn into_view<'shorter>(self) -> Vector3View<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector3> for Vector3View<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector3 {
    let mut dst = Vector3::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector3> for Vector3Mut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector3 {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for Vector3 {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        Vector3View::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> Vector3Mut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, Vector3>::wrap_raw(msg, arena) };
        Vector3Mut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Vector3Mut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector3Mut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Vector3Mut<'msg> {
  type Message = Vector3;
}

impl ::std::fmt::Debug for Vector3Mut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for Vector3Mut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> Vector3Mut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3> {
    self.inner
  }

  pub fn to_owned(&self) -> Vector3 {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // x: optional float
  pub fn x(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // y: optional float
  pub fn y(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // z: optional float
  pub fn z(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_z(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `Vector3Mut` does not perform any shared mutation.
// - `Vector3Mut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for Vector3Mut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for Vector3Mut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for Vector3Mut<'msg> {}

impl<'msg> ::protobuf::AsView for Vector3Mut<'msg> {
  type Proxied = Vector3;
  fn as_view(&self) -> ::protobuf::View<'_, Vector3> {
    Vector3View {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector3Mut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Vector3>
  where
      'msg: 'shorter {
    Vector3View {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for Vector3Mut<'msg> {
  type MutProxied = Vector3;
  fn as_mut(&mut self) -> Vector3Mut<'msg> {
    Vector3Mut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Vector3Mut<'msg> {
  fn into_mut<'shorter>(self) -> Vector3Mut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Vector3 {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Vector3> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> Vector3View {
    Vector3View::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> Vector3Mut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    Vector3Mut::new(::protobuf::__internal::Private, inner)
  }

  // x: optional float
  pub fn x(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // y: optional float
  pub fn y(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // z: optional float
  pub fn z(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_z(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

}  // impl Vector3

impl ::std::ops::Drop for Vector3 {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Vector3 {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Vector3 {
  type Proxied = Self;
  fn as_view(&self) -> Vector3View {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Vector3 {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Vector3Mut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vector3 {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__Vector3_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$!P!P!P".as_ptr(),
              7,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__Vector3_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__Vector3_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector3 {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vector3View<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vector3Mut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector3 {
  type Msg = Vector3;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3 {
  type Msg = Vector3;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector3Mut<'_> {
  type Msg = Vector3;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3Mut<'_> {
  type Msg = Vector3;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3View<'_> {
  type Msg = Vector3;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector3Mut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for Vector3 {}
impl<'a> ::protobuf::MessageMutInterop<'a> for Vector3Mut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for Vector3View<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

