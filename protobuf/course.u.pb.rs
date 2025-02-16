extern crate protobuf_upb as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct Course {
  inner: ::__pb::__runtime::MessageInner
}

impl ::__pb::Message for Course {}

impl ::__std::default::Default for Course {
  fn default() -> Self {
    Self::new()
  }
}

impl ::__pb::Parse for Course {
  fn parse(serialized: &[u8]) -> ::__std::result::Result<Self, ::__pb::ParseError> {
    Self::parse(serialized)
  }
}

impl ::__std::fmt::Debug for Course {
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

impl ::__pb::MergeFrom for Course {
  fn merge_from<'src>(&mut self, src: impl ::__pb::AsView<Proxied = Self>) {
    let mut m = self.as_mut();
    ::__pb::MergeFrom::merge_from(&mut m, src)
  }
}

impl ::__pb::Serialize for Course {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    ::__pb::AsView::as_view(self).serialize()
  }
}

impl ::__pb::Clear for Course {
  fn clear(&mut self) {
    self.as_mut().clear()
  }
}

impl ::__pb::ClearAndParse for Course {
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
// - `Course` is `Sync` because it does not implement interior mutability.
//    Neither does `CourseMut`.
unsafe impl Sync for Course {}

// SAFETY:
// - `Course` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Course {}

impl ::__pb::Proxied for Course {
  type View<'msg> = CourseView<'msg>;
}

impl ::__pb::__internal::SealedInternal for Course {}

impl ::__pb::MutProxied for Course {
  type Mut<'msg> = CourseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CourseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::__pb::__internal::SealedInternal for CourseView<'msg> {}

impl<'msg> ::__pb::MessageView<'msg> for CourseView<'msg> {
  type Message = Course;
}

impl ::__std::fmt::Debug for CourseView<'_> {
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

impl ::__pb::Serialize for CourseView<'_> {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::__pb::__runtime::wire::encode(self.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::__pb::SerializeError)
  }
}

impl ::__std::default::Default for CourseView<'_> {
  fn default() -> CourseView<'static> {
    CourseView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block())
  }
}

