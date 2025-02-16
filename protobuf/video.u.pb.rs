extern crate protobuf_upb as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct Video {
  inner: ::__pb::__runtime::MessageInner
}

impl ::__pb::Message for Video {}

impl ::__std::default::Default for Video {
  fn default() -> Self {
    Self::new()
  }
}

impl ::__pb::Parse for Video {
  fn parse(serialized: &[u8]) -> ::__std::result::Result<Self, ::__pb::ParseError> {
    Self::parse(serialized)
  }
}

impl ::__std::fmt::Debug for Video {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    let string = unsafe {
      ::__pb::__runtime::debug_string(
        self.raw_msg(),
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::__pb::MergeFrom for Video {
  fn merge_from<'src>(&mut self, src: impl ::__pb::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::__pb::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::__pb::Serialize for Video {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    ::__pb::AsView::as_view(self).serialize()
  }
}

impl ::__pb::Clear for Video {
  fn clear(&mut self) {
    self.as_mut().clear()
  }
}

impl ::__pb::ClearAndParse for Video {
  fn clear_and_parse(&mut self, data: &[u8]) -> ::__std::result::Result<(), ::__pb::ParseError> {
    let mut msg = Self::new();

    // SAFETY:
    // - `data.as_ptr()` is valid to read for `data.len()`
    // - `mini_table` is the one used to construct `msg.raw_msg()`
    // - `msg.arena().raw()` is held for the same lifetime as `msg`.
    let status = unsafe {
      ::__pb::__runtime::wire::decode(
          data,
          msg.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          msg.arena())
    };
    match status {
      Ok(_) => {
        ::__std::mem::swap(self, &mut msg);
        Ok(())
      }
      Err(_) => Err(::__pb::ParseError)
    }
  }
}

// SAFETY:
// - `Video` is `Sync` because it does not implement interior mutability.
//    Neither does `VideoMut`.
unsafe impl Sync for Video {}

// SAFETY:
// - `Video` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Video {}

impl ::__pb::Proxied for Video {
  type View<'msg> = VideoView<'msg>;
}

impl ::__pb::__internal::SealedInternal for Video {}

impl ::__pb::MutProxied for Video {
  type Mut<'msg> = VideoMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VideoView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::__pb::__internal::SealedInternal for VideoView<'msg> {}

impl<'msg> ::__pb::MessageView<'msg> for VideoView<'msg> {
  type Message = Video;
}

impl ::__std::fmt::Debug for VideoView<'_> {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    let string = unsafe {
      ::__pb::__runtime::debug_string(
        self.raw_msg(),
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::__pb::Serialize for VideoView<'_> {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::__pb::__runtime::wire::encode(self.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::__pb::SerializeError)
  }
}

impl ::__std::default::Default for VideoView<'_> {
  fn default() -> VideoView<'static> {
    VideoView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block())
  }
}

#[allow(dead_code)]
impl<'msg> VideoView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> Video {
    ::__pb::IntoProxied::into_proxied(*self, ::__pb::__internal::Private)
  }

  // type: optional enum myfirstprotobuf.mooc.content.Video.Type
  pub fn r#type(self) -> crate::video::Type {
    unsafe {
      let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::__pb::__runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (crate::video::Type::Unspecified).into()).try_into().unwrap()
    }
  }

  // url: optional string
  pub fn url(self) -> ::__pb::View<'msg, ::__pb::ProtoString> {
    let str_view = unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          1);
      ::__pb::__runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `VideoView` is `Sync` because it does not support mutation.
unsafe impl Sync for VideoView<'_> {}

// SAFETY:
// - `VideoView` is `Send` because while its alive a `VideoMut` cannot.
// - `VideoView` does not use thread-local data.
unsafe impl Send for VideoView<'_> {}

impl<'msg> ::__pb::Proxy<'msg> for VideoView<'msg> {}
impl<'msg> ::__pb::ViewProxy<'msg> for VideoView<'msg> {}

impl<'msg> ::__pb::AsView for VideoView<'msg> {
  type Proxied = Video;
  fn as_view(&self) -> ::__pb::View<'msg, Video> {
    *self
  }
}

