// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
pub mod wired {
    pub mod ecs {
        #[allow(clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ComponentInstance {
                handle: _rt::Resource<ComponentInstance>,
            }

            impl ComponentInstance {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for ComponentInstance {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]component-instance"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Component {
                handle: _rt::Resource<Component>,
            }

            impl Component {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Component {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]component"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Entity {
                handle: _rt::Resource<Entity>,
            }

            impl Entity {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Entity {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]entity"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Query {
                handle: _rt::Resource<Query>,
            }

            impl Query {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Query {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]query"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct EcsWorld {
                handle: _rt::Resource<EcsWorld>,
            }

            impl EcsWorld {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for EcsWorld {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]ecs-world"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            impl Component {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(&self) -> ComponentInstance {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[method]component.new"]
                            fn wit_import(_: i32) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ComponentInstance::from_handle(ret as u32)
                    }
                }
            }
            impl Entity {
                #[allow(unused_unsafe, clippy::all)]
                pub fn insert(&self, component: ComponentInstance) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[method]entity.insert"]
                            fn wit_import(_: i32, _: i32);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, (&component).take_handle() as i32);
                    }
                }
            }
            impl Query {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(&self) -> _rt::Vec<(Entity, _rt::Vec<ComponentInstance>)> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[method]query.read"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base8 = l1;
                        let len8 = l2;
                        let mut result8 = _rt::Vec::with_capacity(len8);
                        for i in 0..len8 {
                            let base = base8.add(i * 12);
                            let e8 = {
                                let l3 = *base.add(0).cast::<i32>();
                                let l4 = *base.add(4).cast::<*mut u8>();
                                let l5 = *base.add(8).cast::<usize>();
                                let base7 = l4;
                                let len7 = l5;
                                let mut result7 = _rt::Vec::with_capacity(len7);
                                for i in 0..len7 {
                                    let base = base7.add(i * 4);
                                    let e7 = {
                                        let l6 = *base.add(0).cast::<i32>();

                                        ComponentInstance::from_handle(l6 as u32)
                                    };
                                    result7.push(e7);
                                }
                                _rt::cabi_dealloc(base7, len7 * 4, 4);

                                (Entity::from_handle(l3 as u32), result7)
                            };
                            result8.push(e8);
                        }
                        _rt::cabi_dealloc(base8, len8 * 12, 4);
                        result8
                    }
                }
            }
            impl EcsWorld {
                #[allow(unused_unsafe, clippy::all)]
                pub fn register_component(&self) -> Component {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[method]ecs-world.register-component"]
                            fn wit_import(_: i32) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Component::from_handle(ret as u32)
                    }
                }
            }
            impl EcsWorld {
                #[allow(unused_unsafe, clippy::all)]
                pub fn register_query(&self, components: &[&Component]) -> Query {
                    unsafe {
                        let vec0 = components;
                        let len0 = vec0.len();
                        let layout0 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec0.len() * 4, 4);
                        let result0 = if layout0.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout0);
                            }
                            ptr
                        } else {
                            {
                                ::core::ptr::null_mut()
                            }
                        };
                        for (i, e) in vec0.into_iter().enumerate() {
                            let base = result0.add(i * 4);
                            {
                                *base.add(0).cast::<i32>() = (e).handle() as i32;
                            }
                        }

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[method]ecs-world.register-query"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, result0, len0);
                        if layout0.size() != 0 {
                            _rt::alloc::dealloc(result0.cast(), layout0);
                        }
                        Query::from_handle(ret as u32)
                    }
                }
            }
            impl EcsWorld {
                #[allow(unused_unsafe, clippy::all)]
                pub fn spawn(&self, components: _rt::Vec<ComponentInstance>) -> Entity {
                    unsafe {
                        let vec0 = &components;
                        let len0 = vec0.len();
                        let layout0 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec0.len() * 4, 4);
                        let result0 = if layout0.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout0);
                            }
                            ptr
                        } else {
                            {
                                ::core::ptr::null_mut()
                            }
                        };
                        for (i, e) in vec0.into_iter().enumerate() {
                            let base = result0.add(i * 4);
                            {
                                *base.add(0).cast::<i32>() = (e).take_handle() as i32;
                            }
                        }

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wired:ecs/types")]
                        extern "C" {
                            #[link_name = "[method]ecs-world.spawn"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, result0, len0);
                        if layout0.size() != 0 {
                            _rt::alloc::dealloc(result0.cast(), layout0);
                        }
                        Entity::from_handle(ret as u32)
                    }
                }
            }
        }
    }
    pub mod log {
        #[allow(clippy::all)]
        pub mod api {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum LogLevel {
                Debug,
                Info,
                Warn,
                Error,
            }
            impl ::core::fmt::Debug for LogLevel {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        LogLevel::Debug => f.debug_tuple("LogLevel::Debug").finish(),
                        LogLevel::Info => f.debug_tuple("LogLevel::Info").finish(),
                        LogLevel::Warn => f.debug_tuple("LogLevel::Warn").finish(),
                        LogLevel::Error => f.debug_tuple("LogLevel::Error").finish(),
                    }
                }
            }

            impl LogLevel {
                pub(crate) unsafe fn _lift(val: u8) -> LogLevel {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }

                    match val {
                        0 => LogLevel::Debug,
                        1 => LogLevel::Info,
                        2 => LogLevel::Warn,
                        3 => LogLevel::Error,

                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }

            #[allow(unused_unsafe, clippy::all)]
            pub fn log(level: LogLevel, message: &str) {
                unsafe {
                    let vec0 = message;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wired:log/api")]
                    extern "C" {
                        #[link_name = "log"]
                        fn wit_import(_: i32, _: *mut u8, _: usize);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(level.clone() as i32, ptr0.cast_mut(), len0);
                }
            }
        }
    }
}
pub mod exports {
    pub mod wired {
        pub mod script {
            #[allow(clippy::all)]
            pub mod lifecycle {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type EcsWorld = super::super::super::super::wired::ecs::types::EcsWorld;

                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Data {
                    handle: _rt::Resource<Data>,
                }

