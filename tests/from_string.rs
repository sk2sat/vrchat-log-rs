use chrono::{Datelike, Timelike};
use vrchat_log::{log, Log, LogEnum};

#[test]
fn from_string() {
    let log = "2021.01.22 21:38:25 Warning    -  OvrLipSync Awake: Queried SampleRate: 48000 BufferSize: 1024

\r
2021.01.22 21:38:25 Log        -  [AssetBundleDownloadManager] Using default cache directory.

\r
2021.01.22 21:38:25 Log        -  AmplitudeAPI: Loaded 0 events, 2 bytes from cache: C:/Users/sksat/AppData/Local/Temp/VRChat/VRChat\\amplitude.cache

\r
2021.01.22 21:38:25 Log        -  VRC Analytics Initialized

\r
2021.01.22 21:38:26 Log        -  OpenVR initialized!


\r
2021.01.22 21:38:26 Exception  -  NotImplementedException: The method or operation is not implemented.
System.IO.FileSystemWatcher..ctor (System.String path) (at <00000000000000000000000000000000>:0)
VRC.SDKInternal.SDKWatcher.Start () (at <00000000000000000000000000000000>:0)


\r
2021.01.22 21:38:26 Log        -  [VRCApplicationSetup] System Info: \r
    Device Model: To Be Filled By O.E.M. (To Be Filled By O.E.M.)\r
    Processor Type: AMD Ryzen 7 3700X 8-Core Processor \r
    System Memory Size: 32698MB\r
    Operating System: Windows 10  (10.0.0) 64bit\r
    Graphics Device Name: Radeon RX Vega\r
    Graphics Device Version: Direct3D 11.0 [level 11.1]\r
    Graphics Memory Size: 8119MB\r
    Supports Audio: True\r
    XR Device: Index

\r
2021.01.22 21:38:26 Log        -  [GC] Memory Usage: Before Allocation\r
- System.GC.GetTotalMemory: 15.81 MB\r
- Profiler.GetMonoUsed/HeapSizeLong: 15.81 MB / 16.66 MB\r
- Profiler.GetTotalAllocated/Reserved/UnusedMemoryLong: 178.14 MB / 238.86 MB / 60.71 MB\r
- SystemInfo.graphicsMemorySize: 8119\r
- SystemInfo.systemMemorySize:32698\r


\r
";

    let log = vrchat_log::from_str(log).unwrap();
    {
        let warn = log[0].as_warning().unwrap();
        let date = warn.0;
        assert_eq!(date.year(), 2021);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 22);
        assert_eq!(date.hour(), 21);
        assert_eq!(date.minute(), 38);
        assert_eq!(date.second(), 25);
        let msg = warn.1;
        assert_eq!(msg.len(), 1);
        assert_eq!(
            msg[0],
            "OvrLipSync Awake: Queried SampleRate: 48000 BufferSize: 1024"
        );
    }
    {
        let log = log[1].as_log().unwrap();
        assert_eq!(log.date.year(), 2021);
        assert_eq!(log.date.month(), 1);
        assert_eq!(log.date.day(), 22);
        assert_eq!(log.date.hour(), 21);
        assert_eq!(log.date.minute(), 38);
        assert_eq!(log.date.second(), 25);
        assert_eq!(log.typ, log::Type::AssetBundleDownloadManager);
        assert_eq!(log.msg.len(), 1);
        assert_eq!(log.msg[0], "Using default cache directory.");
    }
    {
        let log = log[4].as_log().unwrap();
        assert_eq!(log.date.year(), 2021);
        assert_eq!(log.date.month(), 1);
        assert_eq!(log.date.day(), 22);
        assert_eq!(log.date.hour(), 21);
        assert_eq!(log.date.minute(), 38);
        assert_eq!(log.date.second(), 26);
        assert_eq!(log.typ, log::Type::Message);
        assert_eq!(log.msg.len(), 1);
        assert_eq!(log.msg[0], "OpenVR initialized!");
    }
    {
        let log = log[6].as_log().unwrap();
        assert_eq!(log.date.year(), 2021);
        assert_eq!(log.date.month(), 1);
        assert_eq!(log.date.day(), 22);
        assert_eq!(log.date.hour(), 21);
        assert_eq!(log.date.minute(), 38);
        assert_eq!(log.date.second(), 26);
        assert_eq!(log.typ, log::Type::VRCApplicationSetup);
        assert_eq!(log.msg.len(), 10);
    }
}