impl<'msg> ::__pb::IntoView<'msg> for VideoView<'msg> {
  fn into_view<'shorter>(self) -> VideoView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::__pb::IntoProxied<Video> for VideoView<'msg> {
  fn into_proxied(self, _private: ::__pb::__internal::Private) -> Video {
    let dst = Video::new();
    unsafe { ::__pb::__runtime::upb_Message_DeepCopy(
      dst.inner.msg,
      self.msg,
      <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
      dst.inner.arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::__pb::IntoProxied<Video> for VideoMut<'msg> {
  fn into_proxied(self, _private: ::__pb::__internal::Private) -> Video {
    ::__pb::IntoProxied::into_proxied(::__pb::IntoView::into_view(self), _private)
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Video {
  fn repeated_new(_private: ::__pb::__internal::Private) -> ::__pb::Repeated<Self> {
    let arena = ::__pb::__runtime::Arena::new();
    unsafe {
      ::__pb::Repeated::from_inner(
          ::__pb::__internal::Private,
          ::__pb::__runtime::InnerRepeated::from_raw_parts(
              ::__pb::__runtime::upb_Array_New(arena.raw(), ::__pb::__runtime::CType::Message),
              arena,
          ))
    }
  }

  unsafe fn repeated_free(_private: ::__pb::__internal::Private, _f: &mut ::__pb::Repeated<Self>) {
    // No-op: the memory will be dropped by the arena.
  }

  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `upb_Array*`.
    unsafe { ::__pb::__runtime::upb_Array_Size(f.as_raw(::__pb::__internal::Private)) }
  }
  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: impl ::__pb::IntoProxied<Self>,
  ) {
    unsafe {
        ::__pb::__runtime::upb_Array_Set(
            f.as_raw(::__pb::__internal::Private),
            i,
            <Self as ::__pb::__runtime::UpbTypeConversions>::into_message_value_fuse_if_required(
                f.raw_arena(::__pb::__internal::Private),
                v.into_proxied(::__pb::__internal::Private),
            ),
        )
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const upb_Array*`.
    // - `i < len(f)` is promised by the caller.
    let msg_ptr = unsafe { ::__pb::__runtime::upb_Array_Get(f.as_raw(::__pb::__internal::Private), i).msg_val }
      .expect("upb_Array* element should not be NULL.");
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg_ptr)
  }

  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      ::__pb::__runtime::upb_Array_Resize(f.as_raw(::__pb::__internal::Private), 0, f.raw_arena(::__pb::__internal::Private))
    };
  }
  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: impl ::__pb::IntoProxied<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    // - `msg_ptr` is a valid `upb_Message*`.
    unsafe {
      ::__pb::__runtime::upb_Array_Append(
        f.as_raw(::__pb::__internal::Private),
        <Self as ::__pb::__runtime::UpbTypeConversions>::into_message_value_fuse_if_required(f.raw_arena(::__pb::__internal::Private), v.into_proxied(::__pb::__internal::Private)),
        f.raw_arena(::__pb::__internal::Private)
      );
    };
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
      // SAFETY:
      // - Elements of `src` and `dest` have message minitable `MINI_TABLE`.
      unsafe {
        ::__pb::__runtime::repeated_message_copy_from(src, dest, <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table());
      }
  }

  fn repeated_reserve(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    additional: usize,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      let size = ::__pb::__runtime::upb_Array_Size(f.as_raw(::__pb::__internal::Private));
      ::__pb::__runtime::upb_Array_Reserve(f.as_raw(::__pb::__internal::Private), size + additional, f.raw_arena(::__pb::__internal::Private));
    }
  }
}
impl ::__pb::__runtime::UpbTypeConversions for Video {
    fn upb_type() -> ::__pb::__runtime::CType {
        ::__pb::__runtime::CType::Message
    }

    fn to_message_value(
        val: ::__pb::View<'_, Self>) -> ::__pb::__runtime::upb_MessageValue {
        ::__pb::__runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::__pb::__runtime::RawArena,
      mut val: Self) -> ::__pb::__runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = core::mem::ManuallyDrop::new(
          unsafe { ::__pb::__runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_mutator_message_ref(::__pb::__internal::Private).arena());
      ::__pb::__runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::__pb::__runtime::upb_MessageValue)
        -> ::__pb::View<'msg, Self> {
        VideoView::new(
            ::__pb::__internal::Private,
            unsafe { msg.msg_val }
                .expect("expected present message value in map"))
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VideoMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::__pb::__internal::SealedInternal for VideoMut<'msg> {}

impl<'msg> ::__pb::MessageMut<'msg> for VideoMut<'msg> {
  type Message = Video;
}

impl ::__std::fmt::Debug for VideoMut<'_> {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    let string = unsafe {
      ::__pb::__runtime::debug_string(
        self.raw_msg(),
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::__pb::Serialize for VideoMut<'_> {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    ::__pb::AsView::as_view(self).serialize()
  }
}

impl ::__pb::Clear for VideoMut<'_> {
  fn clear(&mut self) {
    unsafe {
      ::__pb::__runtime::upb_Message_Clear(
          self.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
    }
  }
}

impl ::__pb::MergeFrom for VideoMut<'_> {
  fn merge_from(&mut self, src: impl ::__pb::AsView<Proxied = Video>) {
    // SAFETY: self and src are both valid `Video`s.
    unsafe {
      assert!(
        ::__pb::__runtime::upb_Message_MergeFrom(self.raw_msg(),
          src.as_view().raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          // Use a nullptr for the ExtensionRegistry.
          ::__std::ptr::null(),
          self.arena().raw())
      );
    }
  }
}

#[allow(dead_code)]
impl<'msg> VideoMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(msg) }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::__pb::__internal::Private)
    -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn to_owned(&self) -> Video {
    ::__pb::AsView::as_view(self).to_owned()
  }

  fn arena(&self) -> &::__pb::__runtime::Arena {
    self.inner.arena()
  }

  // type: optional enum myfirstprotobuf.mooc.content.Video.Type
  pub fn r#type(&self) -> crate::video::Type {
    unsafe {
      let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::__pb::__runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (crate::video::Type::Unspecified).into()).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: crate::video::Type) {
    unsafe {
      let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly.
      ::__pb::__runtime::upb_Message_SetBaseFieldInt32(
          self.raw_msg(), f, val.into());
    }
  }

  // url: optional string
  pub fn url(&self) -> ::__pb::View<'_, ::__pb::ProtoString> {
    let str_view = unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          1);
      ::__pb::__runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_url(&mut self, val: impl ::__pb::IntoProxied<::__pb::ProtoString>) {
    let s = val.into_proxied(::__pb::__internal::Private);
    let (view, arena) =
      s.into_inner(::__pb::__internal::Private).into_raw_parts();

    let mm_ref =
      self.as_mutator_message_ref(::__pb::__internal::Private);
    let parent_arena = mm_ref.arena();

    parent_arena.fuse(&arena);

    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                1);
      ::__pb::__runtime::upb_Message_SetBaseFieldString(
        self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
        f,
        view);
    }
  }

}

// SAFETY:
// - `VideoMut` does not perform any shared mutation.
// - `VideoMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for VideoMut<'_> {}

impl<'msg> ::__pb::Proxy<'msg> for VideoMut<'msg> {}
impl<'msg> ::__pb::MutProxy<'msg> for VideoMut<'msg> {}

impl<'msg> ::__pb::AsView for VideoMut<'msg> {
  type Proxied = Video;
  fn as_view(&self) -> ::__pb::View<'_, Video> {
    VideoView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

impl<'msg> ::__pb::IntoView<'msg> for VideoMut<'msg> {
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Video>
  where
      'msg: 'shorter {
    VideoView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

impl<'msg> ::__pb::AsMut for VideoMut<'msg> {
  type MutProxied = Video;
  fn as_mut(&mut self) -> VideoMut<'msg> {
    VideoMut { inner: self.inner }
  }
}

impl<'msg> ::__pb::IntoMut<'msg> for VideoMut<'msg> {
  fn into_mut<'shorter>(self) -> VideoMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Video {
  pub fn new() -> Self {
    let arena = ::__pb::__runtime::Arena::new();
    let raw_msg = unsafe {
        ::__pb::__runtime::upb_Message_New(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            arena.raw()).unwrap()
    };
    Self {
      inner: ::__pb::__runtime::MessageInner {
        msg: raw_msg,
        arena,
      }
    }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  #[doc(hidden)]
  pub fn as_mutator_message_ref(&mut self, _private: ::__pb::__internal::Private) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(&mut self.inner)
  }

  fn arena(&self) -> &::__pb::__runtime::Arena {
    &self.inner.arena
  }

  pub fn parse(data: &[u8]) -> ::__std::result::Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    ::__pb::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> VideoView {
    VideoView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> VideoMut {
    VideoMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // type: optional enum myfirstprotobuf.mooc.content.Video.Type
  pub fn r#type(&self) -> crate::video::Type {
    unsafe {
      let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);

      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      ::__pb::__runtime::upb_Message_GetInt32(
          self.raw_msg(), f, (crate::video::Type::Unspecified).into()).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: crate::video::Type) {
    unsafe {
      let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          mt, 0);
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly.
      ::__pb::__runtime::upb_Message_SetBaseFieldInt32(
          self.raw_msg(), f, val.into());
    }
  }

  // url: optional string
  pub fn url(&self) -> ::__pb::View<'_, ::__pb::ProtoString> {
    let str_view = unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          1);
      ::__pb::__runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_url(&mut self, val: impl ::__pb::IntoProxied<::__pb::ProtoString>) {
    let s = val.into_proxied(::__pb::__internal::Private);
    let (view, arena) =
      s.into_inner(::__pb::__internal::Private).into_raw_parts();

    let mm_ref =
      self.as_mutator_message_ref(::__pb::__internal::Private);
    let parent_arena = mm_ref.arena();

    parent_arena.fuse(&arena);

    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                1);
      ::__pb::__runtime::upb_Message_SetBaseFieldString(
        self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
        f,
        view);
    }
  }

}  // impl Video

impl ::__std::ops::Drop for Video {
  fn drop(&mut self) {
  }
}

impl ::__std::clone::Clone for Video {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::__pb::AsView for Video {
  type Proxied = Self;
  fn as_view(&self) -> VideoView {
    self.as_view()
  }
}

impl ::__pb::AsMut for Video {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VideoMut {
    self.as_mut()
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for Video {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstprotobuf__mooc__content__Video_msg_init)
    }
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for VideoView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstprotobuf__mooc__content__Video_msg_init)
    }
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for VideoMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstprotobuf__mooc__content__Video_msg_init)
    }
  }
}