                type _DataRep<T> = Option<T>;

                impl Data {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Data`.
                    pub fn new<T: GuestData>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _DataRep<T> = Some(val);
                        let ptr: *mut _DataRep<T> = _rt::Box::into_raw(_rt::Box::new(val));
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }

                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestData>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }

                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestData>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }

                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestData>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }

                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }

                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }

                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }

                    // It's theoretically possible to implement the `GuestData` trait twice
                    // so guard against using it with two different types here.
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(!cfg!(target_feature = "threads"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => assert!(
                                    ty == id,
                                    "cannot use two types with this resource type"
                                ),
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }

                    fn as_ptr<T: GuestData>(&self) -> *mut _DataRep<T> {
                        Data::type_guard::<T>();
                        unsafe { T::_resource_rep(self.handle()).cast() }
                    }
                }

                /// A borrowed version of [`Data`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct DataBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Data>,
                }

                impl<'a> DataBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }

                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestData>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }

                    // NB: mutable access is not allowed due to the component model allowing
                    // multiple borrows of the same resource.

                    fn as_ptr<T: 'static>(&self) -> *mut _DataRep<T> {
                        Data::type_guard::<T>();
                        self.rep.cast()
                    }
                }

                unsafe impl _rt::WasmResource for Data {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]wired:script/lifecycle")]
                            extern "C" {
                                #[link_name = "[resource-drop]data"]
                                fn drop(_: u32);
                            }

                            drop(_handle);
                        }
                    }
                }

                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_init_cabi<T: Guest>(arg0: i32) -> i32 {
                    let handle0;
                    let result1 = T::init({
                        handle0 =
                            super::super::super::super::wired::ecs::types::EcsWorld::from_handle(
                                arg0 as u32,
                            );
                        &handle0
                    });
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_update_cabi<T: Guest>(arg0: i32, arg1: i32) {
                    let handle0;
                    T::update(
                        {
                            handle0 = super::super::super::super::wired::ecs::types::EcsWorld::from_handle(arg0 as u32);
                            &handle0
                        },
                        DataBorrow::lift(arg1 as u32 as usize),
                    );
                }
                pub trait Guest {
                    type Data: GuestData;
                    /// Called once to initialize the script.
                    fn init(ecs_world: &EcsWorld) -> Data;
                    /// Called every tick.
                    fn update(ecs_world: &EcsWorld, data: DataBorrow<'_>);
                }
                pub trait GuestData: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]wired:script/lifecycle")]
                            extern "C" {
                                #[link_name = "[resource-new]data"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }

                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]wired:script/lifecycle")]
                            extern "C" {
                                #[link_name = "[resource-rep]data"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                }
                #[doc(hidden)]

                macro_rules! __export_wired_script_lifecycle_cabi{
      ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

        #[export_name = "wired:script/lifecycle#init"]
        unsafe extern "C" fn export_init(arg0: i32,) -> i32 {
          $($path_to_types)*::_export_init_cabi::<$ty>(arg0)
        }
        #[export_name = "wired:script/lifecycle#update"]
        unsafe extern "C" fn export_update(arg0: i32,arg1: i32,) {
          $($path_to_types)*::_export_update_cabi::<$ty>(arg0, arg1)
        }
      };);
    }
                #[doc(hidden)]
                pub(crate) use __export_wired_script_lifecycle_cabi;
            }
        }
    }
}
mod _rt {

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::boxed::Box;
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_script_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::wired::script::lifecycle::__export_wired_script_lifecycle_cabi!($ty with_types_in $($path_to_types_root)*::exports::wired::script::lifecycle);
  )
}
#[doc(inline)]
pub(crate) use __export_script_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:script:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 800] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa3\x05\x01A\x02\x01\
A\x07\x01B\x1d\x04\0\x12component-instance\x03\x01\x04\0\x09component\x03\x01\x04\
\0\x06entity\x03\x01\x04\0\x05query\x03\x01\x04\0\x09ecs-world\x03\x01\x01h\x01\x01\
i\0\x01@\x01\x04self\x05\0\x06\x04\0\x15[method]component.new\x01\x07\x01h\x02\x01\
@\x02\x04self\x08\x09component\x06\x01\0\x04\0\x15[method]entity.insert\x01\x09\x01\
h\x03\x01i\x02\x01p\x06\x01o\x02\x0b\x0c\x01p\x0d\x01@\x01\x04self\x0a\0\x0e\x04\
\0\x12[method]query.read\x01\x0f\x01h\x04\x01i\x01\x01@\x01\x04self\x10\0\x11\x04\
\0$[method]ecs-world.register-component\x01\x12\x01p\x05\x01i\x03\x01@\x02\x04se\
lf\x10\x0acomponents\x13\0\x14\x04\0\x20[method]ecs-world.register-query\x01\x15\
\x01@\x02\x04self\x10\x0acomponents\x0c\0\x0b\x04\0\x17[method]ecs-world.spawn\x01\
\x16\x03\x01\x0fwired:ecs/types\x05\0\x01B\x04\x01m\x04\x05debug\x04info\x04warn\
\x05error\x04\0\x09log-level\x03\0\0\x01@\x02\x05level\x01\x07messages\x01\0\x04\
\0\x03log\x01\x02\x03\x01\x0dwired:log/api\x05\x01\x02\x03\0\0\x09ecs-world\x01B\
\x0a\x02\x03\x02\x01\x02\x04\0\x09ecs-world\x03\0\0\x04\0\x04data\x03\x01\x01h\x01\
\x01i\x02\x01@\x01\x09ecs-world\x03\0\x04\x04\0\x04init\x01\x05\x01h\x02\x01@\x02\
\x09ecs-world\x03\x04data\x06\x01\0\x04\0\x06update\x01\x07\x04\x01\x16wired:scr\
ipt/lifecycle\x05\x03\x04\x01\x13unavi:system/script\x04\0\x0b\x0c\x01\0\x06scri\
pt\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.201.0\x10\
wit-bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
