#[cfg(not(feature = "gte-vapoursynth-api-40"))]
pub use apiv3::*;

#[cfg(feature = "gte-vapoursynth-api-40")]
pub use apiv4::*;

#[cfg(not(feature = "gte-vapoursynth-api-40"))]
mod apiv3 {
    use std::os::raw::*;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFrameRef {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSNodeRef {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSCore {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSPlugin {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSNode {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFuncRef {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSMap {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFrameContext {
        _unused: [u8; 0],
    }
    #[allow(clippy::unreadable_literal)]
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSColorFamily {
        cmGray = 1000000,
        cmRGB = 2000000,
        cmYUV = 3000000,
        cmYCoCg = 4000000,
        cmCompat = 9000000,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSSampleType {
        stInteger = 0,
        stFloat = 1,
    }
    #[allow(clippy::unreadable_literal)]
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSPresetFormat {
        pfNone = 0,
        pfGray8 = 1000010,
        pfGray16 = 1000011,
        pfGrayH = 1000012,
        pfGrayS = 1000013,
        pfYUV420P8 = 3000010,
        pfYUV422P8 = 3000011,
        pfYUV444P8 = 3000012,
        pfYUV410P8 = 3000013,
        pfYUV411P8 = 3000014,
        pfYUV440P8 = 3000015,
        pfYUV420P9 = 3000016,
        pfYUV422P9 = 3000017,
        pfYUV444P9 = 3000018,
        pfYUV420P10 = 3000019,
        pfYUV422P10 = 3000020,
        pfYUV444P10 = 3000021,
        pfYUV420P16 = 3000022,
        pfYUV422P16 = 3000023,
        pfYUV444P16 = 3000024,
        pfYUV444PH = 3000025,
        pfYUV444PS = 3000026,
        pfYUV420P12 = 3000027,
        pfYUV422P12 = 3000028,
        pfYUV444P12 = 3000029,
        pfYUV420P14 = 3000030,
        pfYUV422P14 = 3000031,
        pfYUV444P14 = 3000032,
        pfRGB24 = 2000010,
        pfRGB27 = 2000011,
        pfRGB30 = 2000012,
        pfRGB48 = 2000013,
        pfRGBH = 2000014,
        pfRGBS = 2000015,
        pfCompatBGR32 = 9000010,
        pfCompatYUY2 = 9000011,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSFilterMode {
        fmParallel = 100,
        fmParallelRequests = 200,
        fmUnordered = 300,
        fmSerial = 400,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFormat {
        pub name: [c_char; 32usize],
        pub id: c_int,
        pub colorFamily: c_int,
        pub sampleType: c_int,
        pub bitsPerSample: c_int,
        pub bytesPerSample: c_int,
        pub subSamplingW: c_int,
        pub subSamplingH: c_int,
        pub numPlanes: c_int,
    }
    pub const VSNodeFlags_nfNoCache: VSNodeFlags = VSNodeFlags(1);
    pub const VSNodeFlags_nfIsCache: VSNodeFlags = VSNodeFlags(2);
    #[cfg(feature = "gte-vapoursynth-api-33")]
    pub const VSNodeFlags_nfMakeLinear: VSNodeFlags = VSNodeFlags(4);
    impl ::std::ops::BitOr<VSNodeFlags> for VSNodeFlags {
        type Output = Self;
        #[inline]
        fn bitor(self, other: Self) -> Self {
            VSNodeFlags(self.0 | other.0)
        }
    }
    impl ::std::ops::BitOrAssign for VSNodeFlags {
        #[inline]
        fn bitor_assign(&mut self, rhs: VSNodeFlags) {
            self.0 |= rhs.0;
        }
    }
    impl ::std::ops::BitAnd<VSNodeFlags> for VSNodeFlags {
        type Output = Self;
        #[inline]
        fn bitand(self, other: Self) -> Self {
            VSNodeFlags(self.0 & other.0)
        }
    }
    impl ::std::ops::BitAndAssign for VSNodeFlags {
        #[inline]
        fn bitand_assign(&mut self, rhs: VSNodeFlags) {
            self.0 &= rhs.0;
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VSNodeFlags(pub c_int);
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSPropTypes {
        ptUnset = 117,
        ptInt = 105,
        ptFloat = 102,
        ptData = 115,
        ptNode = 99,
        ptFrame = 118,
        ptFunction = 109,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSGetPropErrors {
        peUnset = 1,
        peType = 2,
        peIndex = 4,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSPropAppendMode {
        paReplace = 0,
        paAppend = 1,
        paTouch = 2,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSCoreInfo {
        pub versionString: *const c_char,
        pub core: c_int,
        pub api: c_int,
        pub numThreads: c_int,
        pub maxFramebufferSize: i64,
        pub usedFramebufferSize: i64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSVideoInfo {
        pub format: *const VSFormat,
        pub fpsNum: i64,
        pub fpsDen: i64,
        pub width: c_int,
        pub height: c_int,
        pub numFrames: c_int,
        pub flags: c_int,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSActivationReason {
        arInitial = 0,
        arFrameReady = 1,
        arAllFramesReady = 2,
        arError = -1,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSMessageType {
        mtDebug = 0,
        mtWarning = 1,
        mtCritical = 2,
        mtFatal = 3,
    }
    pub type VSPublicFunction = unsafe extern "system" fn(
        in_: *const VSMap,
        out: *mut VSMap,
        userData: *mut c_void,
        core: *mut VSCore,
        vsapi: *const VSAPI,
    );
    pub type VSRegisterFunction = unsafe extern "system" fn(
        name: *const c_char,
        args: *const c_char,
        argsFunc: VSPublicFunction,
        functionData: *mut c_void,
        plugin: *mut VSPlugin,
    );
    pub type VSConfigPlugin = unsafe extern "system" fn(
        identifier: *const c_char,
        defaultNamespace: *const c_char,
        name: *const c_char,
        apiVersion: c_int,
        readonly: c_int,
        plugin: *mut VSPlugin,
    );
    pub type VSInitPlugin = Option<
        unsafe extern "system" fn(
            configFunc: VSConfigPlugin,
            registerFunc: VSRegisterFunction,
            plugin: *mut VSPlugin,
        ),
    >;
    pub type VSFreeFuncData = Option<unsafe extern "system" fn(userData: *mut c_void)>;
    pub type VSFilterInit = unsafe extern "system" fn(
        in_: *mut VSMap,
        out: *mut VSMap,
        instanceData: *mut *mut c_void,
        node: *mut VSNode,
        core: *mut VSCore,
        vsapi: *const VSAPI,
    );
    pub type VSFilterGetFrame = unsafe extern "system" fn(
        n: c_int,
        activationReason: c_int,
        instanceData: *mut *mut c_void,
        frameData: *mut *mut c_void,
        frameCtx: *mut VSFrameContext,
        core: *mut VSCore,
        vsapi: *const VSAPI,
    ) -> *const VSFrameRef;
    pub type VSFilterFree = Option<
        unsafe extern "system" fn(
            instanceData: *mut c_void,
            core: *mut VSCore,
            vsapi: *const VSAPI,
        ),
    >;
    pub type VSFrameDoneCallback = Option<
        unsafe extern "system" fn(
            userData: *mut c_void,
            f: *const VSFrameRef,
            n: c_int,
            arg1: *mut VSNodeRef,
            errorMsg: *const c_char,
        ),
    >;
    pub type VSMessageHandler = Option<
        unsafe extern "system" fn(msgType: c_int, msg: *const c_char, userData: *mut c_void),
    >;
    #[cfg(feature = "gte-vapoursynth-api-36")]
    pub type VSMessageHandlerFree = Option<unsafe extern "system" fn(userData: *mut c_void)>;
    #[cfg(feature = "gte-vapoursynth-api-36")]
    pub type VSMessageHandlerId = c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VSAPI {
        pub createCore: unsafe extern "system" fn(threads: c_int) -> *mut VSCore,
        pub freeCore: unsafe extern "system" fn(core: *mut VSCore),
        #[cfg_attr(
            feature = "gte-vapoursynth-api-36",
            deprecated(note = "use `getCoreInfo2` instead")
        )]
        pub getCoreInfo: unsafe extern "system" fn(core: *mut VSCore) -> *const VSCoreInfo,
        pub cloneFrameRef: unsafe extern "system" fn(f: *const VSFrameRef) -> *const VSFrameRef,
        pub cloneNodeRef: unsafe extern "system" fn(node: *mut VSNodeRef) -> *mut VSNodeRef,
        pub cloneFuncRef: unsafe extern "system" fn(f: *mut VSFuncRef) -> *mut VSFuncRef,
        pub freeFrame: unsafe extern "system" fn(f: *const VSFrameRef),
        pub freeNode: unsafe extern "system" fn(node: *mut VSNodeRef),
        pub freeFunc: unsafe extern "system" fn(f: *mut VSFuncRef),
        pub newVideoFrame: unsafe extern "system" fn(
            format: *const VSFormat,
            width: c_int,
            height: c_int,
            propSrc: *const VSFrameRef,
            core: *mut VSCore,
        ) -> *mut VSFrameRef,
        pub copyFrame:
            unsafe extern "system" fn(f: *const VSFrameRef, core: *mut VSCore) -> *mut VSFrameRef,
        pub copyFrameProps: unsafe extern "system" fn(
            src: *const VSFrameRef,
            dst: *mut VSFrameRef,
            core: *mut VSCore,
        ),
        pub registerFunction: unsafe extern "system" fn(
            name: *const c_char,
            args: *const c_char,
            argsFunc: VSPublicFunction,
            functionData: *mut c_void,
            plugin: *mut VSPlugin,
        ),
        pub getPluginById: unsafe extern "system" fn(
            identifier: *const c_char,
            core: *mut VSCore,
        ) -> *mut VSPlugin,
        pub getPluginByNs:
            unsafe extern "system" fn(ns: *const c_char, core: *mut VSCore) -> *mut VSPlugin,
        pub getPlugins: unsafe extern "system" fn(core: *mut VSCore) -> *mut VSMap,
        pub getFunctions: unsafe extern "system" fn(plugin: *mut VSPlugin) -> *mut VSMap,
        #[allow(clippy::type_complexity)]
        pub createFilter: unsafe extern "system" fn(
            in_: *const VSMap,
            out: *mut VSMap,
            name: *const c_char,
            init: VSFilterInit,
            getFrame: VSFilterGetFrame,
            free: VSFilterFree,
            filterMode: c_int,
            flags: c_int,
            instanceData: *mut c_void,
            core: *mut VSCore,
        ),
        pub setError: unsafe extern "system" fn(map: *mut VSMap, errorMessage: *const c_char),
        pub getError: unsafe extern "system" fn(map: *const VSMap) -> *const c_char,
        pub setFilterError:
            unsafe extern "system" fn(errorMessage: *const c_char, frameCtx: *mut VSFrameContext),
        pub invoke: unsafe extern "system" fn(
            plugin: *mut VSPlugin,
            name: *const c_char,
            args: *const VSMap,
        ) -> *mut VSMap,
        pub getFormatPreset:
            unsafe extern "system" fn(id: c_int, core: *mut VSCore) -> *const VSFormat,
        pub registerFormat: unsafe extern "system" fn(
            colorFamily: c_int,
            sampleType: c_int,
            bitsPerSample: c_int,
            subSamplingW: c_int,
            subSamplingH: c_int,
            core: *mut VSCore,
        ) -> *const VSFormat,
        pub getFrame: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNodeRef,
            errorMsg: *mut c_char,
            bufSize: c_int,
        ) -> *const VSFrameRef,
        pub getFrameAsync: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNodeRef,
            callback: VSFrameDoneCallback,
            userData: *mut c_void,
        ),
        pub getFrameFilter: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNodeRef,
            frameCtx: *mut VSFrameContext,
        ) -> *const VSFrameRef,
        pub requestFrameFilter: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNodeRef,
            frameCtx: *mut VSFrameContext,
        ),
        pub queryCompletedFrame: unsafe extern "system" fn(
            node: *mut *mut VSNodeRef,
            n: *mut c_int,
            frameCtx: *mut VSFrameContext,
        ),
        pub releaseFrameEarly: unsafe extern "system" fn(
            node: *mut VSNodeRef,
            n: c_int,
            frameCtx: *mut VSFrameContext,
        ),
        pub getStride: unsafe extern "system" fn(f: *const VSFrameRef, plane: c_int) -> c_int,
        pub getReadPtr: unsafe extern "system" fn(f: *const VSFrameRef, plane: c_int) -> *const u8,
        pub getWritePtr: unsafe extern "system" fn(f: *mut VSFrameRef, plane: c_int) -> *mut u8,
        pub createFunc: unsafe extern "system" fn(
            func: VSPublicFunction,
            userData: *mut c_void,
            free: VSFreeFuncData,
            core: *mut VSCore,
            vsapi: *const VSAPI,
        ) -> *mut VSFuncRef,
        pub callFunc: unsafe extern "system" fn(
            func: *mut VSFuncRef,
            in_: *const VSMap,
            out: *mut VSMap,
            core: *mut VSCore,
            vsapi: *const VSAPI,
        ),
        pub createMap: unsafe extern "system" fn() -> *mut VSMap,
        pub freeMap: unsafe extern "system" fn(map: *mut VSMap),
        pub clearMap: unsafe extern "system" fn(map: *mut VSMap),
        pub getVideoInfo: unsafe extern "system" fn(node: *mut VSNodeRef) -> *const VSVideoInfo,
        pub setVideoInfo:
            unsafe extern "system" fn(vi: *const VSVideoInfo, numOutputs: c_int, node: *mut VSNode),
        pub getFrameFormat: unsafe extern "system" fn(f: *const VSFrameRef) -> *const VSFormat,
        pub getFrameWidth: unsafe extern "system" fn(f: *const VSFrameRef, plane: c_int) -> c_int,
        pub getFrameHeight: unsafe extern "system" fn(f: *const VSFrameRef, plane: c_int) -> c_int,
        pub getFramePropsRO: unsafe extern "system" fn(f: *const VSFrameRef) -> *const VSMap,
        pub getFramePropsRW: unsafe extern "system" fn(f: *mut VSFrameRef) -> *mut VSMap,
        pub propNumKeys: unsafe extern "system" fn(map: *const VSMap) -> c_int,
        pub propGetKey: unsafe extern "system" fn(map: *const VSMap, index: c_int) -> *const c_char,
        pub propNumElements:
            unsafe extern "system" fn(map: *const VSMap, key: *const c_char) -> c_int,
        pub propGetType: unsafe extern "system" fn(map: *const VSMap, key: *const c_char) -> c_char,
        pub propGetInt: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> i64,
        pub propGetFloat: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> f64,
        pub propGetData: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *const c_char,
        pub propGetDataSize: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> c_int,
        pub propGetNode: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *mut VSNodeRef,
        pub propGetFrame: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *const VSFrameRef,
        pub propGetFunc: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *mut VSFuncRef,
        pub propDeleteKey: unsafe extern "system" fn(map: *mut VSMap, key: *const c_char) -> c_int,
        pub propSetInt: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            i: i64,
            append: c_int,
        ) -> c_int,
        pub propSetFloat: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            d: f64,
            append: c_int,
        ) -> c_int,
        pub propSetData: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            data: *const c_char,
            size: c_int,
            append: c_int,
        ) -> c_int,
        pub propSetNode: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            node: *mut VSNodeRef,
            append: c_int,
        ) -> c_int,
        pub propSetFrame: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            f: *const VSFrameRef,
            append: c_int,
        ) -> c_int,
        pub propSetFunc: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            func: *mut VSFuncRef,
            append: c_int,
        ) -> c_int,
        pub setMaxCacheSize: unsafe extern "system" fn(bytes: i64, core: *mut VSCore) -> i64,
        pub getOutputIndex: unsafe extern "system" fn(frameCtx: *mut VSFrameContext) -> c_int,
        pub newVideoFrame2: unsafe extern "system" fn(
            format: *const VSFormat,
            width: c_int,
            height: c_int,
            planeSrc: *mut *const VSFrameRef,
            planes: *const c_int,
            propSrc: *const VSFrameRef,
            core: *mut VSCore,
        ) -> *mut VSFrameRef,
        #[cfg_attr(
            feature = "gte-vapoursynth-api-36",
            deprecated(note = "use `addMessageHandler` and `removeMessageHandler` instead")
        )]
        pub setMessageHandler:
            unsafe extern "system" fn(handler: VSMessageHandler, userData: *mut c_void),
        pub setThreadCount: unsafe extern "system" fn(threads: c_int, core: *mut VSCore) -> c_int,
        pub getPluginPath: unsafe extern "system" fn(plugin: *const VSPlugin) -> *const c_char,

        #[cfg(feature = "gte-vapoursynth-api-31")]
        pub propGetIntArray: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            error: *mut c_int,
        ) -> *const i64,
        #[cfg(feature = "gte-vapoursynth-api-31")]
        pub propGetFloatArray: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            error: *mut c_int,
        ) -> *const f64,
        #[cfg(feature = "gte-vapoursynth-api-31")]
        pub propSetIntArray: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            i: *const i64,
            size: c_int,
        ) -> c_int,
        #[cfg(feature = "gte-vapoursynth-api-31")]
        pub propSetFloatArray: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            d: *const f64,
            size: c_int,
        ) -> c_int,
        #[cfg(feature = "gte-vapoursynth-api-34")]
        pub logMessage: unsafe extern "system" fn(msgType: c_int, msg: *const c_char),
        #[cfg(feature = "gte-vapoursynth-api-36")]
        pub addMessageHandler: unsafe extern "system" fn(
            handler: VSMessageHandler,
            free: VSMessageHandlerFree,
            userData: *mut c_void,
        ) -> VSMessageHandlerId,
        #[cfg(feature = "gte-vapoursynth-api-36")]
        pub removeMessageHandler: unsafe extern "system" fn(id: VSMessageHandlerId) -> c_int,
        #[cfg(feature = "gte-vapoursynth-api-36")]
        pub getCoreInfo2: unsafe extern "system" fn(core: *mut VSCore, info: *mut VSCoreInfo),
    }

