use llvm_sys::prelude::LLVMValueRef;

use std::ffi::CStr;

use types::PointerType;
use values::traits::AsValueRef;
use values::{InstructionValue, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct PointerValue {
    ptr_value: Value
}

impl PointerValue {
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        PointerValue {
            ptr_value: Value::new(value),
        }
    }

    pub fn get_name(&self) -> &CStr {
        self.ptr_value.get_name()
    }

    pub fn set_name(&self, name: &str) {
        self.ptr_value.set_name(name);
    }

    pub fn get_type(&self) -> PointerType {
        PointerType::new(self.ptr_value.get_type())
    }

    pub fn is_null(&self) -> bool {
        self.ptr_value.is_null()
    }

    pub fn is_undef(&self) -> bool {
        self.ptr_value.is_undef()
    }

    pub fn print_to_string(&self) -> &CStr {
        self.ptr_value.print_to_string()
    }

    pub fn print_to_stderr(&self) {
        self.ptr_value.print_to_stderr()
    }

    pub fn as_instruction(&self) -> Option<InstructionValue> {
        self.ptr_value.as_instruction()
    }
}

impl AsValueRef for PointerValue {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.ptr_value.value
    }
}