#[allow(dead_code)]
impl<'msg> CourseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn to_owned(&self) -> Course {
    ::__pb::IntoProxied::into_proxied(*self, ::__pb::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::__pb::View<'msg, ::__pb::ProtoString> {
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

  // authors: repeated string
  pub fn authors(self) -> ::__pb::RepeatedView<'msg, ::__pb::ProtoString> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1);
      ::__pb::__runtime::upb_Message_GetArray(
        self.raw_msg(), f)
    }.map_or_else(
        ::__pb::__runtime::empty_array::<::__pb::ProtoString>,
        |raw| unsafe {
          ::__pb::RepeatedView::from_raw(::__pb::__internal::Private, raw)
        }
      )
  }

  // lectures: repeated message myfirstproto.mooc.Course.LecturesEntry
  pub fn lectures(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoString, crate::course::Lecture> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        2);
      ::__pb::__runtime::upb_Message_GetMap(self.raw_msg(), f)
        .map_or_else(
          ::__pb::__runtime::empty_map::<::__pb::ProtoString, crate::course::Lecture>,
          |raw| ::__pb::MapView::from_raw(::__pb::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `CourseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CourseView<'_> {}

// SAFETY:
// - `CourseView` is `Send` because while its alive a `CourseMut` cannot.
// - `CourseView` does not use thread-local data.
unsafe impl Send for CourseView<'_> {}

impl<'msg> ::__pb::Proxy<'msg> for CourseView<'msg> {}
impl<'msg> ::__pb::ViewProxy<'msg> for CourseView<'msg> {}

impl<'msg> ::__pb::AsView for CourseView<'msg> {
  type Proxied = Course;
  fn as_view(&self) -> ::__pb::View<'msg, Course> {
    *self
  }
}

impl<'msg> ::__pb::IntoView<'msg> for CourseView<'msg> {
  fn into_view<'shorter>(self) -> CourseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::__pb::IntoProxied<Course> for CourseView<'msg> {
  fn into_proxied(self, _private: ::__pb::__internal::Private) -> Course {
    let dst = Course::new();
    unsafe { ::__pb::__runtime::upb_Message_DeepCopy(
      dst.inner.msg,
      self.msg,
      <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
      dst.inner.arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::__pb::IntoProxied<Course> for CourseMut<'msg> {
  fn into_proxied(self, _private: ::__pb::__internal::Private) -> Course {
    ::__pb::IntoProxied::into_proxied(::__pb::IntoView::into_view(self), _private)
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Course {
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
impl ::__pb::__runtime::UpbTypeConversions for Course {
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
        CourseView::new(
            ::__pb::__internal::Private,
            unsafe { msg.msg_val }
                .expect("expected present message value in map"))
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CourseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl<'msg> ::__pb::__internal::SealedInternal for CourseMut<'msg> {}

impl<'msg> ::__pb::MessageMut<'msg> for CourseMut<'msg> {
  type Message = Course;
}

impl ::__std::fmt::Debug for CourseMut<'_> {
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

impl ::__pb::Serialize for CourseMut<'_> {
  fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
    ::__pb::AsView::as_view(self).serialize()
  }
}

impl ::__pb::Clear for CourseMut<'_> {
  fn clear(&mut self) {
    unsafe {
      ::__pb::__runtime::upb_Message_Clear(
          self.raw_msg(),
          <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
    }
  }
}

impl ::__pb::MergeFrom for CourseMut<'_> {
  fn merge_from(&mut self, src: impl ::__pb::AsView<Proxied = Course>) {
    // SAFETY: self and src are both valid `Course`s.
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
impl<'msg> CourseMut<'msg> {
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

  pub fn to_owned(&self) -> Course {
    ::__pb::AsView::as_view(self).to_owned()
  }

  fn arena(&self) -> &::__pb::__runtime::Arena {
    self.inner.arena()
  }

  // name: optional string
  pub fn name(&self) -> ::__pb::View<'_, ::__pb::ProtoString> {
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
  pub fn set_name(&mut self, val: impl ::__pb::IntoProxied<::__pb::ProtoString>) {
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

  // authors: repeated string
  pub fn authors(&self) -> ::__pb::RepeatedView<'_, ::__pb::ProtoString> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1);
      ::__pb::__runtime::upb_Message_GetArray(
        self.raw_msg(), f)
    }.map_or_else(
        ::__pb::__runtime::empty_array::<::__pb::ProtoString>,
        |raw| unsafe {
          ::__pb::RepeatedView::from_raw(::__pb::__internal::Private, raw)
        }
      )
  }
  pub fn authors_mut(&mut self) -> ::__pb::RepeatedMut<'_, ::__pb::ProtoString> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1);
      let raw_array = ::__pb::__runtime::upb_Message_GetOrCreateMutableArray(
            self.raw_msg(),
            f,
            self.arena().raw(),
          ).unwrap();
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          raw_array, self.arena(),
        ),
      )
    }
  }
  pub fn set_authors(&mut self, src: impl ::__pb::IntoProxied<::__pb::Repeated<::__pb::ProtoString>>) {
    let minitable_field = unsafe {
      ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1
      )
    };
    let val = src.into_proxied(::__pb::__internal::Private);
    let inner = val.inner(::__pb::__internal::Private);

    self.arena().fuse(inner.arena());
    unsafe {
        let value_ptr: *const *const std::ffi::c_void =
            &(inner.raw().as_ptr() as *const std::ffi::c_void);
        ::__pb::__runtime::upb_Message_SetBaseField(self.raw_msg(),
          minitable_field,
          value_ptr as *const std::ffi::c_void);
    }
  }

  // lectures: repeated message myfirstproto.mooc.Course.LecturesEntry
  pub fn lectures(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoString, crate::course::Lecture> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        2);
      ::__pb::__runtime::upb_Message_GetMap(self.raw_msg(), f)
        .map_or_else(
          ::__pb::__runtime::empty_map::<::__pb::ProtoString, crate::course::Lecture>,
          |raw| ::__pb::MapView::from_raw(::__pb::__internal::Private, raw)
        )
    }
  }
  pub fn lectures_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoString, crate::course::Lecture> {
    unsafe {
      let parent_mini_table =
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();

      let f =
        ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            parent_mini_table,
            2);

      let map_entry_mini_table =
        ::__pb::__runtime::upb_MiniTable_SubMessage(
            parent_mini_table,
            f);

      let raw_map =
        ::__pb::__runtime::upb_Message_GetOrCreateMutableMap(
            self.raw_msg(),
            map_entry_mini_table,
            f,
            self.arena().raw()).unwrap();
      let inner = ::__pb::__runtime::InnerMapMut::new(
        raw_map, self.arena());
      ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner)
    }
  }
  pub fn set_lectures(&mut self, src: impl ::__pb::IntoProxied<::__pb::Map<::__pb::ProtoString, crate::course::Lecture>>) {
    // TODO: b/355493062 - Fix this extra copy.
    self.lectures_mut().copy_from(src.into_proxied(::__pb::__internal::Private).as_view());
  }

}

// SAFETY:
// - `CourseMut` does not perform any shared mutation.
// - `CourseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CourseMut<'_> {}

impl<'msg> ::__pb::Proxy<'msg> for CourseMut<'msg> {}
impl<'msg> ::__pb::MutProxy<'msg> for CourseMut<'msg> {}

impl<'msg> ::__pb::AsView for CourseMut<'msg> {
  type Proxied = Course;
  fn as_view(&self) -> ::__pb::View<'_, Course> {
    CourseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

impl<'msg> ::__pb::IntoView<'msg> for CourseMut<'msg> {
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Course>
  where
      'msg: 'shorter {
    CourseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

impl<'msg> ::__pb::AsMut for CourseMut<'msg> {
  type MutProxied = Course;
  fn as_mut(&mut self) -> CourseMut<'msg> {
    CourseMut { inner: self.inner }
  }
}

impl<'msg> ::__pb::IntoMut<'msg> for CourseMut<'msg> {
  fn into_mut<'shorter>(self) -> CourseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Course {
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

  pub fn as_view(&self) -> CourseView {
    CourseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CourseMut {
    CourseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // name: optional string
  pub fn name(&self) -> ::__pb::View<'_, ::__pb::ProtoString> {
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
  pub fn set_name(&mut self, val: impl ::__pb::IntoProxied<::__pb::ProtoString>) {
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

  // authors: repeated string
  pub fn authors(&self) -> ::__pb::RepeatedView<'_, ::__pb::ProtoString> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1);
      ::__pb::__runtime::upb_Message_GetArray(
        self.raw_msg(), f)
    }.map_or_else(
        ::__pb::__runtime::empty_array::<::__pb::ProtoString>,
        |raw| unsafe {
          ::__pb::RepeatedView::from_raw(::__pb::__internal::Private, raw)
        }
      )
  }
  pub fn authors_mut(&mut self) -> ::__pb::RepeatedMut<'_, ::__pb::ProtoString> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1);
      let raw_array = ::__pb::__runtime::upb_Message_GetOrCreateMutableArray(
            self.raw_msg(),
            f,
            self.arena().raw(),
          ).unwrap();
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          raw_array, self.arena(),
        ),
      )
    }
  }
  pub fn set_authors(&mut self, src: impl ::__pb::IntoProxied<::__pb::Repeated<::__pb::ProtoString>>) {
    let minitable_field = unsafe {
      ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        1
      )
    };
    let val = src.into_proxied(::__pb::__internal::Private);
    let inner = val.inner(::__pb::__internal::Private);

    self.arena().fuse(inner.arena());
    unsafe {
        let value_ptr: *const *const std::ffi::c_void =
            &(inner.raw().as_ptr() as *const std::ffi::c_void);
        ::__pb::__runtime::upb_Message_SetBaseField(self.raw_msg(),
          minitable_field,
          value_ptr as *const std::ffi::c_void);
    }
  }

  // lectures: repeated message myfirstproto.mooc.Course.LecturesEntry
  pub fn lectures(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoString, crate::course::Lecture> {
    unsafe {
      let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        2);
      ::__pb::__runtime::upb_Message_GetMap(self.raw_msg(), f)
        .map_or_else(
          ::__pb::__runtime::empty_map::<::__pb::ProtoString, crate::course::Lecture>,
          |raw| ::__pb::MapView::from_raw(::__pb::__internal::Private, raw)
        )
    }
  }
  pub fn lectures_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoString, crate::course::Lecture> {
    unsafe {
      let parent_mini_table =
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();

      let f =
        ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            parent_mini_table,
            2);

      let map_entry_mini_table =
        ::__pb::__runtime::upb_MiniTable_SubMessage(
            parent_mini_table,
            f);

      let raw_map =
        ::__pb::__runtime::upb_Message_GetOrCreateMutableMap(
            self.raw_msg(),
            map_entry_mini_table,
            f,
            self.arena().raw()).unwrap();
      let inner = ::__pb::__runtime::InnerMapMut::new(
        raw_map, self.arena());
      ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner)
    }
  }
  pub fn set_lectures(&mut self, src: impl ::__pb::IntoProxied<::__pb::Map<::__pb::ProtoString, crate::course::Lecture>>) {
    // TODO: b/355493062 - Fix this extra copy.
    self.lectures_mut().copy_from(src.into_proxied(::__pb::__internal::Private).as_view());
  }

}  // impl Course

