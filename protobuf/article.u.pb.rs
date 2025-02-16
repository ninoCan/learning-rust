extern crate protobuf_upb as __pb;
extern crate std as __std;
#[path="course.u.pb.rs"]
#[allow(non_snake_case)]
pub mod course;
#[path="video.u.pb.rs"]
#[allow(non_snake_case)]
pub mod video;
#[allow(unused_imports)]
pub use crate::course::*;
#[allow(unused_imports)]
pub use crate::video::*;
#[allow(non_camel_case_types)]
pub struct Article {
  inner: ::__pb::__runtime::MessageInner
}

impl ::__pb::Message for Article {}

impl ::__std::default::Default for Article {
  fn default() -> Self {
    Self::new()
  }
}

impl ::__pb::Parse for Article {
  fn parse(serialized: &[u8]) -> ::__std::result::Result<Self, ::__pb::ParseError> {
    Self::parse(serialized)
  }
}

impl ::__std::fmt::Debug for Article {
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

impl ::__pb::MergeFrom for Article {
  fn merge_from<'src>(&mut self, src: impl ::__pb::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::__pb::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::__pb::Serialize for Article {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    ::__pb::AsView::as_view(self).serialize()
  }
}

impl ::__pb::Clear for Article {
  fn clear(&mut self) {
    self.as_mut().clear()
  }
}

impl ::__pb::ClearAndParse for Article {
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
// - `Article` is `Sync` because it does not implement interior mutability.
//    Neither does `ArticleMut`.
unsafe impl Sync for Article {}

// SAFETY:
// - `Article` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Article {}

impl ::__pb::Proxied for Article {
  type View<'msg> = ArticleView<'msg>;
}

impl ::__pb::__internal::SealedInternal for Article {}

impl ::__pb::MutProxied for Article {
  type Mut<'msg> = ArticleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ArticleView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::__pb::__internal::SealedInternal for ArticleView<'msg> {}

impl<'msg> ::__pb::MessageView<'msg> for ArticleView<'msg> {
  type Message = Article;
}

impl ::__std::fmt::Debug for ArticleView<'_> {
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

impl ::__pb::Serialize for ArticleView<'_> {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::__pb::__runtime::wire::encode(self.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::__pb::SerializeError)
  }
}

impl ::__std::default::Default for ArticleView<'_> {
  fn default() -> ArticleView<'static> {
    ArticleView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block())
  }
}

#[allow(dead_code)]
impl<'msg> ArticleView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> Article {
    ::__pb::IntoProxied::into_proxied(*self, ::__pb::__internal::Private)
  }

  // text: optional string
  pub fn text(self) -> ::__pb::View<'msg, ::__pb::ProtoString> {
    let str_view = unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::__pb::__runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ArticleView` is `Sync` because it does not support mutation.
unsafe impl Sync for ArticleView<'_> {}

// SAFETY:
// - `ArticleView` is `Send` because while its alive a `ArticleMut` cannot.
// - `ArticleView` does not use thread-local data.
unsafe impl Send for ArticleView<'_> {}

impl<'msg> ::__pb::Proxy<'msg> for ArticleView<'msg> {}
impl<'msg> ::__pb::ViewProxy<'msg> for ArticleView<'msg> {}

impl<'msg> ::__pb::AsView for ArticleView<'msg> {
  type Proxied = Article;
  fn as_view(&self) -> ::__pb::View<'msg, Article> {
    *self
  }
}

impl<'msg> ::__pb::IntoView<'msg> for ArticleView<'msg> {
  fn into_view<'shorter>(self) -> ArticleView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::__pb::IntoProxied<Article> for ArticleView<'msg> {
  fn into_proxied(self, _private: ::__pb::__internal::Private) -> Article {
    let dst = Article::new();
    unsafe { ::__pb::__runtime::upb_Message_DeepCopy(
      dst.inner.msg,
      self.msg,
      <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
      dst.inner.arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::__pb::IntoProxied<Article> for ArticleMut<'msg> {
  fn into_proxied(self, _private: ::__pb::__internal::Private) -> Article {
    ::__pb::IntoProxied::into_proxied(::__pb::IntoView::into_view(self), _private)
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Article {
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
impl ::__pb::__runtime::UpbTypeConversions for Article {
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
        ArticleView::new(
            ::__pb::__internal::Private,
            unsafe { msg.msg_val }
                .expect("expected present message value in map"))
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ArticleMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::__pb::__internal::SealedInternal for ArticleMut<'msg> {}

impl<'msg> ::__pb::MessageMut<'msg> for ArticleMut<'msg> {
  type Message = Article;
}

impl ::__std::fmt::Debug for ArticleMut<'_> {
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

impl ::__pb::Serialize for ArticleMut<'_> {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    ::__pb::AsView::as_view(self).serialize()
  }
}

impl ::__pb::Clear for ArticleMut<'_> {
  fn clear(&mut self) {
    unsafe {
      ::__pb::__runtime::upb_Message_Clear(
          self.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
    }
  }
}

impl ::__pb::MergeFrom for ArticleMut<'_> {
  fn merge_from(&mut self, src: impl ::__pb::AsView<Proxied = Article>) {
    // SAFETY: self and src are both valid `Article`s.
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
impl<'msg> ArticleMut<'msg> {
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

  pub fn to_owned(&self) -> Article {
    ::__pb::AsView::as_view(self).to_owned()
  }

  fn arena(&self) -> &::__pb::__runtime::Arena {
    self.inner.arena()
  }

  // text: optional string
  pub fn text(&self) -> ::__pb::View<'_, ::__pb::ProtoString> {
    let str_view = unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::__pb::__runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_text(&mut self, val: impl ::__pb::IntoProxied<::__pb::ProtoString>) {
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
                0);
      ::__pb::__runtime::upb_Message_SetBaseFieldString(
        self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
        f,
        view);
    }
  }

}

// SAFETY:
// - `ArticleMut` does not perform any shared mutation.
// - `ArticleMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ArticleMut<'_> {}