    #[cfg(feature = "vapoursynth-functions")]
    unsafe extern "system" {
        pub fn getVapourSynthAPI(version: c_int) -> *const VSAPI;
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSScript {
        _unused: [u8; 0],
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSEvalFlags {
        efSetWorkingDir = 1,
    }

    #[cfg(feature = "vsscript-functions")]
    unsafe extern "system" {
        #[cfg(feature = "gte-vsscript-api-31")]
        pub fn vsscript_getApiVersion() -> c_int;
        pub fn vsscript_init() -> c_int;
        pub fn vsscript_finalize() -> c_int;
        pub fn vsscript_evaluateScript(
            handle: *mut *mut VSScript,
            script: *const c_char,
            scriptFilename: *const c_char,
            flags: c_int,
        ) -> c_int;
        pub fn vsscript_evaluateFile(
            handle: *mut *mut VSScript,
            scriptFilename: *const c_char,
            flags: c_int,
        ) -> c_int;
        pub fn vsscript_createScript(handle: *mut *mut VSScript) -> c_int;
        pub fn vsscript_freeScript(handle: *mut VSScript);
        pub fn vsscript_getError(handle: *mut VSScript) -> *const c_char;
        pub fn vsscript_getOutput(handle: *mut VSScript, index: c_int) -> *mut VSNodeRef;
        #[cfg(feature = "gte-vsscript-api-31")]
        pub fn vsscript_getOutput2(
            handle: *mut VSScript,
            index: c_int,
            alpha: *mut *mut VSNodeRef,
        ) -> *mut VSNodeRef;
        pub fn vsscript_clearOutput(handle: *mut VSScript, index: c_int) -> c_int;
        pub fn vsscript_getCore(handle: *mut VSScript) -> *mut VSCore;
        pub fn vsscript_getVSApi() -> *const VSAPI;
        #[cfg(feature = "gte-vsscript-api-32")]
        pub fn vsscript_getVSApi2(version: c_int) -> *const VSAPI;
        pub fn vsscript_getVariable(
            handle: *mut VSScript,
            name: *const c_char,
            dst: *mut VSMap,
        ) -> c_int;
        pub fn vsscript_setVariable(handle: *mut VSScript, vars: *const VSMap) -> c_int;
        pub fn vsscript_clearVariable(handle: *mut VSScript, name: *const c_char) -> c_int;
        pub fn vsscript_clearEnvironment(handle: *mut VSScript);
    }
}

#[cfg(feature = "gte-vapoursynth-api-40")]
mod apiv4 {
    use std::os::raw::*;

    pub const VS_AUDIO_FRAME_SAMPLES: usize = 3072;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFrame {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSNode {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSCore {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSPlugin {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSPluginFunction {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFunction {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSMap {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSLogHandle {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSFrameContext {
        _unused: [u8; 0],
    }
    #[allow(clippy::unreadable_literal)]
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSColorFamily {
        cfUndefined = 0,
        cfGray = 1,
        cfRGB = 2,
        cfYUV = 3,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSSampleType {
        stInteger = 0,
        stFloat = 1,
    }
    macro_rules! VS_MAKE_VIDEO_ID {
        ($colorFamily:expr, $sampleType:expr, $bitsPerSample:expr, $subSamplingW:expr, $subSamplingH:expr) => {
            ((($colorFamily as i32) << 28)
                | (($sampleType as i32) << 24)
                | ($bitsPerSample << 16)
                | ($subSamplingW << 8)
                | ($subSamplingH << 0))
        };
    }
    #[allow(clippy::unreadable_literal)]
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSPresetFormat {
        pfNone = 0,
        pfGray8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 8, 0, 0),
        pfGray9 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 9, 0, 0),
        pfGray10 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 10, 0, 0),
        pfGray12 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 12, 0, 0),
        pfGray14 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 14, 0, 0),
        pfGray16 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 16, 0, 0),
        pfGray32 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stInteger, 32, 0, 0),

        pfGrayH = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stFloat, 16, 0, 0),
        pfGrayS = VS_MAKE_VIDEO_ID!(VSColorFamily::cfGray, VSSampleType::stFloat, 32, 0, 0),

        pfYUV410P8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 8, 2, 2),
        pfYUV411P8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 8, 2, 0),
        pfYUV440P8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 8, 0, 1),

        pfYUV420P8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 8, 1, 1),
        pfYUV422P8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 8, 1, 0),
        pfYUV444P8 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 8, 0, 0),

        pfYUV420P9 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 9, 1, 1),
        pfYUV422P9 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 9, 1, 0),
        pfYUV444P9 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 9, 0, 0),

        pfYUV420P10 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 10, 1, 1),
        pfYUV422P10 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 10, 1, 0),
        pfYUV444P10 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 10, 0, 0),

        pfYUV420P12 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 12, 1, 1),
        pfYUV422P12 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 12, 1, 0),
        pfYUV444P12 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 12, 0, 0),

        pfYUV420P14 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 14, 1, 1),
        pfYUV422P14 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 14, 1, 0),
        pfYUV444P14 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 14, 0, 0),

        pfYUV420P16 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 16, 1, 1),
        pfYUV422P16 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 16, 1, 0),
        pfYUV444P16 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stInteger, 16, 0, 0),

        pfYUV420PH = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stFloat, 16, 1, 1),
        pfYUV420PS = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stFloat, 32, 1, 1),
        pfYUV422PH = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stFloat, 16, 1, 0),
        pfYUV422PS = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stFloat, 32, 1, 0),
        pfYUV444PH = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stFloat, 16, 0, 0),
        pfYUV444PS = VS_MAKE_VIDEO_ID!(VSColorFamily::cfYUV, VSSampleType::stFloat, 32, 0, 0),

        pfRGB24 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stInteger, 8, 0, 0),
        pfRGB27 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stInteger, 9, 0, 0),
        pfRGB30 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stInteger, 10, 0, 0),
        pfRGB36 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stInteger, 12, 0, 0),
        pfRGB42 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stInteger, 14, 0, 0),
        pfRGB48 = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stInteger, 16, 0, 0),

        pfRGBH = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stFloat, 16, 0, 0),
        pfRGBS = VS_MAKE_VIDEO_ID!(VSColorFamily::cfRGB, VSSampleType::stFloat, 32, 0, 0),
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSFilterMode {
        fmParallel = 0,
        fmParallelRequests = 1,
        fmUnordered = 2,
        fmFrameState = 3,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSMediaType {
        mtVideo = 1,
        mtAudio = 2,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VSVideoFormat {
        pub colorFamily: c_int,
        pub sampleType: c_int,
        pub bitsPerSample: c_int,
        pub bytesPerSample: c_int,
        pub subSamplingW: c_int,
        pub subSamplingH: c_int,
        pub numPlanes: c_int,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSAudioChannels {
        acFrontLeft = 0,
        acFrontRight = 1,
        acFrontCenter = 2,
        acLowFrequency = 3,
        acBackLeft = 4,
        acBackRight = 5,
        acFrontLeftOFCenter = 6,
        acFrontRightOFCenter = 7,
        acBackCenter = 8,
        acSideLeft = 9,
        acSideRight = 10,
        acTopCenter = 11,
        acTopFrontLeft = 12,
        acTopFrontCenter = 13,
        acTopFrontRight = 14,
        acTopBackLeft = 15,
        acTopBackCenter = 16,
        acTopBackRight = 17,
        acStereoLeft = 29,
        acStereoRight = 30,
        acWideLeft = 31,
        acWideRight = 32,
        acSurroundDirectLeft = 33,
        acSurroundDirectRight = 34,
        acLowFrequency2 = 35,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSAudioFormat {
        pub sampleType: c_int,
        pub bitsPerSample: c_int,
        pub bytesPerSample: c_int,
        pub numChannels: c_int,
        pub channelLayout: u64,
    }

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSPropertyType {
        ptUnset = 0,
        ptInt = 1,
        ptFloat = 2,
        ptData = 3,
        ptFunction = 4,
        ptVideoNode = 5,
        ptAudioNode = 6,
        ptVideoFrame = 7,
        ptAudioFrame = 8,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSMapPropertyError {
        peSuccess = 0,
        peUnset = 1, /* no key exists */
        peType = 2,  /* key exists but not of a compatible type */
        peIndex = 4, /* index out of bounds */
        peError = 3, /* map has error state set */
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSMapAppendMode {
        maReplace = 0,
        maAppend = 1,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSCoreInfo {
        pub versionString: *const c_char,
        pub core: c_int,
        pub api: c_int,
        pub numThreads: c_int,
        pub maxFramebufferSize: i64,
        pub usedFramebufferSize: i64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSVideoInfo {
        pub format: VSVideoFormat,
        pub fpsNum: i64,
        pub fpsDen: i64,
        pub width: c_int,
        pub height: c_int,
        pub numFrames: c_int,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSAudioInfo {
        pub format: VSAudioFormat,
        pub sampleRate: c_int,
        pub numSamples: i64,
        pub numFrames: c_int,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSActivationReason {
        arInitial = 0,
        arAllFramesReady = 1,
        arError = -1,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSMessageType {
        mtDebug = 0,
        mtInformation = 1,
        mtWarning = 2,
        mtCritical = 3,
        mtFatal = 4, /* also terminates the process, should generally not be used by normal filters */
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSCoreCreationFlags {
        ccfEnableGraphInspection = 1,
        ccfDisableAutoLoading = 2,
        ccfDisableLibraryUnloading = 4,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSPluginConfigFlags {
        pcModifiable = 1,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSDataTypeHint {
        dtUnknown = -1,
        dtBinary = 0,
        dtUtf8 = 1,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSRequestPattern {
        rpGeneral = 0,       /* General pattern */
        rpNoFrameReuse = 1, /* When requesting all output frames from the filter no frame will be requested more than once from this input clip, never requests frames beyond the end of the clip */
        rpStrictSpatial = 2, /* Always (and only) requests frame n from the input clip when generating output frame n, never requests frames beyond the end of the clip */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        rpFrameReuseLastOnly = 3, /* Added in API 4.1, This modes is basically identical rpNoFrameReuse except that it hints the last frame may be requested multiple times */
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum VSCacheMode {
        cmAuto = -1,
        cmForceDisable = 0,
        cmForceEnable = 1,
    }

    pub type VSPublicFunction = unsafe extern "system" fn(
        in_: *const VSMap,
        out: *mut VSMap,
        userData: *mut c_void,
        core: *mut VSCore,
        vsapi: *const VSAPI,
    );
    pub type VSInitPlugin =
        Option<unsafe extern "system" fn(plugin: *mut VSPlugin, vspapi: *const VSPLUGINAPI)>;
    pub type VSFreeFunctionData = Option<unsafe extern "system" fn(userData: *mut c_void)>;
    pub type VSFilterGetFrame = unsafe extern "system" fn(
        n: c_int,
        activationReason: c_int,
        instanceData: *mut c_void,
        frameData: *mut *mut c_void,
        frameCtx: *mut VSFrameContext,
        core: *mut VSCore,
        vsapi: *const VSAPI,
    ) -> *const VSFrame;
    pub type VSFilterFree = Option<
        unsafe extern "system" fn(
            instanceData: *mut c_void,
            core: *mut VSCore,
            vsapi: *const VSAPI,
        ),
    >;
    pub type VSFrameDoneCallback = Option<
        unsafe extern "system" fn(
            userData: *mut c_void,
            f: *const VSFrame,
            n: c_int,
            node: *mut VSNode,
            errorMsg: *const c_char,
        ),
    >;
    pub type VSLogHandler = Option<
        unsafe extern "system" fn(msgType: c_int, msg: *const c_char, userData: *mut c_void),
    >;
    pub type VSLogHandlerFree = Option<unsafe extern "system" fn(userData: *mut c_void)>;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VSPLUGINAPI {
        pub getAPIVersion: unsafe extern "system" fn() -> c_int,
        pub configPlugin: unsafe extern "system" fn(
            identifier: *const c_char,
            pluginNamespace: *const c_char,
            name: *const c_char,
            pluginVersion: c_int,
            apiVersion: c_int,
            flags: c_int,
            plugin: *mut VSPlugin,
        ),
        pub registerFunction: unsafe extern "system" fn(
            name: *const c_char,
            args: *const c_char,
            returnType: *const c_char,
            argsFunc: VSPublicFunction,
            functionData: *mut c_void,
            plugin: *mut VSPlugin,
        ),
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VSFilterDependency {
        pub source: *mut VSNode,
        pub requestPattern: c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VSAPI {
        /* Audio and video filter related including nodes */

        /* output nodes are appended to the clip key in the out map */
        pub createVideoFilter: unsafe extern "system" fn(
            out: *mut VSMap,
            name: *const c_char,
            vi: *const VSVideoInfo,
            getFrame: VSFilterGetFrame,
            free: VSFilterFree,
            filterMode: c_int,
            dependencies: *const VSFilterDependency,
            numDeps: c_int,
            instanceData: *mut c_void,
            core: *mut VSCore,
        ),
        /* same as createVideoFilter but returns a pointer to the VSNode directly or NULL on failure */
        pub createVideoFilter2: unsafe extern "system" fn(
            name: *const c_char,
            vi: *const VSVideoInfo,
            getFrame: VSFilterGetFrame,
            free: VSFilterFree,
            filterMode: c_int,
            dependencies: *const VSFilterDependency,
            numDeps: c_int,
            instanceData: *mut c_void,
            core: *mut VSCore,
        ) -> *mut VSNode,
        /* output nodes are appended to the clip key in the out map */
        pub createAudioFilter: unsafe extern "system" fn(
            out: *mut VSMap,
            name: *const c_char,
            vi: *const VSAudioInfo,
            getFrame: VSFilterGetFrame,
            free: VSFilterFree,
            filterMode: c_int,
            dependencies: *const VSFilterDependency,
            numDeps: c_int,
            instanceData: *mut c_void,
            core: *mut VSCore,
        ),
        /* same as createAudioFilter but returns a pointer to the VSNode directly or NULL on failure */
        pub createAudioFilter2: unsafe extern "system" fn(
            name: *const c_char,
            vi: *const VSAudioInfo,
            getFrame: VSFilterGetFrame,
            free: VSFilterFree,
            filterMode: c_int,
            dependencies: *const VSFilterDependency,
            numDeps: c_int,
            instanceData: *mut c_void,
            core: *mut VSCore,
        ) -> *mut VSNode,
        /* Use right after create*Filter*, sets the correct cache mode for using the cacheFrame API and returns the recommended upper number of additional frames to cache per request */
        pub setLinearFilter: unsafe extern "system" fn(node: *mut VSNode) -> c_int,
        /* VSCacheMode, changing the cache mode also resets all options to their default */
        pub setCacheMode: unsafe extern "system" fn(node: *mut VSNode, mode: c_int),
        /* passing -1 means no change */
        pub setCacheOptions: unsafe extern "system" fn(
            node: *mut VSNode,
            fixedSize: c_int,
            maxSize: c_int,
            maxHistorySize: c_int,
        ),

        pub freeNode: unsafe extern "system" fn(node: *mut VSNode),
        pub addNodeRef: unsafe extern "system" fn(node: *mut VSNode) -> *mut VSNode,
        /* returns VSMediaType */
        pub getNodeType: unsafe extern "system" fn(node: *mut VSNode) -> c_int,
        pub getVideoInfo: unsafe extern "system" fn(node: *mut VSNode) -> *const VSVideoInfo,
        pub getAudioInfo: unsafe extern "system" fn(node: *mut VSNode) -> *const VSAudioInfo,

        /* Frame related functions */
        pub newVideoFrame: unsafe extern "system" fn(
            format: *const VSVideoFormat,
            width: c_int,
            height: c_int,
            propSrc: *const VSFrame,
            core: *mut VSCore,
        ) -> *mut VSFrame,
        /* same as newVideoFrame but allows the specified planes to be effectively copied from the source frames */
        pub newVideoFrame2: unsafe extern "system" fn(
            format: *const VSVideoFormat,
            width: c_int,
            height: c_int,
            planeSrc: *const *const VSFrame,
            planes: *const c_int,
            propSrc: *const VSFrame,
            core: *mut VSCore,
        ) -> *mut VSFrame,
        pub newAudioFrame: unsafe extern "system" fn(
            format: *const VSAudioFormat,
            numSamples: c_int,
            propSrc: *const VSFrame,
            core: *mut VSCore,
        ) -> *mut VSFrame,
        /* same as newAudioFrame but allows the specified channels to be effectively copied from the source frames */
        pub newAudioFrame2: unsafe extern "system" fn(
            format: *const VSAudioFormat,
            numSamples: c_int,
            channelSrc: *const *const VSFrame,
            channels: *const c_int,
            propSrc: *const VSFrame,
            core: *mut VSCore,
        ) -> *mut VSFrame,
        pub freeFrame: unsafe extern "system" fn(f: *const VSFrame),
        pub addFrameRef: unsafe extern "system" fn(f: *const VSFrame) -> *const VSFrame,
        pub copyFrame:
            unsafe extern "system" fn(f: *const VSFrame, core: *mut VSCore) -> *mut VSFrame,
        pub getFramePropertiesRO: unsafe extern "system" fn(f: *const VSFrame) -> *const VSMap,
        pub getFramePropertiesRW: unsafe extern "system" fn(f: *mut VSFrame) -> *mut VSMap,

        pub getStride: unsafe extern "system" fn(f: *const VSFrame, plane: c_int) -> c_int,
        pub getReadPtr: unsafe extern "system" fn(f: *const VSFrame, plane: c_int) -> *const u8,
        /* calling this function invalidates previously gotten read pointers to the same frame */
        pub getWritePtr: unsafe extern "system" fn(f: *const VSFrame, plane: c_int) -> *mut u8,

        pub getVideoFrameFormat:
            unsafe extern "system" fn(f: *const VSFrame) -> *const VSVideoFormat,
        pub getAudioFrameFormat:
            unsafe extern "system" fn(f: *const VSFrame) -> *const VSAudioFormat,
        pub getFrameType: unsafe extern "system" fn(f: *const VSFrame) -> c_int,
        pub getFrameWidth: unsafe extern "system" fn(f: *const VSFrame, plane: c_int) -> c_int,
        pub getFrameHeight: unsafe extern "system" fn(f: *const VSFrame, plane: c_int) -> c_int,
        /* returns the number of samples for audio frames */
        pub getFrameLength: unsafe extern "system" fn(f: *const VSFrame) -> c_int,

        /* General format functions  */
        /* up to 32 characters including terminating null may be written to the buffer, non-zero return value on success */
        pub getVideoFormatName:
            unsafe extern "system" fn(format: *const VSVideoFormat, buffer: *mut c_char) -> c_int,
        /* up to 32 characters including terminating null may be written to the buffer, non-zero return value on success */
        pub getAudioFormatName:
            unsafe extern "system" fn(format: *const VSVideoFormat, buffer: *mut c_char) -> c_int,
        /* non-zero return value on success */
        pub queryVideoFormat: unsafe extern "system" fn(
            format: *mut VSVideoFormat,
            colorFamily: c_int,
            sampleType: c_int,
            bitsPerSample: c_int,
            subSamplingW: c_int,
            subSamplingH: c_int,
            core: *mut VSCore,
        ) -> c_int,
        /* non-zero return value on success */
        pub queryAudioFormat: unsafe extern "system" fn(
            format: *mut VSAudioFormat,
            sampleType: c_int,
            bitsPerSample: c_int,
            channelLayout: u64,
            core: *mut VSCore,
        ) -> c_int,
        /* returns 0 on failure */
        pub queryVideoFormatID: unsafe extern "system" fn(
            colorFamily: c_int,
            sampleType: c_int,
            bitsPerSample: c_int,
            subSamplingW: c_int,
            subSamplingH: c_int,
            core: *mut VSCore,
        ) -> u32,
        /* non-zero return value on success */
        pub getVideoFormatByID: unsafe extern "system" fn(
            format: *mut VSAudioFormat,
            id: u32,
            core: *mut VSCore,
        ) -> c_int,

        /* Frame request and filter getframe functions */
        pub getFrame: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNode,
            errorMsg: *mut c_char,
            bufSize: c_int,
        ) -> *const VSFrame,
        /* only for external applications using the core as a library or for requesting frames in a filter constructor, do not use inside a filter's getframe function */
        pub getFrameAsync: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNode,
            callback: VSFrameDoneCallback,
            userData: *mut c_void,
        ),
        /* only use inside a filter's getframe function */
        pub getFrameFilter: unsafe extern "system" fn(
            n: c_int,
            node: *mut VSNode,
            frameCtx: *mut VSFrameContext,
        ) -> *const VSFrame,
        /* only use inside a filter's getframe function */
        pub requestFrameFilter:
            unsafe extern "system" fn(n: c_int, node: *mut VSNode, frameCtx: *mut VSFrameContext),
        /* only use inside a filter's getframe function, unless this function is called a requested frame is kept in memory until the end of processing the current frame */
        pub releaseFrameEarly:
            unsafe extern "system" fn(node: *mut VSNode, n: c_int, frameCtx: *mut VSFrameContext),
        /* used to store intermediate frames in cache, useful for filters where random access is slow, must call setLinearFilter on the node before using or the result is undefined  */
        pub cacheFrame: unsafe extern "system" fn(
            frame: *const VSFrame,
            n: c_int,
            frameCtx: *mut VSFrameContext,
        ),
        /* used to signal errors in the filter getframe function */
        pub setFilterError:
            unsafe extern "system" fn(errorMessage: *const c_char, frameCtx: *mut VSFrameContext),
        /* External functions */
        pub createFunction: unsafe extern "system" fn(
            func: VSPublicFunction,
            userData: *mut c_void,
            free: VSFreeFunctionData,
            core: *mut VSCore,
        ) -> *mut VSFunction,
        pub freeFunction: unsafe extern "system" fn(f: *mut VSFunction),
        pub addFunctionRef: unsafe extern "system" fn(f: *mut VSFunction) -> *mut VSFunction,
        pub callFunction:
            unsafe extern "system" fn(func: *mut VSFunction, r#in: *const VSMap, out: *mut VSMap),

        /* Map and property access functions */
        pub createMap: unsafe extern "system" fn() -> *mut VSMap,
        pub freeMap: unsafe extern "system" fn(map: *mut VSMap),
        pub clearMap: unsafe extern "system" fn(map: *mut VSMap),
        pub copyMap: unsafe extern "system" fn(src: *const VSMap, dst: *mut VSMap),

        /* used to signal errors outside filter getframe function */
        pub mapSetError: unsafe extern "system" fn(map: *mut VSMap, errorMessage: *const c_char),
        /* used to query errors, returns 0 if no error */
        pub mapGetError: unsafe extern "system" fn(map: *const VSMap) -> *const c_char,

        pub mapNumKeys: unsafe extern "system" fn(map: *const VSMap) -> c_int,
        pub mapGetKey: unsafe extern "system" fn(map: *const VSMap, index: c_int) -> *const c_char,
        pub mapDeleteKey: unsafe extern "system" fn(map: *mut VSMap, key: *const c_char) -> c_int,
        /* returns -1 if a key doesn't exist */
        pub mapNumElements:
            unsafe extern "system" fn(map: *const VSMap, key: *const c_char) -> c_int,
        /* returns VSPropertyType */
        pub mapGetType: unsafe extern "system" fn(map: *const VSMap, key: *const c_char) -> c_int,
        pub mapSetEmpty:
            unsafe extern "system" fn(map: *mut VSMap, key: *const c_char, r#type: c_int) -> c_int,

        pub mapGetInt: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> i64,
        pub mapGetIntSaturated: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> c_int,
        pub mapGetIntArray: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            error: *mut c_int,
        ) -> *const i64,
        pub mapSetInt: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            i: i64,
            append: c_int,
        ) -> c_int,
        pub mapSetIntArray: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            i: *const i64,
            size: c_int,
        ) -> c_int,

        pub mapGetFloat: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> c_double,
        pub mapGetFloatSaturated: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> c_float,
        pub mapGetFloatArray: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            error: *mut c_int,
        ) -> *const c_double,
        pub mapSetFloat: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            i: c_double,
            append: c_int,
        ) -> c_int,
        pub mapSetFloatArray: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            i: *const c_double,
            size: c_int,
        ) -> c_int,

        pub mapGetData: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *const c_char,
        pub mapGetDataSize: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> c_int,
        pub mapGetDataTypeHint: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> c_int,
        pub mapSetData: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            data: *const c_char,
            size: c_int,
            r#type: c_int,
            append: c_int,
        ) -> c_int,

        pub mapGetNode: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *mut VSNode,
        /* returns 0 on success */
        pub mapSetNode: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            node: *mut VSNode,
            append: c_int,
        ) -> c_int,
        /* always consumes the reference, even on error */
        pub mapConsumeNode: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            node: *mut VSNode,
            append: c_int,
        ) -> c_int,

        pub mapGetFrame: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *const VSFrame,
        /* returns 0 on success */
        pub mapSetFrame: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            f: *const VSFrame,
            append: c_int,
        ) -> c_int,
        /* always consumes the reference, even on error */
        pub mapConsumeFrame: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            f: *const VSFrame,
            append: c_int,
        ) -> c_int,

        pub mapGetFunction: unsafe extern "system" fn(
            map: *const VSMap,
            key: *const c_char,
            index: c_int,
            error: *mut c_int,
        ) -> *mut VSFunction,
        /* returns 0 on success */
        pub mapSetFunction: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            func: *mut VSFunction,
            append: c_int,
        ) -> c_int,
        /* always consumes the reference, even on error */
        pub mapConsumeFunction: unsafe extern "system" fn(
            map: *mut VSMap,
            key: *const c_char,
            func: *mut VSFunction,
            append: c_int,
        ) -> c_int,

        /* Plugin and plugin function related */
        /* non-zero return value on success  */
        pub registerFunction: unsafe extern "system" fn(
            name: *const c_char,
            args: *const c_char,
            returnType: *const c_char,
            argsFunc: VSPublicFunction,
            functionData: *mut c_void,
            plugin: *mut VSPlugin,
        ) -> c_int,
        pub getPluginByID: unsafe extern "system" fn(
            identifier: *const c_char,
            core: *mut VSCore,
        ) -> *mut VSPlugin,
        pub getPluginByNamespace:
            unsafe extern "system" fn(ns: *const c_char, core: *mut VSCore) -> *mut VSPlugin,
        /* pass NULL to get the first plugin  */
        pub getNextPlugin:
            unsafe extern "system" fn(plugin: *mut VSPlugin, core: *mut VSCore) -> *mut VSPlugin,
        pub getPluginName: unsafe extern "system" fn(plugin: *mut VSPlugin) -> *const c_char,
        pub getPluginID: unsafe extern "system" fn(plugin: *mut VSPlugin) -> *const c_char,
        pub getPluginNamespace: unsafe extern "system" fn(plugin: *mut VSPlugin) -> *const c_char,
        /* pass NULL to get the first plugin function  */
        pub getNextPluginFunction: unsafe extern "system" fn(
            func: *mut VSPluginFunction,
            plugin: *mut VSPlugin,
        ) -> *mut VSPluginFunction,
        pub getPluginFunctionByName: unsafe extern "system" fn(
            name: *const c_char,
            plugin: *mut VSPlugin,
        ) -> *mut VSPluginFunction,
        pub getPluginFunctionName:
            unsafe extern "system" fn(func: *mut VSPluginFunction) -> *const c_char,
        /* returns an argument format string */
        pub getPluginFunctionArguments:
            unsafe extern "system" fn(func: *mut VSPluginFunction) -> *const c_char,
        /* returns an argument format string */
        pub getPluginFunctionReturnType:
            unsafe extern "system" fn(func: *mut VSPluginFunction) -> *const c_char,
        /* the full path to the loaded library file containing the plugin entry point */
        pub getPluginPath: unsafe extern "system" fn(plugin: *const VSPlugin) -> *const c_char,
        pub getPluginVersion: unsafe extern "system" fn(plugin: *const VSPlugin) -> c_int,
        /* user must free the returned VSMap */
        pub invoke: unsafe extern "system" fn(
            plugin: *mut VSPlugin,
            name: *const c_char,
            args: *const VSMap,
        ) -> *mut VSMap,

        /* Core and information */
        /* flags uses the VSCoreCreationFlags enum */
        pub createCore: unsafe extern "system" fn(flags: c_int) -> *mut VSCore,
        /* only call this function after all node, frame and function references belonging to the core have been freed */
        pub freeCore: unsafe extern "system" fn(core: *mut VSCore),
        /* the total cache size at which vapoursynth more aggressively tries to reclaim memory, it is not a hard limit */
        pub setMaxCacheSize: unsafe extern "system" fn(bytes: i64, core: *mut VSCore) -> i64,
        /* setting threads to 0 means automatic detection */
        pub setThreadCount: unsafe extern "system" fn(threads: c_int, core: *mut VSCore) -> c_int,
        pub getCoreInfo: unsafe extern "system" fn(core: *mut VSCore, info: *mut VSCoreInfo),
        pub getAPIVersion: unsafe extern "system" fn() -> c_int,

        /* Message handler */
        pub logMessage:
            unsafe extern "system" fn(msgType: c_int, msg: *const c_char, core: *mut VSCore),
        //     void (VS_CC *logMessage)(int msgType, const char *msg, VSCore *core) VS_NOEXCEPT;
        /* free and userData can be NULL, returns a handle that can be passed to removeLogHandler */
        pub addLogHandler: unsafe extern "system" fn(
            handler: VSLogHandler,
            free: VSLogHandlerFree,
            userData: *mut c_void,
            core: *mut VSCore,
        ) -> *mut VSLogHandle,
        /* returns non-zero if successfully removed */
        pub removeLogHandler:
            unsafe extern "system" fn(handle: *mut VSLogHandle, core: *mut VSCore) -> c_int,

        /* Added in API 4.1, mostly graph and node inspection, PLEASE DON'T USE INSIDE FILTERS */
        /* Additional cache management to free memory */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub clearNodeCache: unsafe extern "system" fn(node: *mut VSNode),
        //     void (VS_CC *clearNodeCache)(VSNode *node) VS_NOEXCEPT; /* clears the cache of the specified node */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub clearCoreCaches: unsafe extern "system" fn(core: *mut VSCore),
        //     void (VS_CC *clearCoreCaches)(VSCore *core) VS_NOEXCEPT; /* clears all caches belonging to the specified core */

        /* Basic node information */
        /* the name passed to create*Filter */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getNodeName: unsafe extern "system" fn(node: *mut VSNode) -> *const c_char,
        /* returns VSFilterMode */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getNodeFilterMode: unsafe extern "system" fn(node: *mut VSNode) -> c_int,
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getNumNodeDependencies: unsafe extern "system" fn(node: *mut VSNode) -> c_int,
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getNodeDependency:
            unsafe extern "system" fn(node: *mut VSNode, index: c_int) -> *const VSFilterDependency,

        /* Node timing functions */
        /* non-zero when filter timing is enabled */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getCoreNodeTiming: unsafe extern "system" fn(core: *mut VSCore) -> c_int,
        /* non-zero enables filter timing, note that disabling simply stops the counters from incrementing */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub setCoreNodeTiming: unsafe extern "system" fn(core: *mut VSCore, enable: c_int),
        /* time spent processing frames in nanoseconds, reset sets the counter to 0 again */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getNodeProcessingTime:
            unsafe extern "system" fn(node: *mut VSNode, reset: c_int) -> i64,
        /* time spent processing frames in nanoseconds in all destroyed nodes, reset sets the counter to 0 again */
        #[cfg(feature = "gte-vapoursynth-api-41")]
        pub getFreedNodeProcessingTime:
            unsafe extern "system" fn(core: *mut VSCore, reset: c_int) -> i64,
    }

    #[cfg(feature = "vsscript-functions")]
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSScript {
        _unused: [u8; 0],
    }

    #[cfg(feature = "vsscript-functions")]
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VSSCRIPTAPI {
        /* Returns the highest supported VSSCRIPT_API_VERSION */
        pub getAPIVersion: unsafe extern "system" fn() -> c_int,
        /* Convenience function for retrieving a VSAPI pointer without having to use the VapourSynth library. Always pass VAPOURSYNTH_API_VERSION */
        pub getVSAPI: unsafe extern "system" fn(version: c_int) -> *const VSAPI,

        /*
         * Providing a pre-created core is useful for setting core creation flags, log callbacks, preload specific plugins and many other things.
         * You must create a VSScript object before evaluating a script. Always takes ownership of the core even on failure. Returns NULL on failure.
         * Pass NULL to have a core automatically created with the default options.
         */
        pub createScript: unsafe extern "system" fn(core: *mut VSCore) -> *mut VSScript,

        /* The core is valid as long as the environment exists, return NULL on error */
        pub getCore: unsafe extern "system" fn(handle: *mut VSScript) -> *mut VSCore,

        /*
         * Evaluates a script passed in the buffer argument. The scriptFilename is only used for display purposes. in Python
         * it means that the main module won't be unnamed in error messages.
         *
         * Returns 0 on success.
         *
         * Note that calling any function other than getError() and freeScript() on a VSScript object in the error state
         * will result in undefined behavior.
         */
        pub evaluateBuffer: unsafe extern "system" fn(
            handle: *mut VSScript,
            buffer: *const c_char,
            scriptFilename: *const c_char,
        ) -> c_int,

        /* Convenience version of the above function that loads the script from scriptFilename and passes as the buffer to evaluateBuffer */
        pub evaluateFile: unsafe extern "system" fn(
            handle: *mut VSScript,
            scriptFilename: *const c_char,
        ) -> c_int,

        /* Returns NULL on success, otherwise an error message */
        pub getError: unsafe extern "system" fn(handle: *mut VSScript) -> *const c_char,

        /* Returns the script's reported exit code */
        pub getExitCode: unsafe extern "system" fn(handle: *mut VSScript) -> c_int,

        /* Fetches a variable of any VSMap storable type set in a script. It is stored in the key with the same name in dst. Returns 0 on success. */
        pub getVariable: unsafe extern "system" fn(
            handle: *mut VSScript,
            name: *const c_char,
            dst: *mut VSMap,
        ) -> c_int,

        /* Sets all keys in the provided VSMap as variables in the script. Returns 0 on success. */
        pub setVariables:
            unsafe extern "system" fn(handle: *mut VSScript, vars: *const VSMap) -> c_int,

        /*
         * The returned nodes must be freed using freeNode() before calling freeScript() since they may depend on data in the VSScript
         * environment. Returns NULL if no node was set as output in the script. Index 0 is used by default in scripts and other
         * values are rarely used.
         */
        pub getOutputNode:
            unsafe extern "system" fn(handle: *mut VSScript, index: c_int) -> *mut VSNode,
        pub getOutputAlphaNode:
            unsafe extern "system" fn(handle: *mut VSScript, index: c_int) -> *mut VSNode,
        pub getAltOutputMode:
            unsafe extern "system" fn(handle: *mut VSScript, index: c_int) -> c_int,
        pub freeScript: unsafe extern "system" fn(handle: *mut VSScript),

        /*
         * Set whether or not the working directory is temporarily changed to the same
         * location as the script file when evaluateFile is called. Off by default.
         */
        pub evalSetWorkingDir: unsafe extern "system" fn(handle: *mut VSScript, setCWD: c_int),

        /*
         * Write a list of set output index values to dst but at most size values.
         * Always returns the total number of available output index values.
         */
        #[cfg(feature = "gte-vsscript-api-42")]
        pub getAvailableOutputNodes:
            unsafe extern "system" fn(handle: *mut VSScript, size: c_int, dst: *mut c_int) -> c_int,
    }

    #[cfg(feature = "vapoursynth-functions")]
    unsafe extern "system" {
        pub fn getVapourSynthAPI(version: c_int) -> *const VSAPI;
    }

    #[cfg(feature = "vsscript-functions")]
    unsafe extern "system" {
        pub fn getVSScriptAPI(version: c_int) -> *const VSSCRIPTAPI;
    }
}