impl ::__std::ops::Drop for Course {
  fn drop(&mut self) {
  }
}

impl ::__std::clone::Clone for Course {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::__pb::AsView for Course {
  type Proxied = Self;
  fn as_view(&self) -> CourseView {
    self.as_view()
  }
}

impl ::__pb::AsMut for Course {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CourseMut {
    self.as_mut()
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for Course {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstproto__mooc__Course_msg_init)
    }
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for CourseView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstproto__mooc__Course_msg_init)
    }
  }
}

unsafe impl ::__pb::__runtime::AssociatedMiniTable for CourseMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
    // This is unsafe only for Rust 1.80 and below and thus can be dropped
    // once our MSRV is 1.81+
    #[allow(unused_unsafe)]
    unsafe {
      ::__std::ptr::addr_of!(myfirstproto__mooc__Course_msg_init)
    }
  }
}

pub mod course {
  #[allow(non_camel_case_types)]
  pub struct Lecture {
    inner: ::__pb::__runtime::MessageInner
  }

  impl ::__pb::Message for Lecture {}

  impl ::__std::default::Default for Lecture {
    fn default() -> Self {
      Self::new()
    }
  }

  impl ::__pb::Parse for Lecture {
    fn parse(serialized: &[u8]) -> ::__std::result::Result<Self, ::__pb::ParseError> {
      Self::parse(serialized)
    }
  }

  impl ::__std::fmt::Debug for Lecture {
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

  impl ::__pb::MergeFrom for Lecture {
    fn merge_from<'src>(&mut self, src: impl ::__pb::AsView<Proxied = Self>) {
      let mut m = self.as_mut();
      ::__pb::MergeFrom::merge_from(&mut m, src)
    }
  }