impl<'msg> ::__pb::Proxy<'msg> for ArticleMut<'msg> {}
impl<'msg> ::__pb::MutProxy<'msg> for ArticleMut<'msg> {}

impl<'msg> ::__pb::AsView for ArticleMut<'msg> {
  type Proxied = Article;
  fn as_view(&self) -> ::__pb::View<'_, Article> {
    ArticleView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

impl<'msg> ::__pb::IntoView<'msg> for ArticleMut<'msg> {
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Article>
  where
      'msg: 'shorter {
    ArticleView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

impl<'msg> ::__pb::AsMut for ArticleMut<'msg> {
  type MutProxied = Article;
  fn as_mut(&mut self) -> ArticleMut<'msg> {
    ArticleMut { inner: self.inner }
  }
}

impl<'msg> ::__pb::IntoMut<'msg> for ArticleMut<'msg> {
  fn into_mut<'shorter>(self) -> ArticleMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Article {
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

  pub fn as_view(&self) -> ArticleView {
    ArticleView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ArticleMut {
    ArticleMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // text: optional string
  pub fn text(&self) -> ::__pb::View<'_, ::__pb::ProtoString> {
    let str_view = unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::__pb::__runtime::upb_Message_GetString(
          self.raw_msg(), f, (b"").into())
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_text(&mut self, val: impl ::__pb::IntoProxied<::__pb::ProtoString>) {
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
                0);
      ::__pb::__runtime::upb_Message_SetBaseFieldString(
        self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
        f,
        view);
    }
  }

}  // impl Article

impl ::__std::ops::Drop for Article {
  fn drop(&mut self) {
  }
}

impl ::__std::clone::Clone for Article {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::__pb::AsView for Article {
  type Proxied = Self;
  fn as_view(&self) -> ArticleView {
    self.as_view()
  }
}

impl ::__pb::AsMut for Article {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ArticleMut {
    self.as_mut()
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for Article {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstprotobuf__mooc__content__Article_msg_init)
    }
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for ArticleView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstprotobuf__mooc__content__Article_msg_init)
    }
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for ArticleMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstprotobuf__mooc__content__Article_msg_init)
    }
  }
}

extern "C" {
  /// Opaque static extern for this message's MiniTable, generated
  /// by the upb C MiniTable codegen. The only valid way to
  /// reference this static is with `std::ptr::addr_of!(..)`.
  static myfirstprotobuf__mooc__content__Article_msg_init: ::__pb::__runtime::upb_MiniTable;
}

// upb kernel doesn't support any owned message or message mut interop.
impl ::__pb::OwnedMessageInterop for Article {}
impl<'a> ::__pb::MessageMutInterop<'a> for ArticleMut<'a> {}

impl<'a> ::__pb::MessageViewInterop<'a> for ArticleView<'a> {
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

impl ::__pb::__internal::MatcherEq for Article {
  fn matches(&self, o: &Self) -> bool {
    ::__pb::__internal::MatcherEq::matches(
      &::__pb::AsView::as_view(self),
      &::__pb::AsView::as_view(o))
  }
}

impl<'a> ::__pb::__internal::MatcherEq for ArticleMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::__pb::__internal::MatcherEq::matches(
      &::__pb::AsView::as_view(self),
      &::__pb::AsView::as_view(o))
  }
}

impl<'a> ::__pb::__internal::MatcherEq for ArticleView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::__pb::__runtime::upb_Message_IsEqual(
          self.msg,
          o.msg,
          <Article as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          0)
    }
  }
}