pub mod video {
  #[repr(transparent)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub struct Type(i32);

  #[allow(non_upper_case_globals)]
  impl Type {
    pub const Unspecified: Type = Type(0);
    pub const Mp4: Type = Type(1);
    pub const Mov: Type = Type(2);
    pub const Unsupported: Type = Type(3);
  }

  impl ::__std::convert::From<Type> for i32 {
    fn from(val: Type) -> i32 {
      val.0
    }
  }

  impl ::__std::convert::From<i32> for Type {
    fn from(val: i32) -> Type {
      Self(val)
    }
  }

  impl ::__std::default::Default for Type {
    fn default() -> Self {
      Self(0)
    }
  }

  impl ::__std::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
      f.debug_tuple(stringify!(Type)).field(&self.0).finish()
    }
  }

  impl ::__pb::IntoProxied<i32> for Type {
    fn into_proxied(self, _: ::__pb::__internal::Private) -> i32 {
      self.0
    }
  }

  impl ::__pb::__internal::SealedInternal for Type {}

  impl ::__pb::Proxied for Type {
    type View<'a> = Type;
  }

  impl ::__pb::Proxy<'_> for Type {}
  impl ::__pb::ViewProxy<'_> for Type {}

  impl ::__pb::AsView for Type {
    type Proxied = Type;

    fn as_view(&self) -> Type {
      *self
    }
  }

  impl<'msg> ::__pb::IntoView<'msg> for Type {
    fn into_view<'shorter>(self) -> Type where 'msg: 'shorter {
      self
    }
  }

  unsafe impl ::__pb::ProxiedInRepeated for Type {
    fn repeated_new(_private: ::__pb::__internal::Private) -> ::__pb::Repeated<Self> {
      ::__pb::__runtime::new_enum_repeated()
    }

    unsafe fn repeated_free(_private: ::__pb::__internal::Private, f: &mut ::__pb::Repeated<Self>) {
      ::__pb::__runtime::free_enum_repeated(f)
    }

    fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
      ::__pb::__runtime::cast_enum_repeated_view(r).len()
    }

    fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: impl ::__pb::IntoProxied<Type>) {
      ::__pb::__runtime::cast_enum_repeated_mut(r).push(val.into_proxied(::__pb::__internal::Private))
    }

    fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
      ::__pb::__runtime::cast_enum_repeated_mut(r).clear()
    }

    unsafe fn repeated_get_unchecked(
        r: ::__pb::View<::__pb::Repeated<Self>>,
        index: usize,
    ) -> ::__pb::View<Type> {
      // SAFETY: In-bounds as promised by the caller.
      unsafe {
        ::__pb::__runtime::cast_enum_repeated_view(r)
          .get_unchecked(index)
          .try_into()
          .unwrap_unchecked()
      }
    }

    unsafe fn repeated_set_unchecked(
        r: ::__pb::Mut<::__pb::Repeated<Self>>,
        index: usize,
        val: impl ::__pb::IntoProxied<Type>,
    ) {
      // SAFETY: In-bounds as promised by the caller.
      unsafe {
        ::__pb::__runtime::cast_enum_repeated_mut(r)
          .set_unchecked(index, val.into_proxied(::__pb::__internal::Private))
      }
    }

    fn repeated_copy_from(
        src: ::__pb::View<::__pb::Repeated<Self>>,
        dest: ::__pb::Mut<::__pb::Repeated<Self>>,
    ) {
      ::__pb::__runtime::cast_enum_repeated_mut(dest)
        .copy_from(::__pb::__runtime::cast_enum_repeated_view(src))
    }

    fn repeated_reserve(
        r: ::__pb::Mut<::__pb::Repeated<Self>>,
        additional: usize,
    ) {
        // SAFETY:
        // - `f.as_raw()` is valid.
        ::__pb::__runtime::reserve_enum_repeated_mut(r, additional);
    }
  }

  // SAFETY: this is an enum type
  unsafe impl ::__pb::__internal::Enum for Type {
    const NAME: &'static str = "Type";

    fn is_known(value: i32) -> bool {
      matches!(value, 0|1|2|3)
    }
  }

  impl ::__pb::__runtime::UpbTypeConversions for Type {
      fn upb_type() -> ::__pb::__runtime::CType {
          ::__pb::__runtime::CType::Enum
      }

      fn to_message_value(
          val: ::__pb::View<'_, Self>) -> ::__pb::__runtime::upb_MessageValue {
          ::__pb::__runtime::upb_MessageValue { int32_val: val.0 }
      }

      unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::__pb::__runtime::RawArena,
        val: Self) -> ::__pb::__runtime::upb_MessageValue {
          ::__pb::__runtime::upb_MessageValue { int32_val: val.0 }
      }

      unsafe fn from_message_value<'msg>(val: ::__pb::__runtime::upb_MessageValue)
          -> ::__pb::View<'msg, Self> {
        Type(unsafe { val.int32_val })
      }
  }

}  // mod video
extern "C" {
  /// Opaque static extern for this message's MiniTable, generated
  /// by the upb C MiniTable codegen. The only valid way to
  /// reference this static is with `std::ptr::addr_of!(..)`.
  static myfirstprotobuf__mooc__content__Video_msg_init: ::__pb::__runtime::upb_MiniTable;
}