  impl ::__pb::Serialize for Lecture {
    fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
      ::__pb::AsView::as_view(self).serialize()
    }
  }

  impl ::__pb::Clear for Lecture {
    fn clear(&mut self) {
      self.as_mut().clear()
    }
  }

  impl ::__pb::ClearAndParse for Lecture {
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
  // - `Lecture` is `Sync` because it does not implement interior mutability.
  //    Neither does `LectureMut`.
  unsafe impl Sync for Lecture {}

  // SAFETY:
  // - `Lecture` is `Send` because it uniquely owns its arena and does
  //   not use thread-local data.
  unsafe impl Send for Lecture {}

  impl ::__pb::Proxied for Lecture {
    type View<'msg> = LectureView<'msg>;
  }

  impl ::__pb::__internal::SealedInternal for Lecture {}

  impl ::__pb::MutProxied for Lecture {
    type Mut<'msg> = LectureMut<'msg>;
  }

  #[derive(Copy, Clone)]
  #[allow(dead_code)]
  pub struct LectureView<'msg> {
    msg: ::__pb::__runtime::RawMessage,
    _phantom: ::__std::marker::PhantomData<&'msg ()>,
  }

  impl<'msg> ::__pb::__internal::SealedInternal for LectureView<'msg> {}

  impl<'msg> ::__pb::MessageView<'msg> for LectureView<'msg> {
    type Message = Lecture;
  }

  impl ::__std::fmt::Debug for LectureView<'_> {
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

  impl ::__pb::Serialize for LectureView<'_> {
    fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
      // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
      let encoded = unsafe {
        ::__pb::__runtime::wire::encode(self.raw_msg(),
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
      };
      encoded.map_err(|_| ::__pb::SerializeError)
    }
  }

  impl ::__std::default::Default for LectureView<'_> {
    fn default() -> LectureView<'static> {
      LectureView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block())
    }
  }

  #[allow(dead_code)]
  impl<'msg> LectureView<'msg> {
    #[doc(hidden)]
    pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
      Self { msg, _phantom: ::__std::marker::PhantomData }
    }

    fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
      self.msg
    }

    pub fn to_owned(&self) -> Lecture {
      ::__pb::IntoProxied::into_proxied(*self, ::__pb::__internal::Private)
    }

    // video: optional message myfirstprotobuf.mooc.content.Video
    pub fn has_video(self) -> bool {
      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0);
        ::__pb::__runtime::upb_Message_HasBaseField(self.raw_msg(), f)
      }
    }
    pub fn video_opt(self) -> ::__pb::Optional<crate::VideoView<'msg>> {
          ::__pb::Optional::new(self.video(), self.has_video())
    }
    pub fn video(self) -> crate::VideoView<'msg> {
      let submsg = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                    <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                    0);
        ::__pb::__runtime::upb_Message_GetMessage(self.raw_msg(), f)
      };
      match submsg {
        None => crate::VideoView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(sub_raw_msg) => crate::VideoView::new(::__pb::__internal::Private, sub_raw_msg),
      }
    }

    // article: optional message myfirstprotobuf.mooc.content.Article
    pub fn has_article(self) -> bool {
      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            1);
        ::__pb::__runtime::upb_Message_HasBaseField(self.raw_msg(), f)
      }
    }
    pub fn article_opt(self) -> ::__pb::Optional<crate::ArticleView<'msg>> {
          ::__pb::Optional::new(self.article(), self.has_article())
    }
    pub fn article(self) -> crate::ArticleView<'msg> {
      let submsg = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                    <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                    1);
        ::__pb::__runtime::upb_Message_GetMessage(self.raw_msg(), f)
      };
      match submsg {
        None => crate::ArticleView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(sub_raw_msg) => crate::ArticleView::new(::__pb::__internal::Private, sub_raw_msg),
      }
    }

    pub fn content(self) -> crate::course::lecture::Content<'msg> {
      match self.content_case() {
        crate::course::lecture::ContentCase::Video =>
            crate::course::lecture::Content::Video(self.video()),
        crate::course::lecture::ContentCase::Article =>
            crate::course::lecture::Content::Article(self.article()),
        _ => crate::course::lecture::Content::not_set(std::marker::PhantomData)
      }
    }

    pub fn content_case(self) -> crate::course::lecture::ContentCase {
      let field_num = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0);
        ::__pb::__runtime::upb_Message_WhichOneofFieldNumber(
              self.raw_msg(), f)
      };
      unsafe {
        crate::course::lecture::ContentCase::try_from(field_num).unwrap_unchecked()
      }
    }
  }

  // SAFETY:
  // - `LectureView` is `Sync` because it does not support mutation.
  unsafe impl Sync for LectureView<'_> {}

  // SAFETY:
  // - `LectureView` is `Send` because while its alive a `LectureMut` cannot.
  // - `LectureView` does not use thread-local data.
  unsafe impl Send for LectureView<'_> {}

  impl<'msg> ::__pb::Proxy<'msg> for LectureView<'msg> {}
  impl<'msg> ::__pb::ViewProxy<'msg> for LectureView<'msg> {}

  impl<'msg> ::__pb::AsView for LectureView<'msg> {
    type Proxied = Lecture;
    fn as_view(&self) -> ::__pb::View<'msg, Lecture> {
      *self
    }
  }

  impl<'msg> ::__pb::IntoView<'msg> for LectureView<'msg> {
    fn into_view<'shorter>(self) -> LectureView<'shorter>
    where
        'msg: 'shorter {
      self
    }
  }

  impl<'msg> ::__pb::IntoProxied<Lecture> for LectureView<'msg> {
    fn into_proxied(self, _private: ::__pb::__internal::Private) -> Lecture {
      let dst = Lecture::new();
      unsafe { ::__pb::__runtime::upb_Message_DeepCopy(
        dst.inner.msg,
        self.msg,
        <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
        dst.inner.arena.raw(),
      ) };
      dst
    }
  }

  impl<'msg> ::__pb::IntoProxied<Lecture> for LectureMut<'msg> {
    fn into_proxied(self, _private: ::__pb::__internal::Private) -> Lecture {
      ::__pb::IntoProxied::into_proxied(::__pb::IntoView::into_view(self), _private)
    }
  }

  unsafe impl ::__pb::ProxiedInRepeated for Lecture {
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
  impl ::__pb::__runtime::UpbTypeConversions for Lecture {
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
          LectureView::new(
              ::__pb::__internal::Private,
              unsafe { msg.msg_val }
                  .expect("expected present message value in map"))
      }
  }

  #[allow(dead_code)]
  #[allow(non_camel_case_types)]
  pub struct LectureMut<'msg> {
    inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
  }

  impl<'msg> ::__pb::__internal::SealedInternal for LectureMut<'msg> {}

  impl<'msg> ::__pb::MessageMut<'msg> for LectureMut<'msg> {
    type Message = Lecture;
  }

  impl ::__std::fmt::Debug for LectureMut<'_> {
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

  impl ::__pb::Serialize for LectureMut<'_> {
    fn serialize(&self) -> ::__std::result::Result<Vec<u8>, ::__pb::SerializeError> {
      ::__pb::AsView::as_view(self).serialize()
    }
  }

  impl ::__pb::Clear for LectureMut<'_> {
    fn clear(&mut self) {
      unsafe {
        ::__pb::__runtime::upb_Message_Clear(
            self.raw_msg(),
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table())
      }
    }
  }

  impl ::__pb::MergeFrom for LectureMut<'_> {
    fn merge_from(&mut self, src: impl ::__pb::AsView<Proxied = Lecture>) {
      // SAFETY: self and src are both valid `Lecture`s.
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
  impl<'msg> LectureMut<'msg> {
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

    pub fn to_owned(&self) -> Lecture {
      ::__pb::AsView::as_view(self).to_owned()
    }

    fn arena(&self) -> &::__pb::__runtime::Arena {
      self.inner.arena()
    }

    // video: optional message myfirstprotobuf.mooc.content.Video
    pub fn has_video(&self) -> bool {
      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0);
        ::__pb::__runtime::upb_Message_HasBaseField(self.raw_msg(), f)
      }
    }
    pub fn clear_video(&mut self) {
      unsafe {
        let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            mt, 0);
        ::__pb::__runtime::upb_Message_ClearBaseField(self.raw_msg(), f);
      }
    }
    pub fn video_opt(&self) -> ::__pb::Optional<crate::VideoView<'_>> {
          ::__pb::Optional::new(self.video(), self.has_video())
    }
    pub fn video(&self) -> crate::VideoView<'_> {
      let submsg = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                    <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                    0);
        ::__pb::__runtime::upb_Message_GetMessage(self.raw_msg(), f)
      };
      match submsg {
        None => crate::VideoView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(sub_raw_msg) => crate::VideoView::new(::__pb::__internal::Private, sub_raw_msg),
      }
    }
    pub fn video_mut(&mut self) -> crate::VideoMut<'_> {
       let raw_msg = unsafe {
         let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
         let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(mt, 0);
         ::__pb::__runtime::upb_Message_GetOrCreateMutableMessage(
             self.raw_msg(), mt, f, self.arena().raw()).unwrap()
       };
       crate::VideoMut::from_parent(::__pb::__internal::Private,
         self.as_mutator_message_ref(::__pb::__internal::Private), raw_msg)
    }
    pub fn set_video(&mut self,
      val: impl ::__pb::IntoProxied<crate::Video>) {

      // The message and arena are dropped after the setter. The
      // memory remains allocated as we fuse the arena with the
      // parent message's arena.
      let mut msg = val.into_proxied(::__pb::__internal::Private);
      self.as_mutator_message_ref(::__pb::__internal::Private)
        .arena()
        .fuse(msg.as_mutator_message_ref(::__pb::__internal::Private).arena());

      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                  0);
        ::__pb::__runtime::upb_Message_SetBaseFieldMessage(
          self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
          f,
          msg.as_mutator_message_ref(::__pb::__internal::Private).msg());
      }
    }

    // article: optional message myfirstprotobuf.mooc.content.Article
    pub fn has_article(&self) -> bool {
      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            1);
        ::__pb::__runtime::upb_Message_HasBaseField(self.raw_msg(), f)
      }
    }
    pub fn clear_article(&mut self) {
      unsafe {
        let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            mt, 1);
        ::__pb::__runtime::upb_Message_ClearBaseField(self.raw_msg(), f);
      }
    }
    pub fn article_opt(&self) -> ::__pb::Optional<crate::ArticleView<'_>> {
          ::__pb::Optional::new(self.article(), self.has_article())
    }
    pub fn article(&self) -> crate::ArticleView<'_> {
      let submsg = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                    <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                    1);
        ::__pb::__runtime::upb_Message_GetMessage(self.raw_msg(), f)
      };
      match submsg {
        None => crate::ArticleView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(sub_raw_msg) => crate::ArticleView::new(::__pb::__internal::Private, sub_raw_msg),
      }
    }
    pub fn article_mut(&mut self) -> crate::ArticleMut<'_> {
       let raw_msg = unsafe {
         let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
         let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(mt, 1);
         ::__pb::__runtime::upb_Message_GetOrCreateMutableMessage(
             self.raw_msg(), mt, f, self.arena().raw()).unwrap()
       };
       crate::ArticleMut::from_parent(::__pb::__internal::Private,
         self.as_mutator_message_ref(::__pb::__internal::Private), raw_msg)
    }
    pub fn set_article(&mut self,
      val: impl ::__pb::IntoProxied<crate::Article>) {

      // The message and arena are dropped after the setter. The
      // memory remains allocated as we fuse the arena with the
      // parent message's arena.
      let mut msg = val.into_proxied(::__pb::__internal::Private);
      self.as_mutator_message_ref(::__pb::__internal::Private)
        .arena()
        .fuse(msg.as_mutator_message_ref(::__pb::__internal::Private).arena());

      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                  1);
        ::__pb::__runtime::upb_Message_SetBaseFieldMessage(
          self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
          f,
          msg.as_mutator_message_ref(::__pb::__internal::Private).msg());
      }
    }

    pub fn content(&self) -> crate::course::lecture::Content<'_> {
      match &self.content_case() {
        crate::course::lecture::ContentCase::Video =>
            crate::course::lecture::Content::Video(self.video()),
        crate::course::lecture::ContentCase::Article =>
            crate::course::lecture::Content::Article(self.article()),
        _ => crate::course::lecture::Content::not_set(std::marker::PhantomData)
      }
    }

    pub fn content_case(&self) -> crate::course::lecture::ContentCase {
      let field_num = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0);
        ::__pb::__runtime::upb_Message_WhichOneofFieldNumber(
              self.raw_msg(), f)
      };
      unsafe {
        crate::course::lecture::ContentCase::try_from(field_num).unwrap_unchecked()
      }
    }
  }

  // SAFETY:
  // - `LectureMut` does not perform any shared mutation.
  // - `LectureMut` is not `Send`, and so even in the presence of mutator
  //   splitting, synchronous access of an arena is impossible.
  unsafe impl Sync for LectureMut<'_> {}

  impl<'msg> ::__pb::Proxy<'msg> for LectureMut<'msg> {}
  impl<'msg> ::__pb::MutProxy<'msg> for LectureMut<'msg> {}

  impl<'msg> ::__pb::AsView for LectureMut<'msg> {
    type Proxied = Lecture;
    fn as_view(&self) -> ::__pb::View<'_, Lecture> {
      LectureView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
    }
  }

  impl<'msg> ::__pb::IntoView<'msg> for LectureMut<'msg> {
    fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Lecture>
    where
        'msg: 'shorter {
      LectureView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
    }
  }

  impl<'msg> ::__pb::AsMut for LectureMut<'msg> {
    type MutProxied = Lecture;
    fn as_mut(&mut self) -> LectureMut<'msg> {
      LectureMut { inner: self.inner }
    }
  }

  impl<'msg> ::__pb::IntoMut<'msg> for LectureMut<'msg> {
    fn into_mut<'shorter>(self) -> LectureMut<'shorter>
    where
        'msg: 'shorter {
      self
    }
  }

  #[allow(dead_code)]
  impl Lecture {
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

    pub fn as_view(&self) -> LectureView {
      LectureView::new(::__pb::__internal::Private, self.inner.msg)
    }

    pub fn as_mut(&mut self) -> LectureMut {
      LectureMut::new(::__pb::__internal::Private, &mut self.inner)
    }

    // video: optional message myfirstprotobuf.mooc.content.Video
    pub fn has_video(&self) -> bool {
      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0);
        ::__pb::__runtime::upb_Message_HasBaseField(self.raw_msg(), f)
      }
    }
    pub fn clear_video(&mut self) {
      unsafe {
        let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            mt, 0);
        ::__pb::__runtime::upb_Message_ClearBaseField(self.raw_msg(), f);
      }
    }
    pub fn video_opt(&self) -> ::__pb::Optional<crate::VideoView<'_>> {
          ::__pb::Optional::new(self.video(), self.has_video())
    }
    pub fn video(&self) -> crate::VideoView<'_> {
      let submsg = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                    <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                    0);
        ::__pb::__runtime::upb_Message_GetMessage(self.raw_msg(), f)
      };
      match submsg {
        None => crate::VideoView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(sub_raw_msg) => crate::VideoView::new(::__pb::__internal::Private, sub_raw_msg),
      }
    }
    pub fn video_mut(&mut self) -> crate::VideoMut<'_> {
       let raw_msg = unsafe {
         let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
         let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(mt, 0);
         ::__pb::__runtime::upb_Message_GetOrCreateMutableMessage(
             self.raw_msg(), mt, f, self.arena().raw()).unwrap()
       };
       crate::VideoMut::from_parent(::__pb::__internal::Private,
         self.as_mutator_message_ref(::__pb::__internal::Private), raw_msg)
    }
    pub fn set_video(&mut self,
      val: impl ::__pb::IntoProxied<crate::Video>) {

      // The message and arena are dropped after the setter. The
      // memory remains allocated as we fuse the arena with the
      // parent message's arena.
      let mut msg = val.into_proxied(::__pb::__internal::Private);
      self.as_mutator_message_ref(::__pb::__internal::Private)
        .arena()
        .fuse(msg.as_mutator_message_ref(::__pb::__internal::Private).arena());

      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                  0);
        ::__pb::__runtime::upb_Message_SetBaseFieldMessage(
          self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
          f,
          msg.as_mutator_message_ref(::__pb::__internal::Private).msg());
      }
    }

    // article: optional message myfirstprotobuf.mooc.content.Article
    pub fn has_article(&self) -> bool {
      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            1);
        ::__pb::__runtime::upb_Message_HasBaseField(self.raw_msg(), f)
      }
    }
    pub fn clear_article(&mut self) {
      unsafe {
        let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            mt, 1);
        ::__pb::__runtime::upb_Message_ClearBaseField(self.raw_msg(), f);
      }
    }
    pub fn article_opt(&self) -> ::__pb::Optional<crate::ArticleView<'_>> {
          ::__pb::Optional::new(self.article(), self.has_article())
    }
    pub fn article(&self) -> crate::ArticleView<'_> {
      let submsg = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                    <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                    1);
        ::__pb::__runtime::upb_Message_GetMessage(self.raw_msg(), f)
      };
      match submsg {
        None => crate::ArticleView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(sub_raw_msg) => crate::ArticleView::new(::__pb::__internal::Private, sub_raw_msg),
      }
    }
    pub fn article_mut(&mut self) -> crate::ArticleMut<'_> {
       let raw_msg = unsafe {
         let mt = <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table();
         let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(mt, 1);
         ::__pb::__runtime::upb_Message_GetOrCreateMutableMessage(
             self.raw_msg(), mt, f, self.arena().raw()).unwrap()
       };
       crate::ArticleMut::from_parent(::__pb::__internal::Private,
         self.as_mutator_message_ref(::__pb::__internal::Private), raw_msg)
    }
    pub fn set_article(&mut self,
      val: impl ::__pb::IntoProxied<crate::Article>) {

      // The message and arena are dropped after the setter. The
      // memory remains allocated as we fuse the arena with the
      // parent message's arena.
      let mut msg = val.into_proxied(::__pb::__internal::Private);
      self.as_mutator_message_ref(::__pb::__internal::Private)
        .arena()
        .fuse(msg.as_mutator_message_ref(::__pb::__internal::Private).arena());

      unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
                  <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
                  1);
        ::__pb::__runtime::upb_Message_SetBaseFieldMessage(
          self.as_mutator_message_ref(::__pb::__internal::Private).msg(),
          f,
          msg.as_mutator_message_ref(::__pb::__internal::Private).msg());
      }
    }

    pub fn content(&self) -> crate::course::lecture::Content<'_> {
      match &self.content_case() {
        crate::course::lecture::ContentCase::Video =>
            crate::course::lecture::Content::Video(self.video()),
        crate::course::lecture::ContentCase::Article =>
            crate::course::lecture::Content::Article(self.article()),
        _ => crate::course::lecture::Content::not_set(std::marker::PhantomData)
      }
    }

    pub fn content_case(&self) -> crate::course::lecture::ContentCase {
      let field_num = unsafe {
        let f = ::__pb::__runtime::upb_MiniTable_GetFieldByIndex(
            <Self as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0);
        ::__pb::__runtime::upb_Message_WhichOneofFieldNumber(
              self.raw_msg(), f)
      };
      unsafe {
        crate::course::lecture::ContentCase::try_from(field_num).unwrap_unchecked()
      }
    }
  }  // impl Lecture

  impl ::__std::ops::Drop for Lecture {
    fn drop(&mut self) {
    }
  }

  impl ::__std::clone::Clone for Lecture {
    fn clone(&self) -> Self {
      self.as_view().to_owned()
    }
  }

  impl ::__pb::AsView for Lecture {
    type Proxied = Self;
    fn as_view(&self) -> LectureView {
      self.as_view()
    }
  }

  impl ::__pb::AsMut for Lecture {
    type MutProxied = Self;
    fn as_mut(&mut self) -> LectureMut {
      self.as_mut()
    }
  }

  unsafe impl ::__pb::__runtime::AssociatedMiniTable for Lecture {
    #[inline(always)]
    fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
      // This is unsafe only for Rust 1.80 and below and thus can be dropped
      // once our MSRV is 1.81+
      #[allow(unused_unsafe)]
      unsafe {
        ::__std::ptr::addr_of!(myfirstproto__mooc__Course__Lecture_msg_init)
      }
    }
  }

  unsafe impl ::__pb::__runtime::AssociatedMiniTable for LectureView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
      // This is unsafe only for Rust 1.80 and below and thus can be dropped
      // once our MSRV is 1.81+
      #[allow(unused_unsafe)]
      unsafe {
        ::__std::ptr::addr_of!(myfirstproto__mooc__Course__Lecture_msg_init)
      }
    }
  }

  unsafe impl ::__pb::__runtime::AssociatedMiniTable for LectureMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::__pb::__runtime::upb_MiniTable {
      // This is unsafe only for Rust 1.80 and below and thus can be dropped
      // once our MSRV is 1.81+
      #[allow(unused_unsafe)]
      unsafe {
        ::__std::ptr::addr_of!(myfirstproto__mooc__Course__Lecture_msg_init)
      }
    }
  }

  pub mod lecture {

    #[non_exhaustive]
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    #[repr(isize)]
    pub enum Content<'msg> {
      Video(::__pb::View<'msg, crate::Video>) = 1,
      Article(::__pb::View<'msg, crate::Article>) = 2,

      #[allow(non_camel_case_types)]
      not_set(std::marker::PhantomData<&'msg ()>) = 0
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[allow(dead_code)]
    pub enum ContentCase {
      Video = 1,
      Article = 2,

      #[allow(non_camel_case_types)]
      not_set = 0
    }

    impl ContentCase {
      #[allow(dead_code)]
      pub(crate) fn try_from(v: u32) -> ::__std::option::Option<ContentCase> {
        match v {
          0 => Some(ContentCase::not_set),
          1 => Some(ContentCase::Video),
          2 => Some(ContentCase::Article),
          _ => None
        }
      }
    }
  }  // mod lecture
  extern "C" {
    /// Opaque static extern for this message's MiniTable, generated
    /// by the upb C MiniTable codegen. The only valid way to
    /// reference this static is with `std::ptr::addr_of!(..)`.
    static myfirstproto__mooc__Course__Lecture_msg_init: ::__pb::__runtime::upb_MiniTable;
  }

  // upb kernel doesn't support any owned message or message mut interop.
  impl ::__pb::OwnedMessageInterop for Lecture {}
  impl<'a> ::__pb::MessageMutInterop<'a> for LectureMut<'a> {}

  impl<'a> ::__pb::MessageViewInterop<'a> for LectureView<'a> {
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

  impl ::__pb::__internal::MatcherEq for Lecture {
    fn matches(&self, o: &Self) -> bool {
      ::__pb::__internal::MatcherEq::matches(
        &::__pb::AsView::as_view(self),
        &::__pb::AsView::as_view(o))
    }
  }

  impl<'a> ::__pb::__internal::MatcherEq for LectureMut<'a> {
    fn matches(&self, o: &Self) -> bool {
      ::__pb::__internal::MatcherEq::matches(
        &::__pb::AsView::as_view(self),
        &::__pb::AsView::as_view(o))
    }
  }

  impl<'a> ::__pb::__internal::MatcherEq for LectureView<'a> {
    fn matches(&self, o: &Self) -> bool {
      unsafe {
        ::__pb::__runtime::upb_Message_IsEqual(
            self.msg,
            o.msg,
            <Lecture as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
            0)
      }
    }
  }

}  // mod course
extern "C" {
  /// Opaque static extern for this message's MiniTable, generated
  /// by the upb C MiniTable codegen. The only valid way to
  /// reference this static is with `std::ptr::addr_of!(..)`.
  static myfirstproto__mooc__Course_msg_init: ::__pb::__runtime::upb_MiniTable;
}

// upb kernel doesn't support any owned message or message mut interop.
impl ::__pb::OwnedMessageInterop for Course {}
impl<'a> ::__pb::MessageMutInterop<'a> for CourseMut<'a> {}

impl<'a> ::__pb::MessageViewInterop<'a> for CourseView<'a> {
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

impl ::__pb::__internal::MatcherEq for Course {
  fn matches(&self, o: &Self) -> bool {
    ::__pb::__internal::MatcherEq::matches(
      &::__pb::AsView::as_view(self),
      &::__pb::AsView::as_view(o))
  }
}

impl<'a> ::__pb::__internal::MatcherEq for CourseMut<'a> {
  fn matches(&self, o: &Self) -> bool {
    ::__pb::__internal::MatcherEq::matches(
      &::__pb::AsView::as_view(self),
      &::__pb::AsView::as_view(o))
  }
}

impl<'a> ::__pb::__internal::MatcherEq for CourseView<'a> {
  fn matches(&self, o: &Self) -> bool {
    unsafe {
      ::__pb::__runtime::upb_Message_IsEqual(
          self.msg,
          o.msg,
          <Course as ::__pb::__runtime::AssociatedMiniTable>::mini_table(),
          0)
    }
  }
}

