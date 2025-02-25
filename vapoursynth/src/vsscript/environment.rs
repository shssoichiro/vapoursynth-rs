use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::ptr;
use std::ptr::NonNull;
use vapoursynth_sys as ffi;

use crate::api::API;
use crate::core::CoreRef;
use crate::map::Map;
use crate::node::Node;
use crate::vsscript::errors::Result;
use crate::vsscript::*;

use crate::vsscript::VSScriptError;

/// VSScript file evaluation flags.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg(not(feature = "gte-vapoursynth-api-40"))]
pub enum EvalFlags {
    Nothing,
    /// The working directory will be changed to the script's directory for the evaluation.
    SetWorkingDir,
}

#[cfg(not(feature = "gte-vapoursynth-api-40"))]
impl EvalFlags {
    #[inline]
    fn ffi_type(self) -> ::std::os::raw::c_int {
        match self {
            EvalFlags::Nothing => 0,
            EvalFlags::SetWorkingDir => ffi::VSEvalFlags::efSetWorkingDir as _,
        }
    }
}

/// Contains two possible variants of arguments to `Environment::evaluate_script()`.
#[derive(Clone, Copy)]
enum EvaluateScriptArgs<'a> {
    /// Evaluate a script contained in the string.
    Script(&'a str),
    /// Evaluate a script contained in the file.
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    File(&'a Path, EvalFlags),
    /// Evaluate a script contained in the file.
    #[cfg(feature = "gte-vapoursynth-api-40")]
    File(&'a Path),
}

/// A wrapper for the VSScript environment.
#[derive(Debug)]
pub struct Environment {
    handle: NonNull<ffi::VSScript>,
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

impl Drop for Environment {
    #[inline]
    fn drop(&mut self) {
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        unsafe {
            ffi::vsscript_freeScript(self.handle.as_ptr());
        }

        #[cfg(feature = "gte-vapoursynth-api-40")]
        unsafe {
            (API::get_cached().vssapi().as_ref().unwrap().freeScript)(self.handle.as_ptr());
        }
    }
}

impl Environment {
    /// Retrieves the VSScript error message.
    ///
    /// # Safety
    /// This function must only be called if an error is present.
    #[inline]
    unsafe fn error(&self) -> CString {
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        unsafe {
            let message = ffi::vsscript_getError(self.handle.as_ptr());
            CStr::from_ptr(message).to_owned()
        }

        #[cfg(feature = "gte-vapoursynth-api-40")]
        unsafe {
            let message =
                (API::get_cached().vssapi().as_ref().unwrap().getError)(self.handle.as_ptr());
            CStr::from_ptr(message).to_owned()
        }
    }

    /// Creates an empty script environment.
    ///
    /// Useful if it is necessary to set some variable in the script environment before evaluating
    /// any scripts.
    pub fn new() -> Result<Self> {
        maybe_initialize();

        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let mut handle = ptr::null_mut();
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let rv = unsafe { call_vsscript!(ffi::vsscript_createScript(&mut handle)) };

        #[cfg(feature = "gte-vapoursynth-api-40")]
        let rv = 0;
        #[cfg(feature = "gte-vapoursynth-api-40")]
        let handle = unsafe {
            (API::get().unwrap().vssapi().as_ref().unwrap().createScript)(ptr::null_mut())
        };

        let environment = Self {
            handle: unsafe { NonNull::new_unchecked(handle) },
        };

        if rv != 0 {
            Err(VSScriptError::new(unsafe { environment.error() }).into())
        } else {
            Ok(environment)
        }
    }

    /// Calls `vsscript_evaluateScript()`.
    ///
    /// `self` is taken by a mutable reference mainly to ensure the atomicity of a call to
    /// `vsscript_evaluateScript()` (a function that could produce an error) and the following call
    /// to `vsscript_getError()`. If atomicity is not enforced, another thread could perform some
    /// operation between these two and clear or change the error message.
    fn evaluate_script(&mut self, args: EvaluateScriptArgs) -> Result<()> {
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let (script, path, flags) = match args {
            EvaluateScriptArgs::Script(script) => (script.to_owned(), None, EvalFlags::Nothing),
            EvaluateScriptArgs::File(path, flags) => {
                let mut file = File::open(path).map_err(Error::FileOpen)?;
                let mut script = String::new();
                file.read_to_string(&mut script).map_err(Error::FileRead)?;

                // vsscript throws an error if it's not valid UTF-8 anyway.
                let path = path.to_str().ok_or(Error::PathInvalidUnicode)?;
                let path = CString::new(path)?;

                (script, Some(path), flags)
            }
        };
        #[cfg(feature = "gte-vapoursynth-api-40")]
        let (script, path) = match args {
            EvaluateScriptArgs::Script(script) => (script.to_owned(), None),
            EvaluateScriptArgs::File(path) => {
                let mut file = File::open(path).map_err(Error::FileOpen)?;
                let mut script = String::new();
                file.read_to_string(&mut script).map_err(Error::FileRead)?;

                // vsscript throws an error if it's not valid UTF-8 anyway.
                let path = path.to_str().ok_or(Error::PathInvalidUnicode)?;
                let path = CString::new(path)?;

                (script, Some(path))
            }
        };

        let script = CString::new(script)?;

        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let rv = unsafe {
            call_vsscript!(ffi::vsscript_evaluateScript(
                &mut self.handle.as_ptr(),
                script.as_ptr(),
                path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
                flags.ffi_type(),
            ))
        };

        #[cfg(feature = "gte-vapoursynth-api-40")]
        let rv = unsafe {
            (API::get_cached().vssapi().as_ref().unwrap().evaluateBuffer)(
                self.handle.as_mut(),
                script.as_ptr(),
                path.as_ref().map(|p| p.as_ptr()).unwrap_or(ptr::null()),
            )
        };

        if rv != 0 {
            Err(VSScriptError::new(unsafe { self.error() }).into())
        } else {
            Ok(())
        }
    }

    /// Creates a script environment and evaluates a script contained in a string.
    #[inline]
    pub fn from_script(script: &str) -> Result<Self> {
        let mut environment = Self::new()?;
        environment.evaluate_script(EvaluateScriptArgs::Script(script))?;
        Ok(environment)
    }

    /// Creates a script environment and evaluates a script contained in a file.
    #[inline]
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    pub fn from_file<P: AsRef<Path>>(path: P, flags: EvalFlags) -> Result<Self> {
        let mut environment = Self::new()?;
        environment.evaluate_script(EvaluateScriptArgs::File(path.as_ref(), flags))?;
        Ok(environment)
    }

    /// Creates a script environment and evaluates a script contained in a file.
    #[inline]
    #[cfg(feature = "gte-vapoursynth-api-40")]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut environment = Self::new()?;
        environment.evaluate_script(EvaluateScriptArgs::File(path.as_ref()))?;
        Ok(environment)
    }

    /// Evaluates a script contained in a string.
    #[inline]
    pub fn eval_script(&mut self, script: &str) -> Result<()> {
        self.evaluate_script(EvaluateScriptArgs::Script(script))
    }

    /// Evaluates a script contained in a file.
    #[inline]
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    pub fn eval_file<P: AsRef<Path>>(&mut self, path: P, flags: EvalFlags) -> Result<()> {
        self.evaluate_script(EvaluateScriptArgs::File(path.as_ref(), flags))
    }

    /// Evaluates a script contained in a file.
    #[inline]
    #[cfg(feature = "gte-vapoursynth-api-40")]
    pub fn eval_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.evaluate_script(EvaluateScriptArgs::File(path.as_ref()))
    }

    /// Clears the script environment.
    #[inline]
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    pub fn clear(&self) {
        // In api v4, this function doesn't exist, we should just drop the environment and
        // create a new one instead.
        unsafe {
            ffi::vsscript_clearEnvironment(self.handle.as_ptr());
        }
    }

    /// Retrieves a node from the script environment. A node in the script must have been marked
    /// for output with the requested index.
    #[cfg(all(
        not(feature = "gte-vsscript-api-31"),
        feature = "vapoursynth-functions"
    ))]
    #[inline]
    pub fn get_output(&self, index: i32) -> Result<Node> {
        // Node needs the API.
        API::get().ok_or(Error::NoAPI)?;

        let node_handle = unsafe { ffi::vsscript_getOutput(self.handle.as_ptr(), index) };
        if node_handle.is_null() {
            Err(Error::NoOutput)
        } else {
            Ok(unsafe { Node::from_ptr(node_handle) })
        }
    }

    /// Retrieves a node from the script environment. A node in the script must have been marked
    /// for output with the requested index. The second node, if any, contains the alpha clip.
    #[cfg(all(
        feature = "gte-vsscript-api-31",
        any(feature = "vapoursynth-functions", feature = "gte-vsscript-api-32")
    ))]
    #[inline]
    pub fn get_output(&mut self, index: i32) -> Result<(Node, Option<Node>)> {
        // Node needs the API.
        API::get().ok_or(Error::NoAPI)?;

        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let mut alpha_handle = ptr::null_mut();
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let node_handle =
            unsafe { ffi::vsscript_getOutput2(self.handle.as_ptr(), index, &mut alpha_handle) };

        #[cfg(feature = "gte-vapoursynth-api-40")]
        let node_handle = unsafe {
            (API::get_cached().vssapi().as_ref().unwrap().getOutputNode)(
                self.handle.as_mut(),
                index,
            )
        };

        if node_handle.is_null() {
            return Err(Error::NoOutput);
        }

        #[cfg(feature = "gte-vapoursynth-api-40")]
        let alpha_handle = unsafe {
            (API::get_cached()
                .vssapi()
                .as_ref()
                .unwrap()
                .getOutputAlphaNode)(self.handle.as_mut(), index)
        };

        let node = unsafe { Node::from_ptr(node_handle) };
        let alpha_node = unsafe { alpha_handle.as_mut().map(|p| Node::from_ptr(p)) };

        Ok((node, alpha_node))
    }

    /// Cancels a node set for output. The node will no longer be available to `get_output()`.
    #[inline]
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    pub fn clear_output(&self, index: i32) -> Result<()> {
        // This functionality does not exist in api v4, it is handled by vapoursynth's cache handler instead
        let rv = unsafe { ffi::vsscript_clearOutput(self.handle.as_ptr(), index) };
        if rv != 0 {
            Err(Error::NoOutput)
        } else {
            Ok(())
        }
    }

    /// Retrieves the VapourSynth core that was created in the script environment. If a VapourSynth
    /// core has not been created yet, it will be created now, with the default options.
    #[cfg(any(feature = "vapoursynth-functions", feature = "gte-vsscript-api-32"))]
    pub fn get_core(&mut self) -> Result<CoreRef> {
        // CoreRef needs the API.
        API::get().ok_or(Error::NoAPI)?;

        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let ptr = unsafe { ffi::vsscript_getCore(self.handle.as_ptr()) };
        #[cfg(feature = "gte-vapoursynth-api-40")]
        let ptr =
            unsafe { (API::get_cached().vssapi().as_ref().unwrap().getCore)(self.handle.as_mut()) };
        if ptr.is_null() {
            Err(Error::NoCore)
        } else {
            Ok(unsafe { CoreRef::from_ptr(ptr) })
        }
    }

    /// Retrieves a variable from the script environment.
    pub fn get_variable(&self, name: &str, map: &mut Map) -> Result<()> {
        let name = CString::new(name)?;
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let rv = unsafe {
            ffi::vsscript_getVariable(self.handle.as_ptr(), name.as_ptr(), map.deref_mut())
        };
        #[cfg(feature = "gte-vapoursynth-api-40")]
        let rv = unsafe {
            (API::get_cached().vssapi().as_ref().unwrap().getVariable)(
                self.handle.as_ptr(),
                name.as_ptr(),
                map.deref_mut(),
            )
        };
        if rv != 0 {
            Err(Error::NoSuchVariable)
        } else {
            Ok(())
        }
    }

    /// Sets variables in the script environment.
    pub fn set_variables(&self, variables: &Map) -> Result<()> {
        #[cfg(not(feature = "gte-vapoursynth-api-40"))]
        let rv = unsafe { ffi::vsscript_setVariable(self.handle.as_ptr(), variables.deref()) };
        #[cfg(feature = "gte-vapoursynth-api-40")]
        let rv = unsafe {
            (API::get_cached().vssapi().as_ref().unwrap().setVariables)(
                self.handle.as_ptr(),
                variables.deref(),
            )
        };
        if rv != 0 {
            Err(Error::NoSuchVariable)
        } else {
            Ok(())
        }
    }

    /// Deletes a variable from the script environment.
    #[cfg(not(feature = "gte-vapoursynth-api-40"))]
    pub fn clear_variable(&self, name: &str) -> Result<()> {
        let name = CString::new(name)?;
        let rv = unsafe { ffi::vsscript_clearVariable(self.handle.as_ptr(), name.as_ptr()) };
        if rv != 0 {
            Err(Error::NoSuchVariable)
        } else {
            Ok(())
        }
    }
}