// upb kernel doesn't support any owned message or message mut interop.
impl ::__pb::OwnedMessageInterop for Video {}
impl<'a> ::__pb::MessageMutInterop<'a> for VideoMut<'a> {}

impl<'a> ::__pb::MessageViewInterop<'a> for VideoView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::__std::ffi::c_void) -> Self {
    Self::new(::__pb::__internal::Private, ::__pb::__runtime::RawMessage::new(*msg as *mut _).unwrap())
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::__std::ffi::c_void) -> Self {
    Self::new(::__pb::__internal::Private, ::__pb::__runtime::RawMessage::new(msg as *mut _).unwrap())
  }
  fn __unstable_as_raw_message(&self) -> *const ::__std::ffi::c_void {
    self.msg.as_ptr() as *const _
  }
}

impl ::__pb::__internal::MatcherEq for Video {
  fn matches(&self, o: &Self) -> bool {
    ::__pb::__internal::MatcherEq::matches(
      &::__pb::AsView::as_view(self),
      &::__pb::AsView::as_view(o))
  }
}

impl<'a> ::__pb::__internal::MatcherEq for VideoMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::__pb::__internal::MatcherEq::matches(
      &::__pb::AsView::as_view(self),
      &::__pb::AsView::as_view(o))
  }
}

impl<'a> ::__pb::__internal::MatcherEq for VideoView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::__pb::__runtime::upb_Message_IsEqual(
          self.msg,
          o.msg,
          <Video as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          0)
    }
  }
}

