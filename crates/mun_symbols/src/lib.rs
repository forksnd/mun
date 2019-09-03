#![feature(const_type_id)]

#[macro_use]
extern crate lazy_static;
extern crate libloading;
extern crate uuid;

mod field;
mod member;
mod method;
mod module;
mod reflection;

pub mod prelude {
    pub use crate::field::FieldInfo;
    pub use crate::member::MemberInfo;
    pub use crate::method::{
        EmptyMethodFactory, Invokable, MethodArg2RetFactory, MethodFactory, MethodInfo,
    };
    pub use crate::module::ModuleInfo;
    pub use crate::reflection::{Reflectable, Reflection, TypeInfo};
    pub use crate::Privacy;
    pub use uuid::Uuid;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Privacy {
    Public,
    Private,
}
