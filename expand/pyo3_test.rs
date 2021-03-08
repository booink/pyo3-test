#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use pyo3::prelude::*;

struct Hello {
    world: String,
}
unsafe impl pyo3::type_object::PyTypeInfo for Hello {
    type Type = Hello;
    type BaseType = pyo3::PyAny;
    type Layout = pyo3::PyCell<Self>;
    type BaseLayout = pyo3::pycell::PyCellBase<pyo3::PyAny>;
    type Initializer = pyo3::pyclass_init::PyClassInitializer<Self>;
    type AsRefTarget = pyo3::PyCell<Self>;
    const NAME: &'static str = "Hello";
    const MODULE: Option<&'static str> = None;
    const DESCRIPTION: &'static str = "\u{0}";
    const FLAGS: usize = 0 | 0;
    #[inline]
    fn type_object_raw(py: pyo3::Python) -> *mut pyo3::ffi::PyTypeObject {
        use pyo3::type_object::LazyStaticType;
        static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
        TYPE_OBJECT.get_or_init::<Self>(py)
    }
}
impl pyo3::PyClass for Hello {
    type Dict = pyo3::pyclass_slots::PyClassDummySlot;
    type WeakRef = pyo3::pyclass_slots::PyClassDummySlot;
    type BaseNativeType = pyo3::PyAny;
}
impl <'a> pyo3::derive_utils::ExtractExt<'a> for &'a Hello {
    type Target = pyo3::PyRef<'a, Hello>;
}
impl <'a> pyo3::derive_utils::ExtractExt<'a> for &'a mut Hello {
    type Target = pyo3::PyRefMut<'a, Hello>;
}
impl pyo3::pyclass::PyClassSend for Hello {
    type ThreadChecker = pyo3::pyclass::ThreadCheckerStub<Hello>;
}
impl pyo3::IntoPy<pyo3::PyObject> for Hello {
    fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self).unwrap(), py)
    }
}
#[doc(hidden)]
pub struct Pyo3MethodsInventoryForHello {
    methods: Vec<pyo3::class::PyMethodDefType>,
}
impl pyo3::class::methods::PyMethodsInventory for Pyo3MethodsInventoryForHello
 {
    fn new(methods: Vec<pyo3::class::PyMethodDefType>) -> Self {
        Self{methods,}
    }
    fn get(&'static self) -> &'static [pyo3::class::PyMethodDefType] {
        &self.methods
    }
}
impl pyo3::class::methods::HasMethodsInventory for Hello {
    type Methods = Pyo3MethodsInventoryForHello;
}


impl ::inventory::Collect for Pyo3MethodsInventoryForHello {
    #[inline]
    fn registry() -> &'static ::inventory::Registry<Self> {
        static REGISTRY: ::inventory::Registry<Pyo3MethodsInventoryForHello> =
            ::inventory::Registry::new();
        &REGISTRY
    }
}
impl pyo3::class::proto_methods::PyProtoMethods for Hello {
    fn for_each_proto_slot<Visitor: FnMut(pyo3::ffi::PyType_Slot)>(visitor:
        Visitor) {
        use pyo3::class::proto_methods::*;
        let protocols = PyClassProtocols::<Hello>::new();
        protocols.object_protocol_slots().iter().chain(protocols.number_protocol_slots()).chain(protocols.iter_protocol_slots()).chain(protocols.gc_protocol_slots()).chain(protocols.descr_protocol_slots()).chain(protocols.mapping_protocol_slots()).chain(protocols.sequence_protocol_slots()).chain(protocols.async_protocol_slots()).chain(protocols.buffer_protocol_slots()).cloned().for_each(visitor);
    }
    fn get_buffer()
        -> Option<&'static pyo3::class::proto_methods::PyBufferProcs> {
            use pyo3::class::proto_methods::*;
            let protocols = PyClassProtocols::<Hello>::new();
            protocols.buffer_procs()
    }
}
#[allow(non_upper_case_globals)]
extern fn __init4579329234899644335() {
    pyo3::inventory::submit({
        {
            type Inventory =
                <Hello as
                pyo3::class::methods::HasMethodsInventory>::Methods;
            <Inventory as
                pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(box
                        [pyo3::class::PyMethodDefType::Getter({
                            unsafe extern "C" fn __wrap(_slf:
                                *mut pyo3::ffi::PyObject,
                                _:
                                *mut std::os::raw::c_void)
                                ->
                                    *mut pyo3::ffi::PyObject {
                                        const
                                            _LOCATION:
                                            &'static str
                                            =
                                            "Hello.world()";
                                        {
                                            let pool =
                                                ::pyo3::GILPool::new();
                                            let unwind_safe_py =
                                                std::panic::AssertUnwindSafe(pool.python());
                                            let result =
                                                match std::panic::catch_unwind(move
                                                    ||
                                                    ->
                                                    ::pyo3::PyResult<_>
                                                    {
                                                        let _py =
                                                            *unwind_safe_py;
                                                        {
                                                            let _cell =
                                                                _py.from_borrowed_ptr::<pyo3::PyCell<Hello>>(_slf);
                                                            let _ref =
                                                                _cell.try_borrow()?;
                                                            let _slf =
                                                                &_ref;
                                                            pyo3::callback::convert(_py,
                                                                {
                                                                    _slf.world.clone()
                                                                })
                                                        }
                                                    })
                                            {
                                                Ok(result)
                                                    =>
                                                    result,
                                                    Err(e)
                                                        =>
                                                        {
                                                            if let Some(string)
                                                                =
                                                                    e.downcast_ref::<String>()
                                                            {
                                                                Err(::pyo3::panic::PanicException::new_err((string.clone(),)))
                                                            } else if let Some(s)
                                                                =
                                                                    e.downcast_ref::<&str>()
                                                            {
                                                                Err(::pyo3::panic::PanicException::new_err((s.to_string(),)))
                                                            } else {
                                                                Err(::pyo3::panic::PanicException::new_err(("panic from Rust code",)))
                                                            }
                                                        }
                                            };
                                            result.unwrap_or_else(|e|
                                                {
                                                    e.restore(pool.python());
                                                    ::pyo3::callback::callback_error()
                                                })
                                        }
                                    }
                            pyo3::class::PyGetterDef::new("world\u{0}",
                                __wrap,
                                "\u{0}")
                        })]))
        }
    });
}
#[used]
#[allow(non_upper_case_globals)]
#[link_section = ".init_array"]
static __init4579329234899644335___rust_ctor___ctor: unsafe extern "C" fn() =
{
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __init4579329234899644335___rust_ctor___ctor() {
        __init4579329234899644335()
    }
    ;
    __init4579329234899644335___rust_ctor___ctor
};
impl pyo3::pyclass::PyClassAlloc for Hello { }
fn pyo3_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Hello>()?;
    Ok(())
}
#[no_mangle]
#[allow(non_snake_case)]
#[doc =
  r" This autogenerated function is called by the python interpreter when importing"]
#[doc = r" the module."]
  pub unsafe extern "C" fn PyInit_pyo3_test() -> *mut pyo3::ffi::PyObject {
      use pyo3::derive_utils::ModuleDef;
      const NAME: &'static str = "pyo3_test\u{0}";
      static MODULE_DEF: ModuleDef = unsafe { ModuleDef::new(NAME) };
      {
          {
              let pool = ::pyo3::GILPool::new();
              let unwind_safe_py = std::panic::AssertUnwindSafe(pool.python());
              let result =
                  match std::panic::catch_unwind(move || -> ::pyo3::PyResult<_>
                      {
                          let _py =
                              *unwind_safe_py;
                          ::pyo3::callback::convert(_py,
                              {
                                  MODULE_DEF.make_module("",
                                      pyo3_test)
                              })
                      }) {
                      Ok(result) => result,
                      Err(e) => {
                          if let Some(string) = e.downcast_ref::<String>() {
                              Err(::pyo3::panic::PanicException::new_err((string.clone(),)))
                          } else if let Some(s) = e.downcast_ref::<&str>() {
                              Err(::pyo3::panic::PanicException::new_err((s.to_string(),)))
                          } else {
                              Err(::pyo3::panic::PanicException::new_err(("panic from Rust code",)))
                          }
                      }
                  };
              result.unwrap_or_else(|e|
                  {
                      e.restore(pool.python());
                      ::pyo3::callback::callback_error()
                  })
          }
      }
  }
